/*
	heap
	This question requires you to implement a binary heap function
*/
// I AM NOT DONE

use std::cmp::Ordering;

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
        self.sift_up(self.count - 1);
    }

    fn parent_idx(&self, idx: usize) -> usize {
        (idx - 1) / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) < self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2 + 1
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        idx * 2 + 2
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
        } else if right < self.count {
            right
        } else {
            idx // No children, return the current index
        }
    }

    fn sift_up(&mut self, mut idx: usize) {
        while idx > 0 {
            let parent_idx = self.parent_idx(idx);
            if (self.comparator)(&self.items[parent_idx], &self.items[idx]) {
                self.items.swap(parent_idx, idx);
                idx = parent_idx;
            } else {
                break;
            }
        }
    }

    fn sift_down(&mut self, mut idx: usize) {
        loop {
            let smallest_child_idx = self.smallest_child_idx(idx);
            if smallest_child_idx == idx {
                break;
            }
            if smallest_child_idx >= self.count {
                break; // No children, break the loop
            }
            self.items.swap(smallest_child_idx, idx);
            idx = smallest_child_idx;
        }
    }
}

impl<T> Iterator for Heap<T>
where
    T: Ord + Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count == 0 {
            None
        } else {
            let result = self.items[0].clone(); // Clone the root element
            self.items[0] = self.items[self.count - 1].clone(); // Move the last element to root
            self.items.pop(); // Remove the last element
            self.count -= 1;
            if self.count > 0 {
                self.sift_down(0);
            }
            Some(result)
        }
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T: Ord + Clone>() -> Heap<T> {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T: Ord + Clone>() -> Heap<T> {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
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
        let mut heap = MaxHeap::new();
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