use arena::arena::Arena;
use crate::core::expressions::Expression;
mod core;
mod tokenizer;

fn main() {
    println!("Hello, world!");

    let mut _test = Expression::Value;
    _test = Expression::Operation;

    let mut arena = Arena::new();
    
    arena.add_node(10);
}
