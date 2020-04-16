use std::error::Error;
use std::fmt;

pub mod array_deque;
pub mod array_queue;
pub mod array_stack;
mod backend;

pub trait Queue<T> {
    fn enqueue(&mut self, x: T);
    fn dequeue(&mut self) -> Option<T>;
}

pub trait Stack<T> {
    fn push(&mut self, x: T);
    fn pop(&mut self) -> Option<T>;
}

pub trait Deque<T> {
    fn addFirst(&mut self, x: T);
    fn addLast(&mut self, x: T);
    fn removeFirst(&mut self) -> Option<T>;
    fn removeLast(&mut self) -> Option<T>;
}

pub trait List<T> {
    fn size(&self) -> usize;
    fn get(&self, index: usize) -> Option<&T>;
    fn set(&mut self, index: usize, item: T) -> Result<(), CollectionError>;
    fn add(&mut self, index: usize, item: T) -> Result<(), CollectionError>;
    fn remove(&mut self, index: usize) -> Option<T>;
}

pub trait USet<T> {
    fn size(&self) -> usize;
    fn add(&mut self, item: T) -> bool;
    fn remove(&mut self, item: T) -> Option<T>;
    fn find(&self, item: &T) -> Option<&T>;
}

pub trait SSet<T> {
    fn size(&self) -> usize;
    fn add(&mut self, item: T) -> bool;
    fn remove(&mut self, item: T) -> Option<T>;
    fn find(&self, item: &T) -> Option<&T>;
}

#[derive(Debug)]
pub struct CollectionError {
    message: &'static str,
}

impl CollectionError {
    pub const fn message(&self) -> &'static str {
        self.message
    }
}

impl fmt::Display for CollectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "error: {}", self.message)
    }
}

impl Error for CollectionError {}

pub const OUT_OF_BOUND_ERROR: CollectionError = CollectionError {
    message: "out of bound",
};
