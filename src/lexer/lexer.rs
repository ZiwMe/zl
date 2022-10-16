use std::{vec::Vec, fmt::Debug};

// Enum for specifying what type of token it is
#[derive(PartialEq, Debug)]
pub enum TokenType {
	INT,
	FLOAT,
	PLUS,
	MINUS,
	MUL,
	DIV,
	EQ,
	LPAREN,
	RPAREN,
	UNDEFINED,
}

// Token data
pub struct Token {
	pub line: u32,
	pub col: u32,
	pub str: String,
	pub tt: TokenType,
}

// Main function to lex the file
pub fn lex_file(str: String) -> Vec<Token> {
	// Split the file content into a vector of strings, one per line
	let lines = split_newline(str);

	// Vector for the generated tokens
	let mut tokens: Vec<Token> = Vec::new();
	// Enumerate over the lines
	for (i, line) in lines.iter().enumerate() {
		// Generate the tokens for that line
		gen_tokens(line, (i as u32) + 1, &mut tokens);
	}
	
	tokens
}

// Splits a string by newlines and returns a Vector of those splits
fn split_newline(str: String) -> Vec<String> {
	let split: Vec<String> = str.lines().map(|s| s.to_string()).collect();
	split
}

// For storing number data peing lexed
struct NumberData {
	num: String,
	reading_num: bool,
	dots: u32,
	col: u32,
}
impl Default for NumberData {
	fn default() -> Self {
		NumberData { num: String::new(), reading_num: false, dots: 0, col: 0 }
	}
}
impl NumberData {
	// For starting/continuing a number being lexed
	fn handle_digit(&mut self, col: u32, c: char) {
		if !self.reading_num {
			self.reading_num = true;
			self.col = col;
		}
		self.num += &String::from(c.to_string());
	}
	// For handling '.' in float, and adding the number token to the vector of token
	fn handle_non_digit(&mut self, c: char, ln: u32, tokens: &mut Vec<Token>) -> bool {
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

// Generate tokens for a line 
fn gen_tokens(line: &String, ln: u32, tokens: &mut Vec<Token>) {
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
