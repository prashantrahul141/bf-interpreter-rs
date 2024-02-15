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
            println!("bfir - Brainf*ck Interpreter in Rust.\n\nUsage:\n    bfirs [filename]");
            exit(0);
        };

        // read file content.
        let file_content = App::get_file_contents(&env::args().collect::<Vec<String>>()[1]);

        // create a new lexer, and tokenize file string.
        let mut lexer = Lexer::new(&file_content);

        // create a new parser
        let mut parser = Parser::new(&mut lexer);
        // parse the tokens to stmts
        parser.parse();

        // creates a new vm
        let mut vm = Vm::new(&parser.statements);
        // run statements.
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
