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

    fn is_at_end(&self) -> bool {
        self.lexer.tokens.len() == 0
    }

    fn match_token(&self, tt: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.lexer.peek().token_type == tt
    }

    fn error(msg: &str, token: &Token) {
        println!("line : {} : {}", token.line, msg);
    }
}
