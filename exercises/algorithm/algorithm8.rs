/*
	queue
	这个问题要求你使用队列来实现栈的功能
*/

use std::collections::VecDeque;

#[derive(Debug)]
pub struct Queue<T> {
    elements: VecDeque<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: VecDeque::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push_back(value)
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.elements.pop_front()
    }

    pub fn peek(&self) -> Option<&T> {
        self.elements.front()
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
        Queue {
            elements: VecDeque::new(),
        }
    }
}

pub struct MyStack<T> {
    q1: Queue<T>,
    q2: Queue<T>,
}

impl<T> MyStack<T> {
    pub fn new() -> Self {
        Self {
            q1: Queue::new(),
            q2: Queue::new(),
        }
    }
    
    pub fn push(&mut self, elem: T) {
        self.q1.enqueue(elem);
    }
    
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        
        while self.q1.size() > 1 {
            if let Some(item) = self.q1.dequeue() {
                self.q2.enqueue(item);
            }
        }
        
        let result = self.q1.dequeue();
        std::mem::swap(&mut self.q1, &mut self.q2);
        result
    }
    
    pub fn is_empty(&self) -> bool {
        self.q1.is_empty() && self.q2.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_stack() {
        let mut s = MyStack::<i32>::new();
        assert_eq!(s.pop(), None);
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Some(3));
        assert_eq!(s.pop(), Some(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Some(5));
        assert_eq!(s.pop(), Some(4));
        assert_eq!(s.pop(), Some(1));
        assert_eq!(s.pop(), None);
        assert_eq!(s.is_empty(), true);
    }
}