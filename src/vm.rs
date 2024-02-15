use crate::enums::{OpCode, Stmt};

// Our virtual machine which will execute instructions.
pub struct Vm<'a> {
    instructions: &'a Vec<Stmt>,
    pointer: usize,
    data: [u8; 30],
}

impl Vm {
    // constructor
    pub fn new() -> Self {
        Self
    }

    // Public run method to start executon
    pub fn run(&mut self) {}
}
