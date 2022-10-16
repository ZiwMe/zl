use std::fmt::Debug;

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
