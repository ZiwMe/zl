use std::vec::Vec;
use crate::lexer::{
	tokens::{Token, TokenType},
	number_data::NumberData
};

// Generate tokens for a line 
pub fn gen_tokens(line: &String, ln: u32, tokens: &mut Vec<Token>) {
	// Data for keeping track of number data
	let mut num_data = NumberData{..Default::default()};

	// Enumerate over the characters in the line
	for (i, c) in line.to_owned().chars().enumerate() {
		// Skip whitespace ' ' && '\t'
		if c.is_whitespace() {
			continue;
		}
		// Check to see if its a number
		if c.is_digit(10) {
			num_data.handle_digit(i as u32, c);
			continue;
		}
		// not reading a number char, but is currently reading a line, aka a '.' or the number just stopped
		if num_data.reading_num {
			if num_data.handle_non_digit(c, ln, tokens) {continue;}
			else {num_data = NumberData {..Default::default()};}
		}

		// Figure out the token type for single char tokens
		let tt = match c {
			'+' => TokenType::PLUS,
			'-' => TokenType::MINUS,
			'*' => TokenType::MUL,
			'/' => TokenType::DIV,
			'(' => TokenType::LPAREN,
			')' => TokenType::RPAREN,
			'=' => TokenType::EQ,
			_ => {
				// Special token matching for panicking if it found an undefined token
				assert!(false, "line {}: col {}: Error: undefined char: {}", ln, i, c.to_string());
				TokenType::UNDEFINED
			}, 
		};
		// Append the token to the vector of tokens
		tokens.append(&mut vec![Token {
			line: ln,
			col: i as u32,
			str: c.to_string(),
			tt,
		}]);
	}
	// For appending the number token to the vector of tokens...
	// this is to prevent the lexer from not appending a number
	// that was at the end of a line
	if num_data.reading_num {
		num_data.handle_non_digit(' ', ln, tokens);
	}
}
