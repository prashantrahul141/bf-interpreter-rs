use crate::token::{Token, TokenType};

/// Lexer for the interpreter.
pub struct Lexer {
    /// Vector of tokens scanned.
    pub tokens: Vec<Token>,
}

impl Lexer {
    /// simple constructor
    /// which internally calls the scanner.
    /// so basically we scan the tokens at the
    /// time of creating a new lexer instance.
    pub fn new(input: &String) -> Self {
        let tokens = Lexer::scan_tokens(input);
        Self { tokens }
    }
}
