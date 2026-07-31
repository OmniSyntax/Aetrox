pub mod ast;

use crate::lexer::Token;
use ast::{Function, Statement};

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    /// Main entry point to parse a full .ax source file
    pub fn parse(&mut self) -> Result<Vec<Function>, String> {
        let mut functions = Vec::new();
        
        // Loop through tokens to parse top-level items like functions
        while self.position < self.tokens.len() {
            functions.push(self.parse_function()?);
        }

        Ok(functions)
    }

    fn parse_function(&mut self) -> Result<Function, String> {
        // Placeholder parser logic for: fn name() { ... }
        Ok(Function {
            name: "main".to_string(),
            body: vec![],
        })
    }
}
pub mod ast;
pub mod expr;

use crate::lexer::Token;
use ast::{Function, Statement};
use expr::ExprParser;

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Function>, String> {
        // Your parsing execution loop
        Ok(vec![])
    }
}