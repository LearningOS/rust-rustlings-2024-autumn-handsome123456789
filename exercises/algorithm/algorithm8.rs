#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue { elements: Vec::new() }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if !self.elements.is_empty() {
            Some(self.elements.remove(0))
        } else {
            None
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.elements.first()
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue::new()
    }
}

pub struct MyStack<T> {
    q1: Queue<T>,
}

impl<T> MyStack<T> {
    pub fn new() -> Self {
        Self { q1: Queue::new() }
    }

    pub fn push(&mut self, elem: T) {
        let mut temp = Queue::new();
        while let Some(item) = self.q1.dequeue() {
            temp.enqueue(item);
        }
        self.q1.enqueue(elem);
        while let Some(item) = temp.dequeue() {
            self.q1.enqueue(item);
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        self.q1.dequeue()
    }

    pub fn is_empty(&self) -> bool {
        self.q1.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack() {
        let mut s = MyStack::<i32>::new();
        assert!(s.pop().is_none());
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Some(3));
        assert_eq!(s.pop(), Some(2));
        s.push(4);
        s.push(5);
        assert!(!s.is_empty());
        assert_eq!(s.pop(), Some(5));
        assert_eq!(s.pop(), Some(4));
        assert_eq!(s.pop(), Some(1));
        assert!(s.pop().is_none());
        assert!(s.is_empty());
    }
}