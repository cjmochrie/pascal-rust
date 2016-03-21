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

#[derive(Debug)]
pub struct Lexer {
	text: Vec<char>,
	pos: usize,
}

impl Lexer {
	pub fn new(text: &str) -> Lexer {
		Lexer { 
			text: text.replace(" ", "")
			.chars()
			.collect(), 
			pos: 0, 
		}
	}

	pub fn get_next_token(&mut self) -> Token {
		if self.pos == self.text.len() { return Token::End }
		let mut text = String::new();
		while self.pos < self.text.len() && self.text[self.pos].is_numeric() {
			text.push(self.text[self.pos]);
			self.pos += 1;
		}
		if text.len() == 0 {
		    text.push(self.text[self.pos]);
		    self.pos += 1;
		}
	    Token::new(&text)
	    
	}

	pub fn retreat(&mut self) {
		self.pos -= 1;
	}
}

pub struct Interpreter {
	lexer: Lexer,
}

impl Interpreter {
	pub fn new(text: &str) -> Interpreter {
		Interpreter { lexer: Lexer::new(&text),  }
	}

	fn factor(&mut self) -> u32 {
		match self.lexer.get_next_token() {
			Token::Integer(x) => x,
			op @ _ => panic!(),
		}
	}

	fn term(&mut self) -> u32 {
		let mut result = self.factor();
		loop {
			match self.lexer.get_next_token() {
				Token::Multiply => result *= self.factor(),
				Token::Divide => result /= self.factor(),
				Token::End => break,
				_ => { self.lexer.retreat(); break },
			}
		}
		result
	}

	pub fn expr(&mut self) -> u32 {
		let mut result = self.term();
		loop {
			match self.lexer.get_next_token() {
				Token::Plus =>  result += self.term(),
				Token::Minus => result -= self.term(),
				_ => break
			}
		}
		result
	}
}

