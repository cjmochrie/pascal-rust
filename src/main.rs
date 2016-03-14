use std::io;

extern crate pascal;
use pascal::interpreter;

pub fn main() {
    let mut input = String::new();
    let mut interpreter = interpreter::round_1::Interpreter::new();
    loop {
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                let to_parse: String = input.drain(..).collect();
                println!(">> {}", interpreter.expr(&to_parse));
            }
            Err(error) => println!("error: {}", error),
        }
        
    }
}     

