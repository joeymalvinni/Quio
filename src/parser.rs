use crate::token::TokenType;
use crate::err::generate_error_message;

use std::process;

pub struct Parser {
    source: String,
    index: usize,
}

impl Parser {
    pub fn new(input: String) -> Parser {
        Parser {
            source: input,
            index: 0,
        }
    }

    pub fn parse(&mut self) -> Vec<TokenType>{
        let mut tokens: Vec<TokenType> = vec![];

        for char in self.source.to_lowercase().chars() {
            match char {
                'q' => tokens.push(TokenType::Start),
                'u' => tokens.push(TokenType::Increment),
                'i' => tokens.push(TokenType::Decrement),
                'r' => tokens.push(TokenType::Reverse),
                'k' => tokens.push(TokenType::SwapNext),
                'p' => tokens.push(TokenType::SwapEmpty),
                'o' => tokens.push(TokenType::Output),
                _ => {
                    let err = generate_error_message(self.source.clone(), self.index, "Invalid character. Expected one of Q, U, I, R, K, P, O.");
                    eprintln!("{}", err);
                    process::exit(1);
                },
            }
        }

        tokens
    }
}