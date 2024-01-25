pub struct Stack<T> {
    top: usize,   // 栈顶
    data: Vec<T>, // 栈数据容器
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self {
            top: 0,
            data: Vec::new(),
        }
    }

    pub fn push(&mut self, item: T) {
        self.data.push(item);
        self.top += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.top == 0 {
            return None;
        }
        self.top -= 1;
        self.data.pop()
    }

    pub fn top(&self) -> Option<&T> {
        if self.top == 0 {
            None
        } else {
            self.data.get(self.top - 1)
        }
    }

    pub fn is_empty(&self) -> bool {
        self.top == 0
    }

    pub fn size(&self) -> usize {
        self.top
    }
}

#[cfg(test)]
mod tests {
    use super::Stack;

    fn get_stack() -> Stack<i32> {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        stack
    }

    #[test]
    fn test_push() {
        let stack = get_stack();

        assert_eq!(stack.data, vec![1, 2, 3]);
    }

    #[test]
    fn test_pop() {
        let mut stack = get_stack();

        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_top() {
        let mut stack = get_stack();

        assert_eq!(stack.top(), Some(&3));
        stack.pop();
        assert_eq!(stack.top(), Some(&2));
        stack.pop();
        assert_eq!(stack.top(), Some(&1));
        stack.pop();
        assert_eq!(stack.top(), None);
    }

    #[test]
    fn test_empty_and_size() {
        let mut stack = get_stack();

        assert!(!stack.is_empty());
        assert_eq!(stack.size(), 3);

        stack.pop();
        assert_eq!(stack.size(), 2);
        stack.pop();
        assert_eq!(stack.size(), 1);
        stack.pop();
        assert_eq!(stack.size(), 0);
        assert!(stack.is_empty());
    }
}
