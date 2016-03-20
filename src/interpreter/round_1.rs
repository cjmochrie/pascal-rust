use std::fmt::{Formatter, self, Display};


#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Integer(u32),
    Plus,
    Minus,
    End,
}

impl Token {
    fn new(token: &str) -> Token {
        if token.chars().next().unwrap().is_numeric() {
            Token::Integer(token.parse::<u32>().unwrap())
        } else {
            match token.chars().next().unwrap() {
                '+' => Token::Plus,
                '-' => Token::Minus,
                _ => Token::End,
            }
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Token::Integer(value) => write!(f, "Token(Integer, {})",  value),
            Token::Plus  => write!(f, "Token(Plus, None)"),
            Token::Minus  => write!(f, "Token(Minus, None)"),
            Token::End => write!(f, "Token(End, None)"),
        }
    }
}

pub struct Interpreter {
    text: String,
    stack: Vec<u32>,
    ops: Vec<Token>,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter { text: String::from(""), stack: vec![], ops: vec![] }
    }

    fn get_next_token(&mut self) -> Token {
        let mut text = String::new();
        while self.text.chars().next().unwrap().is_numeric() {
            text.push(self.text.remove(0));    
        }
        if text.len() == 0 {
            text.push(self.text.remove(0));
        }
        Token::new(&text)
    }
  
    pub fn expr(&mut self, line: &str) -> u32 {
        self.text = String::from(line.replace(" ", ""));
        loop {
            match self.get_next_token() {
                Token::Integer(value) => {
                    if self.ops.len() > 0 {
                        let left = self.stack.pop().expect("should be one u32 on the stack");
                        let op = self.ops.pop().expect("At least one Token on the stack");
                        self.stack.push(calculate(left, value, op));
                    } else {
                        self.stack.push(value);
                    }
                },
                Token::Plus => self.ops.push(Token::Plus),
                Token::Minus => self.ops.push(Token::Minus),
                Token::End => break,
            }
        }
        self.stack.pop().expect("Should be one u32 on the stack")
    }
}

fn calculate(left: u32, right: u32, op: Token) -> u32 {
    match op {
        Token::Plus => left + right,
        Token::Minus => left - right,
        _ => panic!()
    }
}


#[cfg(test)]
mod tests {
    use super::Token;
    #[test]
    fn test_tokens_displayed() {
        let plus = Token::Plus;
        let end = Token::End;
        let nine = Token::Integer { value: 9 };
        println!("{} {} {}", plus, end, nine);
        assert_eq!(format!("{} {} {}", plus, end, nine), "Token(Plus, None) Token(End, None) Token(Integer, 9)")
    }

    use super::Interpreter;

    #[test]
    fn test_interpreter_created() {
        let interpreter = Interpreter::new("9 + 9");
        assert_eq!(interpreter.text, "9 + 9".to_string());
        assert_eq!(interpreter.pos, 0);
        
    }

    #[test]
    fn test_interpreter_returns_correct_tokens() {
        let mut interpreter = Interpreter::new("5+3");
        assert_eq!(interpreter.get_next_token(), Token::Integer { value: 5});
        assert_eq!(interpreter.get_next_token(), Token::Plus);
        assert_eq!(interpreter.get_next_token(), Token::Integer { value: 3});
        assert_eq!(interpreter.get_next_token(), Token::End);
    }

    #[test]
    #[should_panic]
    fn test_interpreter_panics_properly() {
        let mut interpreter = Interpreter::new("+5+3");
        println!("Result: {}", interpreter.expr());
    }

    #[test]
    fn test_interpreter_calculates_properly() {
        let mut interpreter = Interpreter::new("5+3");
        assert_eq!(interpreter.expr(), 8);
    }


}
