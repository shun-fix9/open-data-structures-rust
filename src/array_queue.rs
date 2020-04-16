use crate::backend::{Array, Entry};
use crate::CollectionError;
use crate::OUT_OF_BOUND_ERROR;
use crate::{List, Queue};

const DEFAULT_BACKEND_SIZE: usize = 2;
const SIZE_UP_MULTIPLIER_NUMBER: usize = 2;
const SIZE_DOWN_THRESHOLD: usize = 3;
const SIZE_DOWN_DIVISION_NUMBER: usize = 2;

#[derive(Debug)]
pub struct ArrayQueue<T> {
    backend: Array<T>,
    size: usize,
    index: usize,
}

impl<T> ArrayQueue<T> {
    pub fn new() -> ArrayQueue<T> {
        ArrayQueue {
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

    fn increment_index(&mut self) {
        self.index = self.backend_index(1);
    }
}

impl<T> List<T> for ArrayQueue<T> {
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

        self.shift_right(index, self.size());
        self.size += 1;

        self.set(index, item)
    }

    fn remove(&mut self, index: usize) -> Option<T> {
        if index >= self.size() {
            return None;
        }

        match self.backend.remove(self.backend_index(index)) {
            Some(Entry::Item(item)) => {
                self.shift_right(0, index);
                self.increment_index();
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

impl<T> Queue<T> for ArrayQueue<T> {
    fn enqueue(&mut self, x: T) {
        match self.add(self.size(), x) {
            Ok(()) => (),
            _ => unreachable!(),
        }
    }

    fn dequeue(&mut self) -> Option<T> {
        self.remove(0)
    }
}

impl<T> ArrayQueue<T> {
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
}

#[cfg(test)]
mod tests {
    use super::ArrayQueue;
    use crate::List;
    use crate::Queue;

    #[test]
    pub fn queue() {
        let mut queue = ArrayQueue::new();
        assert_eq!(queue.size(), 0);

        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);
        queue.enqueue(4);
        queue.enqueue(5);

        assert_eq!(queue.size(), 5);

        assert_eq!(queue.get(0), Some(&1));
        assert_eq!(queue.get(1), Some(&2));
        assert_eq!(queue.get(2), Some(&3));
        assert_eq!(queue.get(3), Some(&4));
        assert_eq!(queue.get(4), Some(&5));
        assert_eq!(queue.get(5), None);

        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.dequeue(), Some(2));

        assert_eq!(queue.size(), 3);

        assert_eq!(queue.get(0), Some(&3));
        assert_eq!(queue.get(1), Some(&4));
        assert_eq!(queue.get(2), Some(&5));
        assert_eq!(queue.get(3), None);
        assert_eq!(queue.get(4), None);
        assert_eq!(queue.get(5), None);

        assert_eq!(queue.dequeue(), Some(3));
        assert_eq!(queue.dequeue(), Some(4));
        assert_eq!(queue.dequeue(), Some(5));

        assert_eq!(queue.size(), 0);
    }
}
