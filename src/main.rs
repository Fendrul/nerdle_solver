use crate::core::expressions::Expression;
mod core;
mod tokenizer;

fn main() {
    println!("Hello, world!");

    let mut _test = Expression::Value;
    _test = Expression::Operation;
}
