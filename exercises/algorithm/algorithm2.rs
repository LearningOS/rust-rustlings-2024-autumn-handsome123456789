/*
	double linked list reverse
	This problem requires you to reverse a doubly linked list
*/

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            prev: None,
            next: None,
        }
    }
}

#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        node.prev = self.end;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        if index < 0 || index as u32 >= self.length {
            return None;
        }
        self.get_ith_node(self.start, index as u32)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: u32) -> Option<&T> {
        let mut current = node;
        let mut idx = index;
        while let Some(next_ptr) = current {
            if idx == 0 {
                return Some(unsafe { &(*next_ptr.as_ptr()).val });
            }
            current = unsafe { (*next_ptr.as_ptr()).next };
            idx -= 1;
        }
        None
    }

    pub fn reverse(&mut self) {
        let mut current = self.start;
        let mut prev = None;
        let mut next = None;

        while let Some(node) = current {
            next = unsafe { (*node.as_ptr()).next };
            unsafe { (*node.as_ptr()).next = prev };
            prev = Some(node);
            current = next;
        }

        self.start = prev;
        self.end = next;
    }
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut current = self.start;
        let mut result = String::new();
        while let Some(node) = current {
            let node_ref = unsafe { &*node.as_ptr() };
            result.push_str(&format!("{} ", node_ref.val));
            current = node_ref.next;
        }
        write!(f, "{}", result.trim_end())
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if let Some(next) = self.next {
            let next_val = unsafe { &(*next.as_ptr()).val };
            write!(f, "{} <-> {}", self.val, next_val)
        } else {
            write!(f, "{}", self.val)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_reverse_linked_list_1() {
        let mut list = LinkedList::<i32>::new();
        let original_vec = vec![2, 3, 5, 11, 9, 7];
        let reverse_vec = vec![7, 9, 11, 5, 3, 2];
        for i in original_vec {
            list.add(i);
        }
        println!("Linked List is {}", list);
        list.reverse();
        println!("Reversed Linked List is {}", list);
        for (i, &val) in reverse_vec.iter().enumerate() {
            assert_eq!(val, *list.get(i as i32).unwrap());
        }
    }

    #[test]
    fn test_reverse_linked_list_2() {
        let mut list = LinkedList::<i32>::new();
        let original_vec = vec![34, 56, 78, 25, 90, 10, 19, 34, 21, 45];
        let reverse_vec = vec![45, 21, 34, 19, 10, 90, 25, 78, 56, 34];
        for i in original_vec {
            list.add(i);
        }
        println!("Linked List is {}", list);
        list.reverse();
        println!("Reversed Linked List is {}", list);
        for (i, &val) in reverse_vec.iter().enumerate() {
            assert_eq!(val, *list.get(i as i32).unwrap());
        }
    }
}