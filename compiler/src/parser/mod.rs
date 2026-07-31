pub mod ast;
pub mod expr;

use crate::lexer::Token;
use ast::{Function, Statement, Type, Span};
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

    /// Main entry point to parse tokens into an AST
    pub fn parse(&mut self) -> Result<Vec<Function>, String> {
        let mut functions = Vec::new();
        
        while self.position < self.tokens.len() {
            functions.push(self.parse_function()?);
        }

        Ok(functions)
    }

    fn parse_function(&mut self) -> Result<Function, String> {
        let start_pos = self.position;

        // Expect 'fn'
        if self.position < self.tokens.len() && self.tokens[self.position] == Token::Fn {
            self.position += 1;
        } else {
            return Err("Expected 'fn' keyword at start of function definition".to_string());
        }

        // Expect function name identifier
        let name = match self.tokens.get(self.position) {
            Some(Token::Identifier(n)) => {
                self.position += 1;
                n.clone()
            }
            _ => return Err("Expected function name identifier".to_string()),
        };

        // Expect '('
        if self.position < self.tokens.len() && self.tokens[self.position] == Token::LeftParen {
            self.position += 1;
        } else {
            return Err("Expected '(' after function name".to_string());
        }

        // TODO: Parse parameters here in the future
        // Expect ')'
        if self.position < self.tokens.len() && self.tokens[self.position] == Token::RightParen {
            self.position += 1;
        } else {
            return Err("Expected ')' to close function parameters".to_string());
        }

        // Expect '{'
        if self.position < self.tokens.len() && self.tokens[self.position] == Token::LeftBrace {
            self.position += 1;
        } else {
            return Err("Expected '{' to open function body".to_string());
        }

        // Parse statements inside the function body until '}'
        let mut body = Vec::new();
        while self.position < self.tokens.len() {
            if self.tokens[self.position] == Token::RightBrace {
                self.position += 1; // consume '}'
                break;
            }
            body.push(self.parse_statement()?);
        }

        let end_pos = self.position;

        Ok(Function {
            name,
            parameters: vec![],
            return_type: Type::Void,
            body,
            span: Span::new(start_pos, end_pos),
        })
    }

    /// Parses individual statements inside blocks or functions
    fn parse_statement(&mut self) -> Result<Statement, String> {
        if self.position >= self.tokens.len() {
            return Err("Unexpected end of file while parsing statement".to_string());
        }

        // Handle 'let' or 'let mut' variable declarations
        if self.tokens[self.position] == Token::Let {
            self.position += 1; // consume 'let'

           let mutable = if self.position < self.tokens.len() && self.tokens[self.position] == Token::Mut {
    self.position += 1; // consume 'mut'
    true
} else {
    false
};

            let name = match self.tokens.get(self.position) {
                Some(Token::Identifier(n)) => {
                    self.position += 1;
                    n.clone()
                }
                _ => return Err("Expected variable name identifier after 'let'".to_string()),
            };

            // Expect '='
            if self.position < self.tokens.len() && self.tokens[self.position] == Token::Assign {
                self.position += 1; // consume '='
            } else {
                return Err("Expected '=' in variable declaration".to_string());
            }

            // Parse the assigned expression using ExprParser
            let value = ExprParser::parse(&self.tokens, &mut self.position)?;

            // Expect ';'
            if self.position < self.tokens.len() && self.tokens[self.position] == Token::Semicolon {
                self.position += 1; // consume ';'
            } else {
                return Err("Expected ';' at the end of statement".to_string());
            }

            return Ok(Statement::Let {
                name,
                mutable,
                inferred_type: None,
                value,
            });
        }

        // Otherwise, treat it as a standard expression statement
        let expr = ExprParser::parse(&self.tokens, &mut self.position)?;
        
        if self.position < self.tokens.len() && self.tokens[self.position] == Token::Semicolon {
            self.position += 1; // consume optional statement semicolon
        }

        Ok(Statement::Expression(expr))
    }
}