use core::cell::RefCell;
use std::{cell::Ref, cmp::Ordering, fs::{self, DirEntry, remove_file}};

const  SCOPES_CACHE_WRITE_KEY: usize = 0;
const  SCOPES_FILE_CACHE_EXT: &'static str = ".cache";
const  SCOPES_FILE_CACHE_KEY_PATTERN: &'static str = "%s/%s.cache.key";
const  SCOPES_FILE_CACHE_PATTERN: &'static str = "%s/%s.cache";


thread_local!(static cache_misses: RefCell<usize> = RefCell::new(0));
thread_local!(static cache_inited: RefCell<bool> = RefCell::new(false));
thread_local!(static cache_dir: RefCell<String> = RefCell::new(String::new()));

fn get_cache_misses() -> usize {
    cache_misses.with(|misses| {
        return misses.replace(0);
    })
}
// delete half of all cache files to make space, and/or half of all inodes
// to stay within filesystem limits.
//...........I did not name this.
pub fn perform_thanos_finger_snap() -> Result<(), std::io::Error>{
    let extsize = SCOPES_FILE_CACHE_EXT.len();
    let mut cachefile = cache_dir.with(|dir| {
        return dir.borrow().clone()
    });
    cachefile.push('/');

    struct CacheEntry {
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
    }

    let mut cache_entries = Vec::new();//: Vec<CacheEntry> = Vec::with_capacity(1024);
    let Ok(cache_dir_iter) = fs::read_dir(cache_dir.with(|dir| {
        return dir.borrow().clone()
    })) else {return Ok(())};
    cache_entries = cache_dir_iter.collect();
    let mut to_delete = cache_entries.len() / 2;
    //TODO Maybe add logic to sort the entries first, or make sure they are already sorted
    for entry in cache_entries {
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
pub fn check_cahce_size() {

}
pub fn init_cache() {
    
}
fn get_cache_dir() -> String {
    init_cache();
    cache_dir.with(|dir| {
        return dir.borrow().clone();
    })
}
fn get_cache_key() {

}
fn get_cache_file() {

}
