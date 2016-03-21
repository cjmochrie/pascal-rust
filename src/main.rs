use std::io;

extern crate pascal;
use pascal::interpreter;

pub fn main() {
    let mut input = String::new();
    loop {
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let to_parse: String = input.drain(..).collect();
                let mut interpreter = interpreter::part_4::Interpreter::new(&to_parse);
                println!(">> {}", interpreter.expr());
            }
            Err(error) => println!("error: {}", error),
        }
        
    }
}     

