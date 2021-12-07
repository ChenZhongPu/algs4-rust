#[derive(Debug)]
struct FixedCapacityStack<T, const COUNT: usize> {
    a: [T; COUNT],
    n: usize,
}

impl<T: Sized + Default + Copy + Clone, const COUNT: usize> FixedCapacityStack<T, COUNT> {
    pub fn new() -> Self {
        FixedCapacityStack {
            a: [T::default(); COUNT],
            n: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    pub fn size(&self) -> usize {
        self.n
    }

    pub fn push(&mut self, t: T) {
        self.a[self.n] = t;
        self.n += 1;
    }

    pub fn pop(&mut self) -> T {
        self.n -= 1;
        self.a[self.n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tobe() {
        let mut s = FixedCapacityStack::<&str, 100>::new();
        let line = "to be or not to - be - - that - - - is";
        for w in line.split(' ') {
            if w != "-" {
                s.push(w);
            } else if !s.is_empty() {
                s.pop();
            }
        }
        assert_eq!(2, s.size());
    }

    #[test]
    fn push_pop() {
        let mut s = FixedCapacityStack::<i32, 100>::new();
        s.push(4);
        s.push(5);
        s.push(6);
        assert_eq!(s.pop(), 6);
        assert_eq!(s.pop(), 5);
        assert_eq!(s.pop(), 4);
    }
}
