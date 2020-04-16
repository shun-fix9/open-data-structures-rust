use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub(in crate) struct Array<T> {
    items: Vec<Entry<T>>,
    length: usize,
}

impl<T> Array<T> {
    pub(in crate) fn new(length: usize) -> Array<T> {
        Array {
            items: Self::new_items(length),
            length,
        }
    }

    fn new_items(length: usize) -> Vec<Entry<T>> {
        let mut items = vec![];
        let mut count = 0;
        while count < length {
            items.push(Entry::default());
            count += 1;
        }

        items
    }

    pub(in crate) const fn len(&self) -> usize {
        self.length
    }

    pub(in crate) fn get(&self, index: usize) -> Option<&Entry<T>> {
        if let Some(entry) = self.items.get(index) {
            Some(&entry)
        } else {
            None
        }
    }

    pub(in crate) fn remove(&mut self, index: usize) -> Option<Entry<T>> {
        if index < self.length {
            let mut entry = Entry::default();
            std::mem::swap(&mut entry, &mut self.items[index]);
            Some(entry)
        } else {
            None
        }
    }

    pub(in crate) fn set(&mut self, index: usize, item: T) -> Result<(), BackendError> {
        if index < self.length {
            self.items[index] = Entry::Item(item);
            Ok(())
        } else {
            Err(OUT_OF_BOUND_ERROR)
        }
    }

    pub(in crate) fn shift_right(&mut self, from: usize, to: usize) {
        let mut target_index = to;
        while target_index > from {
            let source = (target_index - 1) % self.items.len();
            let destination = target_index % self.items.len();

            self.items.swap(source, destination);
            target_index -= 1;
        }
    }

    pub(in crate) fn shift_left(&mut self, from: usize, to: usize) {
        let mut target_index = from;
        while target_index < to {
            let source = (target_index + 1) % self.items.len();
            let destination = target_index % self.items.len();

            self.items.swap(destination, source);
            target_index += 1;
        }
    }

    pub(in crate) fn resize(&mut self, new_size: usize, start_at: usize, copy_length: usize) {
        let mut new_items = Self::new_items(new_size);

        let mut count = 0;
        while count < copy_length {
            let target_index = (start_at + count) % self.items.len();

            std::mem::swap(&mut self.items[target_index], &mut new_items[count]);
            count += 1;
        }

        self.items = new_items;
        self.length = new_size;
    }
}

#[derive(Debug, PartialEq)]
pub(in crate) enum Entry<T> {
    Empty,
    Item(T),
}

impl<T> Default for Entry<T> {
    fn default() -> Self {
        Self::Empty
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(in crate) struct BackendError {
    message: &'static str,
}

impl fmt::Display for BackendError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "error: {}", self.message)
    }
}

impl Error for BackendError {}

pub(in crate) const OUT_OF_BOUND_ERROR: BackendError = BackendError {
    message: "out of bound",
};

#[cfg(test)]
mod tests {
    use super::Array;
    use super::Entry;

    #[test]
    pub fn new_array() {
        let length = 5;
        let array: Array<i32> = Array::new(length);

        assert_eq!(array.len(), length);

        assert_eq!(array.get(0), Some(&Entry::Empty));
        assert_eq!(array.get(1), Some(&Entry::Empty));
        assert_eq!(array.get(2), Some(&Entry::Empty));
        assert_eq!(array.get(3), Some(&Entry::Empty));
        assert_eq!(array.get(4), Some(&Entry::Empty));
        assert_eq!(array.get(5), None);
    }

    #[test]
    pub fn set_values() {
        let length = 5;
        let mut array = Array::new(length);

        assert_eq!(array.set(0, 1), Ok(()));
        assert_eq!(array.set(1, 2), Ok(()));
        assert_eq!(array.set(2, 3), Ok(()));
        assert_eq!(array.set(3, 4), Ok(()));
        assert_eq!(array.set(4, 5), Ok(()));

        assert_eq!(array.set(5, 6), Err(super::OUT_OF_BOUND_ERROR));

        assert_eq!(array.get(0), Some(&Entry::Item(1)));
        assert_eq!(array.get(1), Some(&Entry::Item(2)));
        assert_eq!(array.get(2), Some(&Entry::Item(3)));
        assert_eq!(array.get(3), Some(&Entry::Item(4)));
        assert_eq!(array.get(4), Some(&Entry::Item(5)));
        assert_eq!(array.get(5), None);
    }

    #[test]
    pub fn remove_values() {
        let length = 5;
        let mut array = Array::new(length);

        assert_eq!(array.set(0, 1), Ok(()));
        assert_eq!(array.set(1, 2), Ok(()));
        assert_eq!(array.set(2, 3), Ok(()));
        assert_eq!(array.set(3, 4), Ok(()));
        assert_eq!(array.set(4, 5), Ok(()));

        assert_eq!(array.set(5, 6), Err(super::OUT_OF_BOUND_ERROR));

        assert_eq!(array.remove(2), Some(Entry::Item(3)));

        assert_eq!(array.get(0), Some(&Entry::Item(1)));
        assert_eq!(array.get(1), Some(&Entry::Item(2)));
        assert_eq!(array.get(2), Some(&Entry::Empty));
        assert_eq!(array.get(3), Some(&Entry::Item(4)));
        assert_eq!(array.get(4), Some(&Entry::Item(5)));
        assert_eq!(array.get(5), None);
    }

    #[test]
    pub fn shift_right() {
        let length = 5;
        let mut array = Array::new(length);

        assert_eq!(array.set(0, 1), Ok(()));
        assert_eq!(array.set(1, 2), Ok(()));
        assert_eq!(array.set(2, 3), Ok(()));

        array.shift_right(2, 4);

        assert_eq!(array.get(0), Some(&Entry::Item(1)));
        assert_eq!(array.get(1), Some(&Entry::Item(2)));
        assert_eq!(array.get(2), Some(&Entry::Empty));
        assert_eq!(array.get(3), Some(&Entry::Item(3)));
        assert_eq!(array.get(4), Some(&Entry::Empty));
        assert_eq!(array.get(5), None);
    }

    #[test]
    pub fn shift_left() {
        let length = 5;
        let mut array = Array::new(length);

        assert_eq!(array.set(0, 1), Ok(()));
        assert_eq!(array.set(1, 2), Ok(()));
        assert_eq!(array.set(3, 3), Ok(()));

        array.shift_left(2, 4);

        assert_eq!(array.get(0), Some(&Entry::Item(1)));
        assert_eq!(array.get(1), Some(&Entry::Item(2)));
        assert_eq!(array.get(2), Some(&Entry::Item(3)));
        assert_eq!(array.get(3), Some(&Entry::Empty));
        assert_eq!(array.get(4), Some(&Entry::Empty));
        assert_eq!(array.get(5), None);
    }

    #[test]
    pub fn resize() {
        let length = 3;
        let mut array = Array::new(length);

        assert_eq!(array.len(), length);

        assert_eq!(array.set(0, 1), Ok(()));
        assert_eq!(array.set(1, 2), Ok(()));
        assert_eq!(array.set(2, 3), Ok(()));

        let new_size = 4;
        let start_at = 0;
        let copy_length = 2;
        array.resize(new_size, start_at, copy_length);

        assert_eq!(array.len(), new_size);

        assert_eq!(array.get(0), Some(&Entry::Item(1)));
        assert_eq!(array.get(1), Some(&Entry::Item(2)));
        assert_eq!(array.get(2), Some(&Entry::Empty));
        assert_eq!(array.get(3), Some(&Entry::Empty));
        assert_eq!(array.get(4), None);
    }
}
