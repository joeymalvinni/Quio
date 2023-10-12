use std::vec;
use std::process;

use crate::token::TokenType;
use crate::err::generate_error_message;

pub struct Interpreter {
    pub cells: Vec<i32>,
    pub index: usize,
    pub current: usize, 
    pub contents: String,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            cells: vec![ 0 ],
            current: 0,
            index: 0,
            contents: String::new(),
        }
    }

    pub fn interpret(&mut self, statements: Vec<TokenType>, code: String) {
        self.contents = code;

        for statement in statements {
            self.execute(statement);
        }
    }

    fn execute(&mut self, statement: TokenType) {
        match statement {
            TokenType::Start => {},
            TokenType::Increment => self.handle_increment(),
            TokenType::Decrement => self.handle_decrement(),
            TokenType::Output => self.handle_output(),
            TokenType::Reverse => self.handle_reverse(),
            TokenType::SwapEmpty => self.handle_swap_empty(),
            TokenType::SwapNext => self.handle_swap_next(),
        };

        self.current += 1;
    }

    fn handle_increment(&mut self) {
        self.cells[self.index] += 1;
    }

    fn handle_decrement(&mut self) {
        if (self.cells[self.index] - 1) >= 0 {
            self.cells[self.index] -= 1;
        } else {
            let err = generate_error_message(self.contents.clone(), self.current, "The byte at the current data pointer cannot be decreased below 0.");
            eprintln!("{}", err);
            process::exit(1);
        }
    }

    fn handle_output(&self) {
        println!("{}", self.cells[self.index]);
        print!("{}", self.cells[self.index] as u8 as char);
    }

    fn handle_reverse(&mut self) {
        self.cells.reverse();
    }

    fn handle_swap_empty(&mut self) {
        if let Some(front_element) = self.cells.first().cloned() {
            self.cells.remove(0);
            self.cells.push(front_element);
            self.cells.insert(0, 0);
        }
    }

    fn handle_swap_next(&mut self) {
        if self.cells.len() > 2 { 
            self.cells.swap(0, 1);
        } else {
            let err = generate_error_message(self.contents.clone(), self.current, "Cannot perform a swap on 1 cell.");
            eprintln!("{}", err);
            process::exit(1);
        }
    }
}