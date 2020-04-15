#[derive(Debug)]
pub struct Array<'a, T> {
    items: Vec<Entry<'a, T>>,
    length: usize,
}

impl<'a, T> Array<'a, T> {
    pub fn new(length: usize) -> Array<'a, T> {
        Array {
            items: vec![Entry::default(); length],
            length: length,
        }
    }

    pub fn get(&self, index: usize) -> Option<Entry<'a, T>> {
        if let Some(entry) = self.items.get(index) {
            match entry {
                Entry::Empty => Some(Entry::Empty),
                Entry::Item(item) => Some(Entry::Item(item)),
            }
        } else {
            None
        }
    }

    pub fn set(&mut self, index: usize, item: &'a T) -> Result<(), Error> {
        if index < self.length {
            self.items[index] = Entry::Item(item);
            Ok(())
        } else {
            Err(OutOfBoundError)
        }
    }

    pub fn drain<'b>(&mut self, other: Array<'b, T>)
    where
        'b: 'a,
    {
        self.items = other.items;
        self.length = other.length;
    }
}

#[derive(Debug, PartialEq)]
pub enum Entry<'a, T> {
    Empty,
    Item(&'a T),
}

impl<'a, T> Default for Entry<'a, T> {
    fn default() -> Self {
        Self::Empty
    }
}

impl<'a, T> Clone for Entry<'a, T> {
    fn clone(&self) -> Self {
        match self {
            Self::Empty => Self::Empty,
            Self::Item(item) => Self::Item(item),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Error {
    message: &'static str,
}

pub const OutOfBoundError: Error = Error {
    message: "out of bound",
};

#[cfg(test)]
mod tests {
    use super::Array;
    use super::Entry;

    #[test]
    pub fn new_array() {
        let length = 5;
        let array: Array<'static, i32> = Array::new(length);

        check(
            &array,
            vec![
                (0, Some(Entry::Empty)),
                (1, Some(Entry::Empty)),
                (2, Some(Entry::Empty)),
                (3, Some(Entry::Empty)),
                (4, Some(Entry::Empty)),
                (5, None),
            ],
        );
    }

    #[test]
    pub fn set_values() {
        let length = 5;
        let mut array: Array<'static, i32> = Array::new(length);

        assert_eq!(array.set(0, &1), Ok(()));
        assert_eq!(array.set(1, &2), Ok(()));
        assert_eq!(array.set(2, &3), Ok(()));
        assert_eq!(array.set(3, &4), Ok(()));
        assert_eq!(array.set(4, &5), Ok(()));

        assert_eq!(array.set(5, &6), Err(super::OutOfBoundError));

        check(
            &array,
            vec![
                (0, Some(Entry::Item(&1))),
                (1, Some(Entry::Item(&2))),
                (2, Some(Entry::Item(&3))),
                (3, Some(Entry::Item(&4))),
                (4, Some(Entry::Item(&5))),
                (5, None),
            ],
        );
    }

    #[test]
    pub fn drain() {
        let length = 5;
        let mut target: Array<'static, i32> = Array::new(length);
        let mut array: Array<'static, i32> = Array::new(length);

        // set [target]
        assert_eq!(target.set(0, &1), Ok(()));
        assert_eq!(target.set(1, &2), Ok(()));
        assert_eq!(target.set(2, &3), Ok(()));
        assert_eq!(target.set(3, &4), Ok(()));
        assert_eq!(target.set(4, &5), Ok(()));

        array.drain(target);

        check(
            &array,
            vec![
                (0, Some(Entry::Item(&1))),
                (1, Some(Entry::Item(&2))),
                (2, Some(Entry::Item(&3))),
                (3, Some(Entry::Item(&4))),
                (4, Some(Entry::Item(&5))),
                (5, None),
            ],
        );
    }

    fn check<'a, T>(array: &Array<'a, T>, items: Vec<(usize, Option<Entry<'a, T>>)>)
    where
        T: std::fmt::Debug + std::cmp::PartialEq,
    {
        for (i, entry) in items.iter() {
            assert_eq!(array.get(*i), *entry);
        }
    }
}
