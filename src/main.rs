mod lexer;

use std::{vec::Vec, fs};

// Function that reads a file at a given location, and returns it as a string
fn read_file(path: String) -> String {
	println!("Reading file at {}", path);
	fs::read_to_string(path).expect("Couldn't read file")
}

// Function for gnerating padding strings
fn gen_padding_str(len: usize, ammount: usize) -> String {
	let mut padd = String::new();
	for _ in 0..(ammount - len) {
		padd += &" ".to_string();
	}
	padd
}

// Debug function to nicely print the lexer tokens
fn print_lex_tokens(tokens: &Vec<lexer::lexer::Token>) {
	// Loops over all the tokens
	for t in tokens.iter() {
		// Generates a string to use as padding after the token string
		let str_padd = gen_padding_str((t.str).chars().count(), 5);
		
		// Generates a string to use as padding after the token type
		let tt_padd = gen_padding_str(format!("{:?}", t.tt).chars().count(), 10);

		// Prints the token in a formated way that is easy to read
		println!("Token str: {}{} type: {:?}{} line: {}\t col: {}", t.str, str_padd, t.tt, tt_padd, t.line, t.col);
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
