use crate::enums::{OpCode, Stmt};

// Our virtual machine which will execute instructions.
pub struct Vm<'a> {
    instructions: &'a Vec<Stmt>,
    pointer: usize,
    data: [u8; 30],
}

impl<'a> Vm<'a> {
    // constructor
    pub fn new(instructions: &'a Vec<Stmt>) -> Self {
        Self {
            pointer: 0,
            data: [0; 30],
            instructions,
        }
    }

    // Public run method to start executon
    pub fn run(&mut self) {
        for instruction in self.instructions {
            match instruction {
                Stmt::NodeStmt(node) => self.run_node(node),
                Stmt::WhileStmt(_) => todo!(),
            }
        }
    }

}
