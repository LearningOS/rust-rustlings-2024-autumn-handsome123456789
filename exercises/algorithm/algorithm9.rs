/*
	heap
	This question requires you to implement a binary heap function
*/
// I AM NOT DONE

pub struct Heap<T>
where
    T: Ord + Clone,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Ord + Clone,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: Vec::new(),
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        self.items.push(value);
        self.count += 1;
        self.heapify_up(self.count - 1);
    }

    fn parent_idx(&self, idx: usize) -> usize {
        (idx - 1) / 2
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2 + 1
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        idx * 2 + 2
    }

    fn has_left_child(&self, idx: usize) -> bool {
        self.left_child_idx(idx) < self.count
    }

    fn has_right_child(&self, idx: usize) -> bool {
        self.right_child_idx(idx) < self.count
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        let left = self.left_child_idx(idx);
        let right = self.right_child_idx(idx);
        if left < self.count && right < self.count {
            let left_val = &self.items[left];
            let right_val = &self.items[right];
            if (self.comparator)(left_val, right_val) {
                left
            } else {
                right
            }
        } else if left < self.count {
            left
        } else {
            right
        }
    }

    fn heapify_up(&mut self, mut idx: usize) {
        while idx > 0 {
            let parent_idx = self.parent_idx(idx);
            if self.items.get(parent_idx).map_or(false, |p| (self.comparator)(p, &self.items[idx])) {
                self.items.swap(parent_idx, idx);
                idx = parent_idx;
            } else {
                break;
            }
        }
    }

    fn heapify_down(&mut self, mut idx: usize) {
        loop {
            let smallest_child_idx = self.smallest_child_idx(idx);
            if smallest_child_idx == idx {
                break;
            }
            self.items.swap(idx, smallest_child_idx);
            idx = smallest_child_idx;
            if idx >= self.count {
                break;
            }
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            self.items.swap(0, self.count - 1);
            let result = self.items.pop();
            self.count -= 1;
            if !self.is_empty() {
                self.heapify_down(0);
            }
            result
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.items.get(0)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Ord + Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}

pub fn new_min_heap<T: Ord + Clone>() -> Heap<T> {
    Heap::new(|a, b| a < b)
}

pub fn new_max_heap<T: Ord + Clone>() -> Heap<T> {
    Heap::new(|a, b| a > b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_heap() {
        let mut heap = new_max_heap::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = new_min_heap();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = new_max_heap();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}