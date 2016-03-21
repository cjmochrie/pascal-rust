#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Integer(u32),
    Plus,
    Minus,
    Multiply,
    Divide,
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
                '*' => Token::Multiply,
                '/' => Token::Divide,
                _ => Token::End,
            }
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
                Token::End => break,               
                op @ _ => self.ops.push(op),
            }
        }
        self.stack.pop().expect("Should be one u32 on the stack")
    }
}

fn calculate(left: u32, right: u32, op: Token) -> u32 {
    match op {
        Token::Plus => left + right,
        Token::Minus => left - right,
        Token::Multiply => left * right,
        Token::Divide => left / right,
        _ => panic!()
    }
}
