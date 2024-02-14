use crate::{
    enums::{OpCode, Stmt, Token, TokenType},
    lexer::Lexer,
};

/// Parser - Parses stream of tokens into vector of instructions.
pub struct Parser<'a> {
    lexer: &'a mut Lexer,
    pub instructions: Vec<Stmt>,
}

impl<'a> Parser<'a> {
    // constructor.
    pub fn new(lexer: &'a mut Lexer) -> Self {
        Self {
            lexer,
            instructions: vec![],
        }
    }

    pub fn parse(&self) {}
}
