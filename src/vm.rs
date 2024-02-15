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
            self.run_stmt(instruction);
        }
    }

    //  runs statements.
    fn run_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::NodeStmt(node) => self.run_opcode(node),
            Stmt::WhileStmt(node) => self.while_stmt(node),
        }
    }

    // while statements
    fn while_stmt(&mut self, node: &Vec<Stmt>) {
        while self.data[self.pointer] != 0 {
            for stmt in node {
                self.run_stmt(stmt);
            }
        }
    }

    // runs opcodes to vm.
    fn run_opcode(&mut self, node: &OpCode) {
        match node {
            // moves pointer forward.
            OpCode::MovePtrForward => self.pointer += 1,

            // moves pointer backward.
            OpCode::MovePtrBackward => self.pointer -= 1,

            // reads input from stdin and writes to current pointed cell.
            OpCode::ReadFromStdin => {
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap_or(0);
                let first_char = input.bytes().nth(0).unwrap_or(0);
                self.data[self.pointer] = first_char;
            }

            // writes current cell as char to stdout.
            OpCode::WriteToStdout => {
                print!(
                    "{}",
                    std::char::from_u32(self.data[self.pointer] as u32).unwrap_or('0')
                )
            }

            // increments or decrements current pointed value.
            OpCode::Crement(value) => {
                self.data[self.pointer] = (self.data[self.pointer] as i8 + value) as u8;
            }
        }
    }
}
