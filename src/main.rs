use open_data_structures::array_deque::ArrayDeque;
use open_data_structures::array_queue::ArrayQueue;
use open_data_structures::array_stack::ArrayStack;
use open_data_structures::Deque;
use open_data_structures::Queue;
use open_data_structures::Stack;

fn main() {
    array_stack();
    array_queue();
    array_deque();
}

fn array_stack() {
    let mut stack = ArrayStack::new();

    stack.push(1);
    stack.push(2);
    stack.push(42);

    println!("{:?}", stack);
    println!("ArrayStack.pop: {:?}", stack.pop());
    println!("ArrayStack.pop: {:?}", stack.pop());
    println!("{:?}", stack);
}

fn array_queue() {
    let mut queue = ArrayQueue::new();

    queue.enqueue(1);
    queue.enqueue(2);
    queue.enqueue(42);

    println!("{:?}", queue);
    println!("ArrayQueue.dequeue: {:?}", queue.dequeue());
    println!("ArrayQueue.dequeue: {:?}", queue.dequeue());
    println!("{:?}", queue);
}

fn array_deque() {
    let mut deque = ArrayDeque::new();

    deque.add_first(1);
    deque.add_first(2);
    deque.add_first(42);
    deque.add_last(0);
    deque.add_last(3);
    deque.add_last(5);

    println!("{:?}", deque);
    println!("ArrayDeque.remove_first: {:?}", deque.remove_first());
    println!("ArrayDeque.remove_first: {:?}", deque.remove_first());
    println!("ArrayDeque.remove_last:  {:?}", deque.remove_last());
    println!("ArrayDeque.remove_last:  {:?}", deque.remove_last());
    println!("{:?}", deque);
}
