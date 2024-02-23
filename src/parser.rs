use crate::{
    enums::{OpCode, Stmt, Token, TokenType},
    lexer::Lexer,
};

/// Parser - Parses stream of tokens into vector of instructions.
pub struct Parser<'a> {
    lexer: &'a mut Lexer,
    pub statements: Vec<Stmt>,
}

impl<'a> Parser<'a> {
    // constructor.
    pub fn new(lexer: &'a mut Lexer) -> Self {
        Self {
            lexer,
            statements: vec![],
        }
    }

    /// Public parse function which instiates the parsing process.
    pub fn parse(&mut self) {
        while !self.match_token(TokenType::Eof) {
            if let Some(stmt) = self.parse_stmt() {
                self.statements.push(stmt);
            }
        }
    }

    /// Parses indiviual instruction.
    fn parse_stmt(&mut self) -> Option<Stmt> {
        let current = self.lexer.pop();

        match current.token_type {
            // >
            TokenType::RightAngle => Some(Stmt::NodeStmt(OpCode::MovePtrForward)),
            // <
            TokenType::LeftAngle => Some(Stmt::NodeStmt(OpCode::MovePtrBackward)),
            // +
            TokenType::Plus => Some(Stmt::NodeStmt(OpCode::Crement(1))),
            // -
            TokenType::Minus => Some(Stmt::NodeStmt(OpCode::Crement(-1))),
            // ,
            TokenType::Comma => Some(Stmt::NodeStmt(OpCode::ReadFromStdin)),
            // .
            TokenType::Dot => Some(Stmt::NodeStmt(OpCode::WriteToStdout)),
            // for '[' we call the parse_instruction function recurisvely.
            TokenType::LeftSquare => {
                let mut inner_instructions: Vec<Stmt> = vec![];

                while !self.match_token(TokenType::RightSquare) {
                    if let Some(instruction) = self.parse_stmt() {
                        inner_instructions.push(instruction);
                    }
                }

                // consuming ']'
                self.lexer.pop();

                Some(Stmt::WhileStmt(inner_instructions))
            }
            // if we encounted a ']' without a starting ']' its a parser error.
            TokenType::RightSquare => {
                Parser::error("Unexpected ']'", &current);
                None
            }
            // shoudnt be reachable.
            TokenType::Eof => None,
        }
    }

    // Tells if there are no more tokens to parse.
    fn is_at_end(&self) -> bool {
        self.lexer.tokens.is_empty()
    }

    // matches the current token with the given type.
    fn match_token(&self, tt: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.lexer.peek().token_type == tt
    }

    // error reporting for our parser.
    fn error(msg: &str, token: &Token) {
        println!("line : {} : {}", token.line, msg);
    }
}
