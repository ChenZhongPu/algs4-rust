//! # Resizing Array Stack
use std::alloc::{self, Layout};
use std::marker::PhantomData;
use std::ptr;
use std::ptr::NonNull;

/// A Resizing Array Stack
///
/// https://doc.rust-lang.org/nomicon/vec/vec.html
pub struct ResizingStack<T> {
    a: NonNull<T>,
    n: usize,
    capacity: usize,
    _marker: PhantomData<T>,
}

unsafe impl<T: Send> Send for ResizingStack<T> {}
unsafe impl<T: Sync> Sync for ResizingStack<T> {}

impl<T> ResizingStack<T> {
    pub fn new(capacity: usize) -> Self {
        let layout = Layout::array::<T>(capacity).unwrap();
        let ptr = unsafe { alloc::alloc(layout) };
        ResizingStack {
            a: NonNull::new(ptr as *mut T).unwrap(),
            n: 0,
            capacity,
            _marker: PhantomData,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    pub fn size(&self) -> usize {
        self.n
    }

    fn resize(&mut self, new_cap: usize) {
        assert!(new_cap >= self.n);

        let new_layout = Layout::array::<T>(new_cap).unwrap();
        let old_layout = Layout::array::<T>(self.capacity).unwrap();
        let old_ptr = self.a.as_ptr() as *mut u8;
        let new_ptr = unsafe { alloc::realloc(old_ptr, old_layout, new_layout.size()) };

        self.a = NonNull::new(new_ptr as *mut T).unwrap();
        self.capacity = new_cap;
    }

    pub fn push(&mut self, t: T) {
        if self.n == self.capacity {
            self.resize(self.capacity * 2);
        }
        unsafe {
            ptr::write(self.a.as_ptr().add(self.n), t);
        }
        self.n += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.n == 0 {
            None
        } else {
            self.n -= 1;
            let item;
            unsafe { item = Some(ptr::read(self.a.as_ptr().add(self.n))) }
            if self.n > 0 && self.n == self.capacity / 4 {
                self.resize(self.capacity / 2);
            }
            item
        }
    }
}

impl<T> Drop for ResizingStack<T> {
    fn drop(&mut self) {
        if self.capacity != 0 {
            while self.pop().is_some() {}
            let layout = Layout::array::<T>(self.capacity).unwrap();
            unsafe {
                alloc::dealloc(self.a.as_ptr() as *mut u8, layout);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_pop() {
        let mut s = ResizingStack::new(10);
        s.push(4);
        s.push(5);
        s.push(6);
        assert_eq!(s.pop(), Some(6));
        assert_eq!(s.pop(), Some(5));
        assert_eq!(s.pop(), Some(4));
        assert_eq!(s.pop(), None);
    }

    #[test]
    fn resize() {
        let mut s = ResizingStack::new(2);
        s.push(4);
        s.push(5);
        s.push(6); // before push -> resize
        assert_eq!(4, s.capacity);
        s.pop();
        s.pop(); // after pop -> resize
        assert_eq!(2, s.capacity);
    }
}
