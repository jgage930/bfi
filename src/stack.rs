pub struct Stack<T> {
    values: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }

    pub fn push(&mut self, t: T) {
        self.values.insert(0, t);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.values.len() == 0 {
            return None;
        }

        Some(self.values.remove(0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testing_stack() -> Stack<usize> {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        stack.push(4);

        stack
    }

    #[test]
    fn test_push() {
        assert_eq!(testing_stack().values, vec![4, 3, 2, 1]);
    }

    #[test]
    fn test_pop() {
        assert_eq!(testing_stack().pop(), Some(4));
    }
}
