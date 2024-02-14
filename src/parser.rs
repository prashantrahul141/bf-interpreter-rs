use crate::{
    enums::{OpCode, Stmt, Token, TokenType},
    lexer::Lexer,
};

/// Parser - Parses stream of tokens into vector of instructions.
pub struct Parser<'a> {
    tokens: &'a mut Vec<Token>,
}

impl<'a> Parser<'a> {
    // constructor.
    pub fn new(tokens: &'a mut Vec<Token>) -> Self {
        Self { tokens }
    }

    pub fn parse(&self) {}
}
