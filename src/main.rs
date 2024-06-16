use arena::arena::Arena;
use crate::core::expressions::Expression;
mod core;
mod parser;

fn main() {
    println!("Hello, world!");

    let mut _test = Expression::Value;

    let mut arena = Arena::new();
    
    arena.add_node(10);
}
