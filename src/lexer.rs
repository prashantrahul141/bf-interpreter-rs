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

        input
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
            .collect()
    }
}
