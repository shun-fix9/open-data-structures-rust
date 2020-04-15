mod backend;

pub trait Queue<T> {
    fn enqueue(x: T);
    fn dequeue() -> Option<T>;
}

pub trait Stack<T> {
    fn push(x: T);
    fn pop() -> Option<T>;
}

pub trait Deque<T> {
    fn addFirst(x: T);
    fn addLast(x: T);
    fn removeFirst() -> Option<T>;
    fn removeLast() -> Option<T>;
}

pub trait List<T> {
    fn size() -> usize;
    fn get(index: usize) -> Option<T>;
    fn set(index: usize, item: T) -> Result<(), Error>;
    fn add(index: usize, item: T) -> Result<(), Error>;
    fn remove(index: usize) -> Option<T>;
}

pub trait USet<T> {
    fn size() -> usize;
    fn add(item: T) -> bool;
    fn remove(item: T) -> Option<T>;
    fn find(item: &T) -> Option<T>;
}

pub trait SSet<T> {
    fn size() -> usize;
    fn add(item: T) -> bool;
    fn remove(item: T) -> Option<T>;
    fn find(item: &T) -> Option<T>;
}

pub struct Error {
    message: &'static str,
}
