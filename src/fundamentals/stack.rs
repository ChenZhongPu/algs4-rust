//! # Stack
//!
//! Stack implementation based on `Vec`.

#[derive(Debug, Default)]
pub struct Stack<T> {
    data: Vec<T>,
}

impl<T> Stack<T> {
    pub fn push(&mut self, t: T) {
        self.data.push(t);
    }

    pub fn pop(&mut self) -> T {
        self.data.pop().expect("cannot pop from empty")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_pop() {
        let mut stack = Stack::default();
        stack.push(4);
        stack.push(5);
        stack.push(6);
        assert_eq!(stack.pop(), 6);
        assert_eq!(stack.pop(), 5);
        assert_eq!(stack.pop(), 4);
    }

    #[test]
    #[should_panic]
    fn pop_empty() {
        let mut stack = Stack::default();
        stack.push(1);
        stack.pop();
        stack.pop();
    }
}
