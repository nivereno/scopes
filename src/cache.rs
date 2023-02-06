use std::{io::prelude::*, ops::Add};
use core::cell::RefCell;
use std::{cell::Cell, cmp::Ordering, fs::{self, DirEntry, remove_file, File}, ops::AddAssign, collections::hash_map::DefaultHasher, hash::Hasher, fmt::format};

use flate2::write::ZlibEncoder;

use crate::config::{SCOPES_MAX_CACHE_SIZE, SCOPES_MAX_CACHE_INODES};

const  SCOPES_CACHE_WRITE_KEY: usize = 0;
const  SCOPES_FILE_CACHE_EXT: &'static str = ".cache";
const  SCOPES_FILE_CACHE_KEY_PATTERN: &'static str = "{}/{}.cache.key";
const  SCOPES_FILE_CACHE_PATTERN: &'static str = "{}/{}.cache";


thread_local!(static cache_misses: Cell<usize> = Cell::new(0));
thread_local!(static cache_inited: Cell<bool> = Cell::new(false));
thread_local!(static cache_dir: RefCell<String> = RefCell::new(String::new()));

fn get_cache_misses() -> usize {
    cache_misses.with(|misses| {
        return misses.replace(0);
    })
}
// delete half of all cache files to make space, and/or half of all inodes
// to stay within filesystem limits.
//...........I did not name this.
pub fn perform_thanos_finger_snap(num_files: usize) -> Result<(), std::io::Error>{
    //let extsize = SCOPES_FILE_CACHE_EXT.len();
    /*let mut cachefile = cache_dir.with(|dir| {
        return dir.borrow().clone()
    });
    cachefile.push('/');*/

    /*struct CacheEntry {
        path: String,
        atime: isize,
        size: isize,
    }
    impl PartialEq for CacheEntry {
        fn eq(&self, other: &Self) -> bool {
            return self.atime == other.atime
        }
    }
    impl PartialOrd for CacheEntry {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            return self.atime.partial_cmp(&other.atime)
        }
    }*/

    //let mut cache_entries = Vec::new();//: Vec<CacheEntry> = Vec::with_capacity(1024);
    let Ok(cache_dir_iter) = fs::read_dir(cache_dir.with(|dir| {
        return dir.borrow().clone();
    })) else {return Ok(())};
    //cache_entries = cache_dir_iter.collect();
    let mut to_delete = num_files / 2;
    //TODO Maybe add logic to sort the entries first, or make sure they are already sorted
    for entry in cache_dir_iter {
        if to_delete == 0 {
            return Ok(())
        }
        match entry {
            Ok(ent) => todo!(),//remove_file(ent.path())?, TODO deleting files is scary
            Err(ent) => return Err(ent)
        }
        to_delete -= 1;
    }
    return Ok(())
}
// count cumulative size of cache files and clean up if too big
pub fn check_cache_size() -> Result<(), std::io::Error> {
    let mut cache_size: u64 = 0;
    let mut num_files: usize = 0;

    let Ok(cache_dir_iter) = fs::read_dir(cache_dir.with(|dir| {
        return dir.borrow().clone()
    })) else {return Ok(())};
    for entry in cache_dir_iter {
        match entry {
            Ok(ent) => {
                cache_size += ent.metadata()?.len();
                num_files += 1;
            }
            Err(ent) => return Err(ent)
        }
    }
    if cache_size >= SCOPES_MAX_CACHE_SIZE as u64 || num_files >= SCOPES_MAX_CACHE_INODES {
        perform_thanos_finger_snap(num_files);
    }
    return Ok(())
}
pub fn init_cache() {
    if cache_inited.with(|inited| {
        if inited.get().eq(&true) {
            return true
        }
        inited.replace(true);
        return false
    }){return}


    todo!();



    check_cache_size();
}
fn get_cache_dir() -> String {
    init_cache();
    cache_dir.with(|dir| {
        return dir.borrow().clone();
    })
}
fn get_cache_key(hash: u64, content: &[u8]) -> String {
    let mut h: [u64; 4] = [0, 0, 0, 0];
    let mut hasher = DefaultHasher::new(); //TODO maybe use a different hasher, maybe use the hasher differently
    if content.len() < 4 {
        hasher.write_u64(hash);
        h[0] = hasher.finish();
    } else {
        let part = content.len() / 4;
        hasher.write(&content[..part]);
        h[0] = hasher.finish();
        hasher.write(&content[part.. part + part]);
        h[1] = hasher.finish();
        hasher.write(&content[part*2..part*2 + part]);
        h[2] = hasher.finish();
        hasher.write(&content[part*3..part*3 + part]);
        h[3] = hasher.finish();
    }

    let mut key = String::with_capacity(64);
    for i in 0..=3 {
        key.push_str(&(h[i].to_string())); 
        key.push_str("016");
        key.push_str("llx"); //TODO PRIx64
    }
    return key
}
fn get_cache_file(key: &str) -> Option<String> {
    init_cache();
    return cache_dir.with(|dir| {
        let filepath = format!("{}/{}.cache", dir.borrow(), key);
            if let Ok(meta) = fs::metadata(&filepath) {
                if meta.is_file() {
                    return Some(filepath);
                }
            }
            
            cache_misses.with(|misses| {
                misses.set(misses.get().add(1));
            });
            return None;
    });
}

fn set_cache(key: &str, key_content: &str, content: &[u8]) -> Result<(), anyhow::Error> {
    return cache_dir.with(|dir| {
        let filepath = format!("{}/{}.cache", dir.borrow(), key);
        let file = File::open(filepath)?;
        let mut writer = flate2::write::ZlibEncoder::new(file, flate2::Compression::default());
        writer.write_all(content)?;
        return Ok(())
    });
}
