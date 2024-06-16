use arena::arena::NodeRef;
use Expression::{Operation, Value};
use OperationType::{Add, Divide, Multiply, Subtract};
use crate::core::expressions::DivideError::{DivideNotEvenly, DivisionByZero};

pub enum Expression {
    Operation(OperationType, NodeRef<Expression>, NodeRef<Expression>),
    Value(u8)
}

impl Expression {
    pub fn interpret(&self) -> Result<i32, DivideError> {
        match self {
            Operation(operation, left, right) => {
                let left_value = left.borrow().value().interpret()?;
                let right_value = right.borrow().value().interpret()?;

                match operation {
                    Add => Ok(left_value + right_value),
                    Subtract => Ok(left_value - right_value),
                    Multiply => Ok(left_value * right_value),
                    Divide => {
                        if right_value == 0 {
                            return Err(DivisionByZero)
                        }

                        if left_value % right_value != 0 {
                            return Err(DivideNotEvenly)
                        }

                        Ok(left_value / right_value)
                    }
                }
            },

            Value(value) => Ok(*value as i32)
        }
    }
}


#[derive(PartialEq, Debug)]
pub enum OperationType {
    Add,
    Subtract,
    Multiply,
    Divide
}

#[derive(PartialEq, Debug)]
pub enum DivideError {
    DivisionByZero,
    DivideNotEvenly
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;
    use arena::arena::Arena;

    type NodeRef<T> = Rc<RefCell<T>>;

    fn node<T>(value: T) -> NodeRef<T> {
        Rc::new(RefCell::new(value))
    }

    #[test]
    fn test_addition() {
        let mut arena = Arena::new();

        let three = arena.add_node(Value(3));
        let four = arena.add_node(Value(4));

        let expr = Operation(Add, three, four, );
        assert_eq!(expr.interpret(), Ok(7));
    }

    #[test]
    fn test_subtraction() {
        let mut arena = Arena::new();
        
        let ten = arena.add_node(Value(10));
        let four = arena.add_node(Value(4));
        
        let expr = Operation(Subtract, ten, four);
        assert_eq!(expr.interpret(), Ok(6));
    }

    #[test]
    fn test_multiplication() {
        let mut arena = Arena::new();

        let three = arena.add_node(Value(3));
        let four = arena.add_node(Value(4));
        
        let expr = Operation(Multiply, three, four);
        assert_eq!(expr.interpret(), Ok(12));
    }

    #[test]
    fn test_division() {
        let mut arena = Arena::new();
        
        let eight = arena.add_node(Value(8));
        let two = arena.add_node(Value(2));
        
        let expr = Operation(Divide, eight, two);
        assert_eq!(expr.interpret(), Ok(4));
    }

    #[test]
    fn test_division_by_zero() {
        let mut arena = Arena::new();
        
        let eight = arena.add_node(Value(8));
        let zero = arena.add_node(Value(0));
        
        let expr = Operation(Divide, eight, zero);
        assert_eq!(expr.interpret(), Err(DivisionByZero));
    }

    #[test]
    fn test_divide_not_evenly() {
        let mut arena = Arena::new();
        
        let seven = arena.add_node(Value(7));
        let two = arena.add_node(Value(2));
        
        let expr = Operation(Divide, seven, two);
        assert_eq!(expr.interpret(), Err(DivideNotEvenly));
    }

    #[test]
    fn test_nested_operations() {
        let mut arena = Arena::new();
        
        let two = arena.add_node(Value(2));
        let three = arena.add_node(Value(3));
        
        let add = arena.add_node(Operation(Add, two, three));
        
        let five = arena.add_node(Value(5));
        
        let multiply = Operation(Multiply, add, five);
        assert_eq!(multiply.interpret(), Ok(25));
    }
}
