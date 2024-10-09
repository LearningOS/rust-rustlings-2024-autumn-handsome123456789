/*
	stack
	This question requires you to use a stack to achieve a bracket match
*/

// I AM NOT DONE
#[derive(Debug)]
struct Stack<T> {
    data: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Self { data: Vec::new() }
    }
    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    fn len(&self) -> usize {
        self.data.len()
    }
    fn clear(&mut self) {
        self.data.clear();
    }
    fn push(&mut self, val: T) {
        self.data.push(val);
    }
    fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }
    fn peek(&self) -> Option<&T> {
        self.data.last()
    }
    fn peek_mut(&mut self) -> Option<&mut T> {
        self.data.last_mut()
    }
    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
    fn iter(&self) -> Iter<T> {
        Iter {
            iter: self.data.iter(),
        }
    }
    fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            iter: self.data.iter_mut(),
        }
    }
}

struct IntoIter<T>(Stack<T>);
impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

struct Iter<'a, T: 'a> {
    iter: std::slice::Iter<'a, T>,
}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

struct IterMut<'a, T: 'a> {
    iter: std::slice::IterMut<'a, T>,
}
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

fn bracket_match(bracket: &str) -> bool {
    let mut stack = Stack::<char>::new();
    for c in bracket.chars() {
        match c {
            '(' | '[' | '{' => stack.push(c),
            ')' => {
                if stack.pop() != Some('(') {
                    return false;
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return false;
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return false;
                }
            }
            _ => {}
        }
    }
    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bracket_matching_1() {
        let s = "(2+3){func}[abc]";
        assert_eq!(bracket_match(s), true);
    }
    #[test]
    fn bracket_matching_2() {
        let s = "(2+3)*(3-1";
        assert_eq!(bracket_match(s), false);
    }
    #[test]
    fn bracket_matching_3() {
        let s = "{{([])})}";
        assert_eq!(bracket_match(s), false);
    }
    #[test]
    fn bracket_matching_4() {
        let s = "{{(}[)]}";
        assert_eq!(bracket_match(s), false);
    }
    #[test]
    fn bracket_matching_5() {
        let s = "[[[]]]]]]]]]]";
        assert_eq!(bracket_match(s), false);
    }
    #[test]
    fn bracket_matching_6() {
        let s = "";
        assert_eq!(bracket_match(s), true);
    }
}