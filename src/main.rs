use open_data_structures::array_stack;
use open_data_structures::Stack;

fn main() {
    println!("Hello, world!");
    array_stack();
}

fn array_stack() {
    let mut array = array_stack::ArrayStack::new();

    array.push(42);

    println!("{:?}", array);
}
