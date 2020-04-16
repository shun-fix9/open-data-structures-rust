use crate::backend::Array;
use crate::backend::Entry;
use crate::CollectionError;
use crate::List;
use crate::Stack;
use crate::OUT_OF_BOUND_ERROR;

const DEFAULT_BACKEND_SIZE: usize = 2;
const SIZE_UP_MULTIPLIER_FACTOR: usize = 2;
const SIZE_DOWN_THRESHOLD: usize = 3;
const SIZE_DOWN_DIVISION_FACTOR: usize = 2;

#[derive(Debug)]
pub struct ArrayStack<T> {
    array: Array<T>,
    size: usize,
}

impl<T> ArrayStack<T> {
    pub fn new() -> ArrayStack<T> {
        ArrayStack {
            array: Array::new(DEFAULT_BACKEND_SIZE),
            size: 0,
        }
    }

    #[allow(dead_code)]
    fn backend_len(&self) -> usize {
        self.array.len()
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

        match self.array.get(index) {
            Some(Entry::Item(item)) => Some(item),
            _ => unreachable!(),
        }
    }

    fn set(&mut self, index: usize, item: T) -> Result<(), CollectionError> {
        if index >= self.size() {
            return Err(OUT_OF_BOUND_ERROR);
        }

        match self.array.set(index, item) {
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

        self.set(index, item)
    }

    fn remove(&mut self, index: usize) -> Option<T> {
        if index >= self.size() {
            return None;
        }

        match self.array.remove(index) {
            Some(Entry::Item(item)) => {
                self.shift_left_to(index);

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
        self.size() == self.array.len()
    }

    fn is_size_down_required(&self) -> bool {
        self.size() * SIZE_DOWN_THRESHOLD < self.array.len()
    }

    fn size_up(&mut self) {
        self.array
            .resize(self.size(), self.array.len() * SIZE_UP_MULTIPLIER_FACTOR);
    }

    fn size_down(&mut self) {
        self.array
            .resize(self.size(), self.array.len() / SIZE_DOWN_DIVISION_FACTOR);
    }

    fn shift_right_from(&mut self, index: usize) {
        self.array.shift_right(index, self.size());
        self.size += 1;
    }

    fn shift_left_to(&mut self, index: usize) {
        self.array.shift_left(index, self.size());
        self.size -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::ArrayStack;
    use crate::List;
    use crate::Stack;

    #[test]
    pub fn stack() {
        let mut array = ArrayStack::new();
        assert_eq!(array.size(), 0);
        assert_eq!(array.backend_len(), 2);

        array.push(1);
        array.push(2);
        array.push(3);
        array.push(4);
        array.push(5);

        assert_eq!(array.size(), 5);
        assert_eq!(array.backend_len(), 8);

        check(
            &array,
            vec![
                (0, Some(&1)),
                (1, Some(&2)),
                (2, Some(&3)),
                (3, Some(&4)),
                (4, Some(&5)),
                (5, None),
            ],
        );

        assert_eq!(array.pop(), Some(5));
        assert_eq!(array.pop(), Some(4));

        assert_eq!(array.size(), 3);

        check(
            &array,
            vec![
                (0, Some(&1)),
                (1, Some(&2)),
                (2, Some(&3)),
                (3, None),
                (4, None),
                (5, None),
            ],
        );

        assert_eq!(array.pop(), Some(3));
        assert_eq!(array.backend_len(), 4);

        assert_eq!(array.pop(), Some(2));
        assert_eq!(array.pop(), Some(1));

        assert_eq!(array.size(), 0);
        assert_eq!(array.backend_len(), 1);
    }

    fn check<T>(array: &ArrayStack<T>, items: Vec<(usize, Option<&T>)>)
    where
        T: std::fmt::Debug + std::cmp::PartialEq,
    {
        for (i, entry) in items.iter() {
            assert_eq!(array.get(*i), *entry);
        }
    }
}
