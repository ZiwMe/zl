mod lexer;

use std::{vec::Vec, fs};

fn read_file(path: String) -> String {
	println!("Reading file at {}", path);
	fs::read_to_string(path).expect("Couldn't read file")
}

fn print_lex_tokens(tokens: &Vec<lexer::lexer::Token>) {
	for t in tokens.iter() {
		let padding = format!("{:?}", t.tt).chars().count();
		let mut pad_str: String = "".to_string();
		for _ in 0..(10 - padding) {
			pad_str += &" ".to_string();
		}
		println!("Token str: {} type: {:?}{} line: {}\t col: {}", t.str, t.tt, pad_str, t.line, t.col);
	}
}

fn main() {
	let file_content = read_file("file.zl".to_string());
    let tokens = lexer::lexer::lex_file(file_content);
	print_lex_tokens(&tokens);
}
