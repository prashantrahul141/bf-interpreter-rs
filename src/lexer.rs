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

    /// Scans a string into Vec of Tokens.
    /// # Arguments
    /// * input - Immutable reference to the string to scan.
    /// # Returns
    /// * tokens - Vector of Tokens scanned.
    fn scan_tokens(input: &String) -> Vec<Token> {
        let mut line = 0;

        let mut tokens = input
            // taking characters of strings.
            .chars()
            // mapping through it.
            .flat_map(|lexeme| match lexeme {
                // if the character is >
                '>' => Some(Token {
                    line,
                    token_type: TokenType::RightAngle,
                }),

                // if the character is <
                '<' => Some(Token {
                    line,
                    token_type: TokenType::LeftAngle,
                }),

                // if the character is +
                '+' => Some(Token {
                    line,
                    token_type: TokenType::Plus,
                }),

                // if the character is -
                '-' => Some(Token {
                    line,
                    token_type: TokenType::Minus,
                }),

                // if the character is ]
                ']' => Some(Token {
                    line,
                    token_type: TokenType::RightSquare,
                }),

                // if the character is [
                '[' => Some(Token {
                    line,
                    token_type: TokenType::LeftSquare,
                }),

                // if the character is ,
                ',' => Some(Token {
                    line,
                    token_type: TokenType::Comma,
                }),

                // if the character is .
                '.' => Some(Token {
                    line,
                    token_type: TokenType::Dot,
                }),

                // if its a newline character, we increase line number count.
                '\n' => {
                    line += 1;
                    None
                }

                // consider everything else as comments.
                _ => None,
            })
            .collect::<Vec<Token>>();

        // push a EOF token at the end.
        tokens.push(Token {
            line: 0,
            token_type: TokenType::Eof,
        });

        tokens.reverse();

        tokens
    }

    /// Removes and Returns the next Token.
    pub fn pop(&mut self) -> Token {
        self.tokens
            .pop()
            .expect("[Lexer] Failed to get next token.")
    }

    /// Returns reference to the next Token without removing it.
    pub fn peek(&self) -> &Token {
        &self
            .tokens
            .last()
            .expect("[Lexer] Failed to peek next token.")
    }
}
