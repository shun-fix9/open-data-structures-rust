use crate::backend::{Array, Entry};
use crate::CollectionError;
use crate::OUT_OF_BOUND_ERROR;
use crate::{List, Stack};

const DEFAULT_BACKEND_SIZE: usize = 2;
const SIZE_UP_MULTIPLIER_NUMBER: usize = 2;
const SIZE_DOWN_THRESHOLD: usize = 3;
const SIZE_DOWN_DIVISION_NUMBER: usize = 2;

#[derive(Debug)]
pub struct ArrayStack<T> {
    backend: Array<T>,
    size: usize,
}

impl<T> ArrayStack<T> {
    pub fn new() -> ArrayStack<T> {
        ArrayStack {
            backend: Array::new(DEFAULT_BACKEND_SIZE),
            size: 0,
        }
    }

    fn backend_len(&self) -> usize {
        self.backend.len()
    }
}

impl<T> List<T> for ArrayStack<T> {
    fn size(&self) -> usize {
        self.size
    }

    fn get(&self, index: usize) -> Option<&T> {
        if index >= self.size() {
            return None;
        }

        match self.backend.get(index) {
            Some(Entry::Item(item)) => Some(item),
            _ => unreachable!(),
        }
    }

    fn set(&mut self, index: usize, item: T) -> Result<(), CollectionError> {
        if index >= self.size() {
            return Err(OUT_OF_BOUND_ERROR);
        }

        match self.backend.set(index, item) {
            Ok(()) => Ok(()),
            _ => unreachable!(),
        }
    }

    fn add(&mut self, index: usize, item: T) -> Result<(), CollectionError> {
        if index > self.size() {
            return Err(OUT_OF_BOUND_ERROR);
        }

        if self.is_size_up_required() {
            self.size_up();
        }

        self.shift_right_from(index);
        self.size += 1;

        self.set(index, item)
    }

    fn remove(&mut self, index: usize) -> Option<T> {
        if index >= self.size() {
            return None;
        }

        match self.backend.remove(index) {
            Some(Entry::Item(item)) => {
                self.shift_left_to(index);
                self.size -= 1;

                if self.is_size_down_required() {
                    self.size_down();
                }

                Some(item)
            }
            _ => unreachable!(),
        }
    }
}

impl<T> Stack<T> for ArrayStack<T> {
    fn push(&mut self, x: T) {
        match self.add(self.size(), x) {
            Ok(()) => (),
            _ => unreachable!(),
        }
    }

    fn pop(&mut self) -> Option<T> {
        self.remove(self.size() - 1)
    }
}

impl<T> ArrayStack<T> {
    fn is_size_up_required(&self) -> bool {
        self.size() == self.backend_len()
    }

    fn is_size_down_required(&self) -> bool {
        self.size() * SIZE_DOWN_THRESHOLD < self.backend_len()
    }

    fn size_up(&mut self) {
        self.backend.resize(
            self.backend_len() * SIZE_UP_MULTIPLIER_NUMBER,
            0,
            self.size(),
        );
    }

    fn size_down(&mut self) {
        self.backend.resize(
            self.backend_len() / SIZE_DOWN_DIVISION_NUMBER,
            0,
            self.size(),
        );
    }

    fn shift_right_from(&mut self, index: usize) {
        self.backend.shift_right(index, self.size());
    }

    fn shift_left_to(&mut self, index: usize) {
        self.backend.shift_left(index, self.size());
    }
}

#[cfg(test)]
mod tests {
    use super::ArrayStack;
    use crate::List;
    use crate::Stack;

    #[test]
    pub fn stack() {
        let mut stack = ArrayStack::new();
        assert_eq!(stack.size(), 0);

        stack.push(1);
        stack.push(2);
        stack.push(3);
        stack.push(4);
        stack.push(5);

        assert_eq!(stack.size(), 5);

        assert_eq!(stack.get(0), Some(&1));
        assert_eq!(stack.get(1), Some(&2));
        assert_eq!(stack.get(2), Some(&3));
        assert_eq!(stack.get(3), Some(&4));
        assert_eq!(stack.get(4), Some(&5));
        assert_eq!(stack.get(5), None);

        assert_eq!(stack.pop(), Some(5));
        assert_eq!(stack.pop(), Some(4));

        assert_eq!(stack.size(), 3);

        assert_eq!(stack.get(0), Some(&1));
        assert_eq!(stack.get(1), Some(&2));
        assert_eq!(stack.get(2), Some(&3));
        assert_eq!(stack.get(3), None);
        assert_eq!(stack.get(4), None);
        assert_eq!(stack.get(5), None);

        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));

        assert_eq!(stack.size(), 0);
    }
}
