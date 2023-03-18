
use crate::qualify_type::{QualifierKind, Qualifier};
use std::cell::RefCell;
use std::collections::{HashSet, HashMap};
thread_local!(static uniques: RefCell<HashSet<Box<Unique_Qualifier>>> = RefCell::new(HashSet::new()));


struct Mutate_Qualifier {

}
impl Mutate_Qualifier {

}
struct View_Qualifier {

}
impl View_Qualifier {

}
struct Unique_Qualifier {
    qualifier: Qualifier,
    //kind: QualifierKind::QK_Unique,
    id: isize
}

impl Unique_Qualifier {
    pub fn new(id: isize) -> Unique_Qualifier {
        todo!()
    }
}
pub fn map_unique_id(idmap: &HashMap<isize, HashSet<isize>>, fromid: i32, toid: i32) {

}
pub fn dump_idmap(idmap: &HashMap<isize, HashSet<isize>>) {

}
pub fn difference_idset() -> HashSet<isize> {
    todo!()
}
pub fn intersect_idset() -> HashSet<isize> {
    todo!()
}
pub fn union_idset() -> HashSet<isize> {
    todo!()
}
pub fn dump_idset() {

}
/*
void dump_idmap(const ID2SetMap &idmap) {
    StyledStream ss;
    for (auto && entry : idmap) {
        ss << entry.first << ": ";
        for (auto && id : entry.second) {
            ss << id << " ";
        }
        ss << std::endl;
    }
    ss << idmap.size() << " entries" << std::endl;
}

IDSet difference_idset(const IDSet &a, const IDSet &b) {
    IDSet c;
    c.reserve(a.size());
    for (auto id : a) {
        assert(id);
        if (!b.count(id))
            c.insert(id);
    }
    return c;
}

IDSet intersect_idset(const IDSet &a, const IDSet &b) {
    IDSet c;
    c.reserve(std::min(a.size(), b.size()));
    for (auto id : a) {
        assert(id);
        if (b.count(id))
            c.insert(id);
    }
    return c;
}

IDSet union_idset(const IDSet &a, const IDSet &b) {
    IDSet c;
    c.reserve(std::max(a.size(), b.size()));
    for (auto id : a) {
        assert(id);
        c.insert(id);
    }
    for (auto id : b) {
        assert(id);
        c.insert(id);
    }
    return c;
}

void dump_idset(const IDSet &a) {
    StyledStream ss;
    for (auto id : a) {
        ss << id << " ";
    }
    ss << std::endl;
} */