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

    fn run_node(&mut self, node: &OpCode) {
        match node {
            OpCode::MovePtrForward => self.pointer += 1,
            OpCode::MovePtrBackward => self.pointer -= 1,
            OpCode::ReadFromStdin => {
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap_or(0);
                let first_char = input.bytes().nth(0).unwrap_or(0);
                self.data[self.pointer] = first_char;
            }

            OpCode::WriteToStdout => {
                print!(
                    "{}",
                    std::char::from_u32(self.data[self.pointer] as u32).unwrap_or('0')
                )
            }
            OpCode::Crement(value) => {
                self.data[self.pointer] = (self.data[self.pointer] as i8 + value) as u8;
            }
        }

    }
}
