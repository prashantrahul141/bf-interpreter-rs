use std::{self, env, fs, process::exit};

use crate::{lexer::Lexer, parser::Parser, vm::Vm};

/// Top level App struct to control everything in one place.
pub struct App;

impl App {
    // simple constructor.
    pub fn new() -> Self {
        Self
    }

    // start the interpreter.
    pub fn run(&self) {
        // if != 2 number of arguments are passed.
        if env::args().len() != 2 {
            println!("BFIR - BrainF*ck Interpreter in Rust.\n\nUsage:\n    bfirs [FILENAME]");
            exit(0);
        };

        // read file content.
        let file_content = App::get_file_contents(&env::args().collect::<Vec<String>>()[1]);

        // create a new lexer, and tokenize file string.
        let mut lexer = Lexer::new(&file_content);

        // debug log
        for token in &lexer.tokens {
            dbg!(token);
        }

        // create a new parser
        let mut parser = Parser::new(&mut lexer);
        // parse the tokens to stmts
        parser.parse();

        // debug log
        for instruction in &parser.instructions {
            dbg!(instruction);
        }

        let mut vm = Vm::new();
        vm.run();
    }

    // reads file content, returns string of file content otherwise panics.
    fn get_file_contents(filepath: &str) -> String {
        let contents = fs::read_to_string(filepath);

        if let Ok(file) = contents {
            return file;
        }

        panic!("Failed to open file.")
    }
}
