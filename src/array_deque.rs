use crate::backend::{Array, Entry};
use crate::CollectionError;
use crate::OUT_OF_BOUND_ERROR;
use crate::{Deque, List};

const DEFAULT_BACKEND_SIZE: usize = 2;
const SIZE_UP_MULTIPLIER_NUMBER: usize = 2;
const SIZE_DOWN_THRESHOLD: usize = 3;
const SIZE_DOWN_DIVISION_NUMBER: usize = 2;

#[derive(Debug)]
pub struct ArrayDeque<T> {
    backend: Array<T>,
    size: usize,
    index: usize,
}

impl<T> ArrayDeque<T> {
    pub fn new() -> ArrayDeque<T> {
        ArrayDeque {
            backend: Array::new(DEFAULT_BACKEND_SIZE),
            size: 0,
            index: 0,
        }
    }

    fn backend_len(&self) -> usize {
        self.backend.len()
    }

    fn backend_index(&self, index: usize) -> usize {
        (index + self.index) % self.backend_len()
    }

    fn is_first_half(&self, index: usize) -> bool {
        index < self.size() / 2
    }
}

impl<T> List<T> for ArrayDeque<T> {
    fn size(&self) -> usize {
        self.size
    }

    fn get(&self, index: usize) -> Option<&T> {
        if index >= self.size() {
            return None;
        }

        match self.backend.get(self.backend_index(index)) {
            Some(Entry::Item(item)) => Some(item),
            _ => unreachable!(),
        }
    }

    fn set(&mut self, index: usize, item: T) -> Result<(), CollectionError> {
        if index >= self.size() {
            return Err(OUT_OF_BOUND_ERROR);
        }

        match self.backend.set(self.backend_index(index), item) {
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

        if self.is_first_half(index) {
            self.shift_left(self.backend_len() - 1, index + self.backend_len() - 1);
            self.index = self.backend_index(self.backend_len() - 1);
        } else {
            self.shift_right(index, self.size());
        }

        self.size += 1;

        self.set(index, item)
    }

    fn remove(&mut self, index: usize) -> Option<T> {
        if index >= self.size() {
            return None;
        }

        match self.backend.remove(self.backend_index(index)) {
            Some(Entry::Item(item)) => {
                if self.is_first_half(index) {
                    self.shift_right(0, index);
                    self.index = self.backend_index(1);
                } else {
                    self.shift_left(index, self.size());
                }

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

impl<T> Deque<T> for ArrayDeque<T> {
    fn addFirst(&mut self, x: T) {
        match self.add(0, x) {
            Ok(()) => (),
            _ => unreachable!(),
        }
    }

    fn addLast(&mut self, x: T) {
        match self.add(self.size(), x) {
            Ok(()) => (),
            _ => unreachable!(),
        }
    }

    fn removeFirst(&mut self) -> Option<T> {
        self.remove(0)
    }

    fn removeLast(&mut self) -> Option<T> {
        self.remove(self.size() - 1)
    }
}

impl<T> ArrayDeque<T> {
    fn is_size_up_required(&self) -> bool {
        self.size() == self.backend_len()
    }

    fn is_size_down_required(&self) -> bool {
        self.size() * SIZE_DOWN_THRESHOLD < self.backend_len()
    }

    fn size_up(&mut self) {
        self.backend.resize(
            self.backend_len() * SIZE_UP_MULTIPLIER_NUMBER,
            self.index,
            self.size(),
        );
        self.index = 0;
    }

    fn size_down(&mut self) {
        self.backend.resize(
            self.backend_len() / SIZE_DOWN_DIVISION_NUMBER,
            self.index,
            self.size(),
        );
        self.index = 0;
    }

    fn shift_right(&mut self, from: usize, to: usize) {
        self.backend
            .shift_right(self.backend_index(from), self.backend_index(to));
    }

    fn shift_left(&mut self, from: usize, to: usize) {
        self.backend
            .shift_left(self.backend_index(from), self.backend_index(to));
    }
}

#[cfg(test)]
mod tests {
    use super::ArrayDeque;
    use crate::Deque;
    use crate::List;

    #[test]
    pub fn deque() {
        let mut deque = ArrayDeque::new();
        assert_eq!(deque.size(), 0);

        deque.addLast(1);
        deque.addLast(2);
        deque.addLast(3);
        deque.addFirst(4);
        deque.addFirst(5);

        assert_eq!(deque.size(), 5);

        assert_eq!(deque.get(0), Some(&5));
        assert_eq!(deque.get(1), Some(&4));
        assert_eq!(deque.get(2), Some(&1));
        assert_eq!(deque.get(3), Some(&2));
        assert_eq!(deque.get(4), Some(&3));
        assert_eq!(deque.get(5), None);

        assert_eq!(deque.removeLast(), Some(3));
        assert_eq!(deque.removeLast(), Some(2));

        assert_eq!(deque.size(), 3);

        assert_eq!(deque.get(0), Some(&5));
        assert_eq!(deque.get(1), Some(&4));
        assert_eq!(deque.get(2), Some(&1));
        assert_eq!(deque.get(3), None);
        assert_eq!(deque.get(4), None);
        assert_eq!(deque.get(5), None);

        assert_eq!(deque.removeFirst(), Some(5));
        assert_eq!(deque.removeFirst(), Some(4));
        assert_eq!(deque.removeFirst(), Some(1));

        assert_eq!(deque.size(), 0);
    }
}
