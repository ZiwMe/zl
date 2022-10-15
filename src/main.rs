mod lexer;

use std::{vec::Vec, fs};

// Function that reads a file at a given location, and returns it as a string
fn read_file(path: String) -> String {
	println!("Reading file at {}", path);
	fs::read_to_string(path).expect("Couldn't read file")
}

// Debug function to nicely print the lexer tokens
fn print_lex_tokens(tokens: &Vec<lexer::lexer::Token>) {
	// Loops over all the tokens
	for t in tokens.iter() {
		// Generates a string to use as padding after the token type
		let padding = format!("{:?}", t.tt).chars().count();
		let mut pad_str: String = "".to_string();
		// TODO: Find a better way to do this
		for _ in 0..(10 - padding) {
			pad_str += &" ".to_string();
		}
		// Prints the token in a formated way that is easy to read
		println!("Token str: {} type: {:?}{} line: {}\t col: {}", t.str, t.tt, pad_str, t.line, t.col);
	}
}

fn main() {
	// Get the file content
	let file_content = read_file("file.zl".to_string());
	// Lex the file
    let tokens = lexer::lexer::lex_file(file_content);
	// Print the tokens from the lexer
	print_lex_tokens(&tokens);
}
