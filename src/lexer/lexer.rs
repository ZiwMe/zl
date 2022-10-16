use std::vec::Vec;
use crate::lexer::{tokenizer::gen_tokens, tokens::Token};


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

