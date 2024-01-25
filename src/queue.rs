pub struct Queue<T> {
    cap: usize,   // 容量
    data: Vec<T>, // 数据容器
}

impl<T> Queue<T> {
    pub fn new(size: usize) -> Self {
        Queue {
            cap: size,
            data: Vec::with_capacity(size),
        }
    }

    pub fn push(&mut self, item: T) -> Result<(), String> {
        if self.size() == self.cap {
            return Err("Queue is full".to_string());
        }

        self.data.insert(0, item);
        Ok(())
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.size() > 0 {
            self.data.pop()
        } else {
            None
        }
    }

    pub fn top(&self) -> Option<&T> {
        self.data.last()
    }

    pub fn is_empty(&self) -> bool {
        self.size() == 0
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }
}

#[cfg(test)]
mod tests {
    use super::Queue;

    #[allow(unused)]
    fn get_queue() -> Queue<i32> {
        let mut queue = Queue::new(5);
        queue.push(1);
        queue.push(2);
        queue.push(3);
        queue
    }

    #[test]
    fn test_push() {
        let mut queue = get_queue();

        assert_eq!(queue.data, vec![3, 2, 1]);

        assert_eq!(queue.push(4), Ok(()));
        assert_eq!(queue.push(5), Ok(()));
        assert_eq!(queue.push(6), Err("Queue is full".to_string()));
    }

    #[test]
    fn test_pop() {
        let mut queue = get_queue();

        assert_eq!(queue.pop(), Some(1));
        assert_eq!(queue.pop(), Some(2));
        assert_eq!(queue.pop(), Some(3));
        assert_eq!(queue.pop(), None);
    }

    #[test]
    fn test_top() {
        let mut queue = get_queue();

        assert_eq!(queue.top(), Some(&1));
        queue.pop();
        assert_eq!(queue.top(), Some(&2));
        queue.pop();
        assert_eq!(queue.top(), Some(&3));
        queue.pop();
        assert_eq!(queue.top(), None);
    }

    #[test]
    fn test_empty_and_size() {
        let mut queue = get_queue();

        assert!(!queue.is_empty());
        assert_eq!(queue.size(), 3);

        queue.pop();
        assert_eq!(queue.size(), 2);
        queue.pop();
        assert_eq!(queue.size(), 1);
        queue.pop();
        assert_eq!(queue.size(), 0);
        assert!(queue.is_empty());
    }
}
