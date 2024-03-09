use std::fmt::Debug;

// 节点
#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

impl<T> Node<T> {
    fn new(elem: T) -> Self {
        Self { elem, next: None }
    }
}

// 链表 Vec
struct LVec<T> {
    size: usize,
    head: Link<T>,
}

impl<T: Copy + Debug> LVec<T> {
    fn new() -> Self {
        Self {
            size: 0,
            head: None,
        }
    }

    fn clear(&mut self) {
        self.size = 0;
        self.head = None;
    }

    fn len(&self) -> usize {
        self.size
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn push(&mut self, elem: T) {
        let node = Node::new(elem);

        if self.is_empty() {
            self.head = Some(Box::new(node));
        } else {
            let mut curr = self.head.as_mut().unwrap();
            // 找到最后一个节点
            for _ in 0..self.size - 1 {
                curr = curr.next.as_mut().unwrap();
            }

            // 最后一个节点后插入新数据
            curr.next = Some(Box::new(node));
        }

        self.size += 1;
    }

    /// 栈末尾加入数据
    fn append(&mut self, other: &mut Self) {
        while let Some(node) = other.head.as_mut().take() {
            self.push(node.elem);
            other.head = node.next.take();
        }
        other.clear()
    }

    fn insert(&mut self, mut index: usize, elem: T) {
        if index >= self.size {
            index = self.size
        }

        // 分三种情况插入
        let mut node = Node::new(elem);
        if self.is_empty() {
            self.head = Some(Box::new(node));
        } else if index == 0 {
            node.next = self.head.take();
            self.head = Some(Box::new(node));
        } else {
            let mut curr = self.head.as_mut().unwrap();
            for _ in 0..index - 1 {
                curr = curr.next.as_mut().unwrap();
            }
            node.next = curr.next.take();
            curr.next = Some(Box::new(node));
        }

        self.size += 1;
    }

    fn pop(&mut self) -> Option<T> {
        self.remove(self.size - 1)
    }

    fn remove(&mut self, index: usize) -> Option<T> {
        if index >= self.size {
            return None;
        }

        let mut node;
        if index == 0 {
            node = self.head.take().unwrap();
            self.head = node.next.take();
        } else {
            let mut curr = self.head.as_mut().unwrap();
            for _ in 0..index - 1 {
                curr = curr.next.as_mut().unwrap();
            }
            node = curr.next.take().unwrap();
            curr.next = node.next.take();
        }

        self.size -= 1;

        Some(node.elem)
    }

    fn print(&self) {
        let mut curr = self.head.as_ref();
        while let Some(node) = curr {
            println!("lvec val: {:#?}", node.elem);
            curr = node.next.as_ref();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LVec;

    #[test]
    fn test_lvec() {
        let mut lvec: LVec<i32> = LVec::new();
        lvec.push(10);
        lvec.push(11);
        lvec.push(12);
        lvec.push(13);
        lvec.insert(0, 9);

        lvec.print();

        let mut lvec2 = LVec::new();
        lvec2.insert(0, 8);
        lvec2.append(&mut lvec);
        lvec2.len();
        lvec2.print();

        let res1 = lvec2.pop();
        let res2 = lvec2.remove(0);
        println!("pop {:#?}", res1.unwrap());
        println!("remove {:#?}", res2.unwrap());
        lvec2.print();
    }
}
