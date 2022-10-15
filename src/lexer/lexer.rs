use std::{vec::Vec, fmt::Debug};

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

pub struct Token {
	pub line: u32,
	pub col: u32,
	pub str: String,
	pub tt: TokenType,
}

pub fn lex_file(str: String) -> Vec<Token> {
	let lines = split_newline(str);

	let mut tokens: Vec<Token> = Vec::new();
	for (i, line) in lines.iter().enumerate() {
		gen_tokens(line, i as u32, &mut tokens);
	}
	
	tokens
}

fn split_newline(str: String) -> Vec<String> {
	let split: Vec<String> = str.lines().map(|s| s.to_string()).collect();
	split
}


fn gen_tokens(line: &String, line_num: u32, tokens: &mut Vec<Token>) {
	let ln = line_num + 1;
	
	let mut num_str: String = String::new();
	let mut reading_num: bool = false;
	let mut dot_count: u32 = 0; 

	for (i, c) in line.to_owned().chars().enumerate() {
		if c.is_whitespace() {
			continue;
		}
		if c.is_digit(10) {
			num_str += &String::from(c.to_string());
			reading_num = true;
			continue;
		}
		if reading_num {
			if c == '.' {
				dot_count += 1;
				assert!(dot_count <= 1, "line {ln}: col {i}: Error: Too many punctuations in float, ammount {dot_count}");
				continue;
			}
			let mut tt = TokenType::INT;
			if dot_count > 0 {
				tt = TokenType::FLOAT;
			}
			tokens.append(&mut vec![Token {
				line: ln,
				col: i as u32,
				str: num_str,
				tt,
			}]);
			reading_num = false;
			dot_count = 0;
			num_str = "".to_string();
		}
		let tt = match c {
			'+' => TokenType::PLUS,
			'-' => TokenType::MINUS,
			'*' => TokenType::MUL,
			'/' => TokenType::DIV,
			'(' => TokenType::LPAREN,
			')' => TokenType::RPAREN,
			'=' => TokenType::EQ,
			_ => TokenType::UNDEFINED, 
		};
		if tt == TokenType::UNDEFINED {
			assert!(false, "line {}: col {}: Error: undefined char: {}", ln, i, c.to_string());
		}
		tokens.append(&mut vec![Token {
			line: ln,
			col: i as u32,
			str: c.to_string(),
			tt,
		}]);
	}
}
