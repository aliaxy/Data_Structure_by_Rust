pub struct Deque<T> {
    cap: usize,
    data: Vec<T>,
}

impl<T> Deque<T> {
    pub fn new(size: usize) -> Self {
        Deque {
            cap: size,
            data: Vec::with_capacity(size),
        }
    }

    pub fn push_front(&mut self, item: T) -> Result<(), String> {
        if self.size() == self.cap {
            return Err("Queue is full".to_string());
        }
        self.data.insert(0, item);

        Ok(())
    }

    pub fn push_back(&mut self, item: T) -> Result<(), String> {
        if self.size() == self.cap {
            return Err("Queue is full".to_string());
        }
        self.data.push(item);

        Ok(())
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.data.is_empty() {
            None
        } else {
            Some(self.data.remove(0))
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.data.is_empty() {
            None
        } else {
            self.data.pop()
        }
    }

    pub fn front(&self) -> Option<&T> {
        self.data.first()
    }

    pub fn back(&self) -> Option<&T> {
        self.data.last()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }
}

#[cfg(test)]
mod tests {
    use super::Deque;

    #[allow(unused)]
    fn get_deque() -> Deque<i32> {
        let mut deque = Deque::new(5);
        deque.push_back(1);
        deque.push_back(2);
        deque.push_back(3);
        deque
    }

    #[test]
    #[allow(unused)]
    fn test_push() {
        let mut deque = get_deque();

        assert_eq!(deque.data, vec![1, 2, 3]);

        deque.push_front(4);
        assert_eq!(deque.data, vec![4, 1, 2, 3]);
        deque.push_front(5);
        assert_eq!(deque.data, vec![5, 4, 1, 2, 3]);
        assert_eq!(deque.push_front(6), Err("Queue is full".to_string()));
    }

    #[test]
    fn test_pop() {
        let mut deque = get_deque();

        assert_eq!(deque.pop_front(), Some(1));
        assert_eq!(deque.pop_back(), Some(3));
        assert_eq!(deque.pop_back(), Some(2));
        assert_eq!(deque.pop_front(), None);
        assert_eq!(deque.pop_back(), None);
    }

    #[test]
    fn test_front_and_back() {
        let mut deque = get_deque();

        assert_eq!(deque.front(), Some(&1));
        assert_eq!(deque.back(), Some(&3));

        deque.pop_front();
        assert_eq!(deque.front(), Some(&2));
        assert_eq!(deque.back(), Some(&3));

        deque.pop_back();
        assert_eq!(deque.front(), Some(&2));
        assert_eq!(deque.back(), Some(&2));

        deque.pop_back();
        assert_eq!(deque.front(), None);
        assert_eq!(deque.back(), None);
    }

    #[test]
    fn test_empty_and_size() {
        let mut deque = get_deque();

        assert!(!deque.is_empty());
        assert_eq!(deque.size(), 3);

        deque.pop_front();
        assert_eq!(deque.size(), 2);
        deque.pop_front();
        assert_eq!(deque.size(), 1);
        deque.pop_front();
        assert_eq!(deque.size(), 0);
        assert!(deque.is_empty());
    }
}
