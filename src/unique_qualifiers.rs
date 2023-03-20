
use crate::qualify_type::{QualifierKind, Qualifier};
use std::cell::RefCell;
use std::collections::{HashSet, HashMap, self, hash_map};
use std::hash::Hasher;
thread_local!(static views: RefCell<HashSet<Box<View_Qualifier>>> = RefCell::new(HashSet::new()));
thread_local!(static uniques: RefCell<HashSet<Box<Unique_Qualifier>>> = RefCell::new(HashSet::new()));


struct Mutate_Qualifier {
    qualifier: Qualifier,

}
impl Mutate_Qualifier {
    pub fn new(kind: QualifierKind) -> Mutate_Qualifier {
        return Mutate_Qualifier{qualifier: Qualifier::new(kind)}
    }
}
struct View_Qualifier {
    qualifier: Qualifier,
    ids: HashSet<isize>,
    sorted_ids: Vec<isize>,
    prehash: u64
}
impl View_Qualifier {
    pub fn new(_ids: HashSet<isize>) -> View_Qualifier {
        let _qualifier = Qualifier::new(QualifierKind::QK_View);
        let mut _sorted_ids = Vec::new();
        for id in _ids.iter() {
            _sorted_ids.push(id.clone());
        }
        _sorted_ids.sort();
        let mut h = hash_map::DefaultHasher::new();
        for entry in _sorted_ids.iter() {
            h.write_isize(entry.clone());
        }
        return View_Qualifier{ qualifier: _qualifier, ids: _ids, sorted_ids: _sorted_ids, prehash: h.finish() }
    }
}
struct Unique_Qualifier {
    qualifier: Qualifier,
    //kind: QualifierKind::QK_Unique,
    id: isize
}

impl Unique_Qualifier {
    pub fn new(_id: isize) -> Unique_Qualifier {
        return Unique_Qualifier { qualifier: Qualifier::new(QualifierKind::QK_Unique), id: _id }
    }
}
pub fn map_unique_id(idmap: &mut HashMap<isize, HashSet<isize>>, fromid: isize, toid: isize) {
    if let Some(set) = idmap.get_mut(&fromid) {
        set.insert(toid);
    }
    let mut set = HashSet::new();
    set.insert(toid);
    idmap.insert(fromid, set);
}
pub fn dump_idmap(idmap: &HashMap<isize, HashSet<isize>>) {
    for (i, s) in idmap {
        println!("{i}: ");
        for j in s {
            print!("{j} ");
        }
    }
    let size = idmap.len();
    println!("{size} entries")
}
pub fn difference_idset(a: &HashSet<isize>, b: &HashSet<isize>) -> HashSet<isize> {
    let mut c: HashSet<isize> = HashSet::new();
    for id in a.difference(b).cloned() {
        c.insert(id);
    }
    return c
}
pub fn intersect_idset(a: &HashSet<isize>, b: &HashSet<isize>) -> HashSet<isize> {
    let mut c: HashSet<isize> = HashSet::new();
    for id in a.intersection(b).cloned() {
        c.insert(id);
    }
    return c
}
pub fn union_idset(a: &HashSet<isize>, b: &HashSet<isize>) -> HashSet<isize> {
    let mut c: HashSet<isize> = HashSet::new();
    for id in a.union(b).cloned() {
        c.insert(id);
    }
    return c
}
pub fn dump_idset(a: &HashSet<isize>) {
    for id in a {
        print!("{id} ")
    }
    println!()
}