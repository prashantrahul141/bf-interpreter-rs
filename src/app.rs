use std::{self, env, fs, process::exit};

use crate::{lexer::Lexer, parser::Parser};

/// Top level App struct to control everything in one place.
pub struct App;

impl App {
    // simple constructor.
    pub fn new() -> Self {
        Self
    }

    // reads file content, returns string of file content otherwise panics.
    fn get_file_contents(filepath: &str) -> String {
        let contents = fs::read_to_string(filepath);

        if let Ok(file) = contents {
            return file;
        }

        panic!("Failed to open file.")
    }

    // start the interpreter.
    pub fn run(&self) {
        if env::args().len() != 2 {
            println!("BFIR - BrainF*ck Interpreter in Rust.\n\nUsage:\n    bfi [FILENAME]");
            exit(0);
        };

        let file_content = App::get_file_contents(&env::args().collect::<Vec<String>>()[1]);

        let mut lexer = Lexer::new(&file_content);

        let parser = Parser::new(&mut lexer.tokens);
        parser.parse();

        for token in lexer.tokens {
            dbg!(token);
        }
    }
}
