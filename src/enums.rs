// All the operators which exists in brainf*ck
#[derive(Debug, PartialEq)]
pub enum TokenType {
    RightAngle,  // >
    LeftAngle,   // <
    Plus,        // +
    Minus,       // -
    RightSquare, // ]
    LeftSquare,  // [
    Comma,       // ,
    Dot,         // .
    Eof,         // end of file.
}

/// Holds information about a token.
#[derive(Debug)]
pub struct Token {
    pub line: usize,
    pub token_type: TokenType,
}

