use std::{cell::RefCell, collections::BTreeMap, ffi::c_void, iter::{Zip, zip}, borrow::Borrow};

use libc::malloc;

// for allocated pointers, register the size of the range
thread_local!(static tracked_allocations: RefCell<BTreeMap<*mut c_void, usize>> = RefCell::new(BTreeMap::new()));

pub fn track(ptr: *mut c_void, size: usize) {
    tracked_allocations.with(|tracked| {
        tracked.borrow_mut().insert(ptr, size)
    });
}

pub fn tracked_malloc(size: usize) -> *mut c_void {
    unsafe {
        let ptr = malloc(size);
        track(ptr, size);
        return ptr
    }
}

////TODO not sure about this someone should check
pub fn find_allocation(srcptr: *mut c_void, start: &mut *mut c_void, size: &mut usize) -> bool {
    return tracked_allocations.with(|tracked| {
        let tracked = tracked.borrow();
        let mut this = tracked.iter().peekable();
        while let Some(pair) = this.next() {
            if let Some(next) = this.peek() {
                if next.0 > &srcptr {
                    *start = pair.0.clone();
                    *size = pair.1.clone();
                    return srcptr >= *start && (srcptr as u8) < (*start as u8 + *size as u8);
                }
            } else {
                if pair == tracked.borrow().iter().next().unwrap() {
                    return false
                }
                *start = pair.0.clone();
                *size = pair.1.clone();
                return srcptr >= *start && (srcptr as u8) < (*start as u8 + *size as u8);
            }
        }
        return  false;
    });
}
/* 

bool find_allocation(void *srcptr,  void *&start, size_t &size) {
    auto it = tracked_allocations.upper_bound(srcptr);
    if (it == tracked_allocations.begin())
        return false;
    it--;
    start = it->first;
    size = it->second;
    return (srcptr >= start)&&((uint8_t*)srcptr < ((uint8_t*)start + size));
}*/