use std::ptr;

pub struct List<T> {
    pub head: Link<T>,
    pub count: usize
}

pub type Link<T> = *mut Node<T>;

pub struct Node<T> {
    pub elem: T,
    pub next: Link<T>,
}

pub struct IntoIter<T>(List<T>);

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}
impl<T> List<T> {
    pub fn new() -> Self {
        List { head: ptr::null_mut(), count: 0 }
    }
    pub fn push(&mut self, elem: T) {
        unsafe {
            let new = Box::into_raw(Box::new(Node {
                elem: elem,
                next: ptr::null_mut(),
            }));
            if self.head.is_null() {
                self.head = new;
            } else {
                (*new).next = self.head;
                self.head = new;
            }
            self.count += 1;
        }
    }
    pub fn reverse(&mut self) {
        unsafe {
            let mut prev = std::ptr::null_mut();
            let mut curr = self.head;
            let mut next = std::ptr::null_mut();
            while !curr.is_null() {
                next = (*curr).next;
                (*curr).next = prev;
                prev = curr;
                curr = next;
            }
            self.head = prev;
        }
    }
    pub fn push_list(&mut self, list: Link<T>) {
        if list.is_null() {
            return
        }
        unsafe {
        let mut tail = (*list).next;
        if tail.is_null() {
            (*list).next = self.head;
        } else {
            while !(*tail).next.is_null() {
                tail = (*tail).next;
            }
            (*tail).next = self.head;
        }
        self.head = list;
    }
    }
    pub fn pop(&mut self) -> Option<T> {
        unsafe {
            if self.head.is_null() {
                return None
            } else {
                let head = Box::from_raw(self.head);
                self.head = head.next;
                return Some(head.elem)
            }
            self.count -= 1;
        }
    }

    pub fn peek(&self) -> Option<&T> {
        unsafe {
            self.head.as_ref().map(|node| &node.elem)
        }
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        unsafe {
            self.head.as_mut().map(|node| &mut node.elem)
        }
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<'_, T> {
        unsafe {
            Iter { next: self.head.as_ref() }
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        unsafe {
            IterMut { next: self.head.as_mut() }
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() { }
    }
}
impl<T> Default for List<T> {
    fn default() -> Self {
        List { head: ptr::null_mut(), count: 0 }     
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            self.next.map(|node| {
                self.next = node.next.as_ref();
                &node.elem
            })
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            self.next.take().map(|node| {
                self.next = node.next.as_mut();
                &mut node.elem
            })
        }
    }
}