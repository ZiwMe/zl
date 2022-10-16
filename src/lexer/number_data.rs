use std::vec::Vec;
use crate::lexer::tokens::{Token, TokenType};

// For storing number data peing lexed
pub struct NumberData {
	pub num: String,
	pub reading_num: bool,
	pub dots: u32,
	pub col: u32,
}
impl Default for NumberData {
	fn default() -> Self {
		NumberData { num: String::new(), reading_num: false, dots: 0, col: 0 }
	}
}
impl NumberData {
	// For starting/continuing a number being lexed
	pub fn handle_digit(&mut self, col: u32, c: char) {
		if !self.reading_num {
			self.reading_num = true;
			self.col = col;
		}
		self.num += &String::from(c.to_string());
	}
	// For handling '.' in float, and adding the number token to the vector of token
	pub fn handle_non_digit(&mut self, c: char, ln: u32, tokens: &mut Vec<Token>) -> bool {
		if c == '.' { 
			self.dots += 1;
			self.num += &String::from(".");
			assert!(self.dots <= 1, "line {}: col {}: Error: Too many punctuations in float, ammount {}",
					ln, self.col, self.dots);
			return true;
		}
		let tt = if self.dots > 0 { TokenType::FLOAT } else { TokenType::INT };
		tokens.append(&mut vec![Token {
			line: ln,
			col: self.col,
			str: format!("{}", self.num),
			tt
			
		}]);
		return false;
	}
}
