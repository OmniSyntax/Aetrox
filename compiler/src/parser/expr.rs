use crate::lexer::Token;
use super::ast::{Expr, BinaryOperator};

pub struct ExprParser;

impl ExprParser {
    /// Entry point for parsing expressions (handles binary operations like +, -, *, /)
    pub fn parse(tokens: &[Token], position: &mut usize) -> Result<Expr, String> {
        Self::parse_additive(tokens, position)
    }

    /// Parses additive operators (+ and -)
    fn parse_additive(tokens: &[Token], position: &mut usize) -> Result<Expr, String> {
        let mut left = Self::parse_multiplicative(tokens, position)?;

        while *position < tokens.len() {
            let op = match tokens[*position] {
                Token::Plus => BinaryOperator::Add,
                Token::Minus => BinaryOperator::Sub,
                _ => break,
            };
            *position += 1; // consume operator

            let right = Self::parse_multiplicative(tokens, position)?;
            left = Expr::BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    /// Parses multiplicative operators (* and /)
    fn parse_multiplicative(tokens: &[Token], position: &mut usize) -> Result<Expr, String> {
        let mut left = Self::parse_primary(tokens, position)?;

        while *position < tokens.len() {
            let op = match tokens[*position] {
                Token::Star => BinaryOperator::Mul,
                Token::Slash => BinaryOperator::Div,
                _ => break,
            };
            *position += 1; // consume operator

            let right = Self::parse_primary(tokens, position)?;
            left = Expr::BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    /// Parses base expressions (literals, identifiers, function calls, and parentheses)
    pub fn parse_primary(tokens: &[Token], position: &mut usize) -> Result<Expr, String> {
        if *position >= tokens.len() {
            return Err("Unexpected end of file while parsing expression".to_string());
        }

        let token = &tokens[*position];
        *position += 1;

        match token {
            Token::IntLiteral(val) => Ok(Expr::IntLiteral(*val)),
            Token::StringLiteral(val) => Ok(Expr::StringLiteral(val.clone())),
            Token::Identifier(name) => {
                // Check if it's a function call like `print(args)`
                if *position < tokens.len() && tokens[*position] == Token::LeftParen {
                    *position += 1; // consume '('
                    let mut args = Vec::new();

                    // Parse arguments until closing parenthesis
                    while *position < tokens.len() && tokens[*position] != Token::RightParen {
                        args.push(Self::parse(tokens, position)?);
                        if *position < tokens.len() && tokens[*position] == Token::Comma {
                            *position += 1; // consume comma
                        }
                    }

                    if *position < tokens.len() {
                        *position += 1; // consume ')'
                    } else {
                        return Err("Expected ')' to close function call arguments".to_string());
                    }

                    Ok(Expr::Call {
                        name: name.clone(),
                        args,
                    })
                } else {
                    Ok(Expr::Identifier(name.clone()))
                }
            }
            Token::LeftParen => {
                // Handle parenthesized expressions like `(2 + 3)`
                let expr = Self::parse(tokens, position)?;
                if *position < tokens.len() && tokens[*position] == Token::RightParen {
                    *position += 1; // consume ')'
                } else {
                    return Err("Expected ')' to close parenthesized expression".to_string());
                }
                Ok(expr)
            }
            _ => Err(format!("Unexpected token in expression: {:?}", token)),
        }
    }
}