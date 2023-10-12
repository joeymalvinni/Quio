mod parser;
mod interpreter;
pub mod token;
pub mod err;

use std::env;
use std::fs;
use std::process;
use interpreter::Interpreter;
use parser::Parser;

struct Quio {
    program: Interpreter,
    parser: Parser,
    contents: String,
}

impl Quio {
    fn new(name: String) -> Self {
        let file_contents: Result<String, std::io::Error> = fs::read_to_string(name.clone());

        let mut file_contents: String = match file_contents {
            Ok(file) => file,
            Err(_e) => {
                eprintln!("Error: The system cannot find the path specified. Finding file {}", name);
                process::exit(1);
            }
        };

        file_contents.retain(|c| !c.is_whitespace()); // remove whitespace

        Quio {
            program: Interpreter::new(),
            parser: Parser::new(file_contents.clone()),
            contents: file_contents,
        }
    }

    fn run(&mut self) {
        let tokens = self.parser.parse();
        self.program.interpret(tokens, self.contents.clone());
    }    
}

fn main() {
    let args: Vec<_> = env::args().skip(1).collect();

    if let Some(file) = args.get(0) {
        let mut quio = Quio::new(file.to_string());

        quio.run();
    } else {
        eprintln!("Invalid arguments. Expected filename as first argument.");
        process::exit(1);
    }
}