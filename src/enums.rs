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

// Instruction set supported by the vm.
#[derive(Debug)]
pub enum OpCode {
    // moves pointer one cell right.
    MovePtrForward,
    // moves pointer one cell left.
    MovePtrBackward,
    // reads one character from stdin.
    ReadFromStdin,
    // writes one character to stdout.
    WriteToStdout,
    // increment or decrement value at the ptr.
    Crement(i8),
}

// statements supported by the vm.
#[derive(Debug)]
pub enum Stmt {
    NodeStmt(OpCode),
    WhileStmt(Vec<Stmt>),
}
