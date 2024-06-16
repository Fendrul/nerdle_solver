use file_reader::file_reader::FileReader;
use arena::arena::Arena;
use OperationType::Add;
use crate::core::expressions::{Expression, OperationType};
use crate::core::expressions::Expression::Value;
use crate::core::expressions::OperationType::{Divide, Multiply, Subtract};
use crate::parser::Token::{OperationToken, ValueToken};

pub fn parse(file_path: &str) -> Vec<Vec<Expression>> {
    let file_reader = FileReader::new(file_path).expect("Could not read file");

    for line in file_reader {
        parse_line(&line);
    }

    todo!("Implement parsing logic here");
}

fn parse_line(line: &str) -> Vec<Token> {
    let mut vec = Vec::new();
    
    for char in line.chars() {
        vec.push(match char {
            '+' | '-' | '*' | '/' => {
                if let Some(OperationToken(_)) = vec.last() { panic!("Two consecutive operations at line {}", line) }
                
                OperationToken(map_char_to_operation_type(char)) 
            },
            '0'..='9' => ValueToken(char.to_digit(10).unwrap()),
            _ => panic!("Invalid character at line {}", line)
        });
    }
    
    todo!("Implement parsing logic here");
}

fn map_char_to_operation_type(char: char) -> OperationType {
    match char {
        '+' => Add,
        '-' => Subtract,
        '*' => Multiply,
        '/' => Divide,
        _ => panic!("Invalid operation type")
    }
}

enum Token {
    OperationToken(OperationType),
    ValueToken(u32)
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;
    use arena::arena::Arena;
    use Expression::{Operation, Value};
    use OperationType::Add;
    use crate::core::expressions::{Expression, OperationType};

    #[test]
    fn test_parse() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "1+2").unwrap();

        let arena_to_test = super::parse(file_path.to_str().unwrap());

        let mut arena = Arena::new();
        let one = arena.add_node(Value(1));
        let two = arena.add_node(Value(2));
        let add = arena.add_node(Operation(Add, one, two));
    }
}