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

    pub fn parse(&mut self) {
        while !matches!(self.lexer.peek().token_type, TokenType::Eof) {
            if let Some(instruction) = self.parse_instruction() {
                self.instructions.push(instruction);
            }
        }
    }

    fn parse_instruction(&mut self) -> Option<Stmt> {
        let current = self.lexer.pop();

        match current.token_type {
            TokenType::RightAngle => Some(Stmt::NodeStmt(OpCode::MovePtrForward)),
            TokenType::LeftAngle => Some(Stmt::NodeStmt(OpCode::MovePtrBackward)),
            TokenType::Plus => Some(Stmt::NodeStmt(OpCode::Crement(1))),
            TokenType::Minus => Some(Stmt::NodeStmt(OpCode::Crement(-1))),
            TokenType::Comma => Some(Stmt::NodeStmt(OpCode::ReadFromStdin)),
            TokenType::Dot => Some(Stmt::NodeStmt(OpCode::WriteToStdout)),
            TokenType::LeftSquare => {
                let mut inner_instructions: Vec<Stmt> = vec![];

                while !self.match_token(TokenType::RightSquare) {
                    if let Some(instruction) = self.parse_instruction() {
                        inner_instructions.push(instruction);
                    }
                }

                // consuming ']'
                self.lexer.pop();

                Some(Stmt::WhileStmt(inner_instructions))
            }
            TokenType::RightSquare => {
                Parser::error("Unexpected ']'", &current);
                None
            }
            TokenType::Eof => None,
        }
    }

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
