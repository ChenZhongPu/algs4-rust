//! # Resizing Array Stack
use std::alloc::{self, Layout};
use std::iter::Rev;
use std::marker::PhantomData;
use std::ops::Deref;
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
    pub fn new() -> Self {
        let init_cap = 8;
        let layout = Layout::array::<T>(init_cap).unwrap();
        let ptr = unsafe { alloc::alloc(layout) };
        ResizingStack {
            a: NonNull::new(ptr as *mut T).unwrap(),
            n: 0,
            capacity: init_cap,
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

impl<T> Deref for ResizingStack<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.a.as_ptr(), self.n) }
    }
}

impl<T> ResizingStack<T> {
    pub fn values(&self) -> impl Iterator<Item = &T> {
        self.deref().iter().rev()
    }
}

pub struct StackIter<'a, T> {
    inner: Rev<std::slice::Iter<'a, T>>,
}

impl<'a, T> Iterator for StackIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<T> ResizingStack<T> {
    pub fn iter(&self) -> StackIter<'_, T> {
        StackIter {
            inner: self.deref().iter().rev(),
        }
    }
}

impl<'a, T> IntoIterator for &'a ResizingStack<T> {
    type Item = &'a T;
    type IntoIter = StackIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        StackIter {
            inner: self.deref().iter().rev(),
        }
    }
}

pub struct IntoIter<T>(ResizingStack<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<T> IntoIterator for ResizingStack<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
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

impl<T> Default for ResizingStack<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn push_pop() {
        let mut s = ResizingStack::new();
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
        let mut s = ResizingStack::new();
        assert_eq!(8, s.capacity);
        for i in 0..=8 {
            s.push(i);
        }
        assert_eq!(16, s.capacity);
        for _ in 0..=4 {
            s.pop();
        }
        assert_eq!(8, s.capacity);
    }

    #[test]
    fn values() {
        let mut s = ResizingStack::new();
        s.push(4);
        s.push(5);
        s.push(6);

        let mut iterator = s.values();
        assert_eq!(iterator.next(), Some(&6));
        assert_eq!(iterator.next(), Some(&5));
        assert_eq!(iterator.next(), Some(&4));
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn into_iter() {
        let mut s = ResizingStack::new();
        s.push(4);
        s.push(5);
        s.push(6);

        let mut iterator = s.into_iter();
        assert_eq!(iterator.next(), Some(6));
        assert_eq!(iterator.next(), Some(5));
        assert_eq!(iterator.next(), Some(4));
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn iter() {
        let mut s = ResizingStack::new();
        s.push(4);
        s.push(5);
        s.push(6);

        let mut iterator = s.iter();
        assert_eq!(iterator.next(), Some(&6));
        assert_eq!(iterator.next(), Some(&5));
        assert_eq!(iterator.next(), Some(&4));
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn for_loop() {
        let mut s = ResizingStack::new();
        s.push(4);
        s.push(5);
        s.push(6);
        // &s -> s.iter() -> IntoIterator::into_iter(&v)

        let mut v = vec![];
        for &i in &s {
            v.push(i);
        }
        assert_eq!(v, vec![6, 5, 4]);

        let mut v = vec![];
        for &i in s.iter() {
            v.push(i);
        }
        assert_eq!(v, vec![6, 5, 4]);
    }
}
