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

// Generate tokens for a line 
fn gen_tokens(line: &String, ln: u32, tokens: &mut Vec<Token>) {
	// Data for keeping track of number data
	let mut num_str: String = String::new(); // str version of the number
	let mut reading_num: bool = false; // bool to tell the code if it is currently reading a number
	let mut dot_count: u32 = 0; // keeping track of the ammount of dots in a float to catch bugs
	let mut start_col_num = 0; // Keeping track of the start of the number

	// Enumerate over the characters in the line
	for (i, c) in line.to_owned().chars().enumerate() {
		// Skip whitespace ' ' && '\t'
		if c.is_whitespace() {
			continue;
		}
		// Check to see if its a number
		if c.is_digit(10) {
			// If it is a number, and is not reading a number, aka the start of the number
			if !reading_num {
				reading_num = true;
				start_col_num = i;
			}
			// Add the number char to the str of the currnt number
			num_str += &String::from(c.to_string());
			continue;
		}
		// not reading a number char, but is currently reading a line, aka a '.' or the number just stopped
		if reading_num {
			// Is the char a dot?
			if c == '.' {
				// Increase the dot count 
				dot_count += 1;
				num_str += &".".to_string();
				// panick if there are too many dot's in a float
				assert!(dot_count <= 1, "line {ln}: col {i}: Error: Too many punctuations in float, ammount {dot_count}");
				continue;
			}
			// What type of number is it
			let mut tt = TokenType::INT;
			// Has it 1 or more dots?
			if dot_count > 0 {
				// make it a float
				tt = TokenType::FLOAT;
			}
			// Add the number token to the vector of tokens
			tokens.append(&mut vec![Token {
				line: ln,
				col: start_col_num as u32,
				str: num_str,
				tt,
			}]);
			// Reset values
			reading_num = false;
			dot_count = 0;
			num_str = "".to_string();
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
	if reading_num {
		// What type of number is it
		let mut tt = TokenType::INT;
		// Has it 1 or more dots?
		if dot_count > 0 {
			// make it a float
			tt = TokenType::FLOAT;
		}
		// Add the number token to the vector of tokens
		tokens.append(&mut vec![Token {
			line: ln,
			col: start_col_num as u32,
			str: num_str,
			tt,
		}]);
	}
}
