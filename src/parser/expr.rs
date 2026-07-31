use crate::lexer::Token;
use super::ast::Expr;

pub struct ExprParser;

impl ExprParser {
    /// Parses a basic expression from the token stream
    pub fn parse_primary(tokens: &[Token], position: &mut usize) -> Result<Expr, String> {
        if *position >= tokens.len() {
            return Err("Unexpected end of file while parsing expression".to_string());
        }

        let token = &tokens[*position];
        *position += 1;

        match token {
            Token::IntLiteral(val) => Ok(Expr::IntLiteral(*val)),
            Token::Identifier(name) => {
                // Check if it's a function call like `print(...)`
                if *position < tokens.len() && tokens[*position] == Token::LeftParen {
                    *position += 1; // consume '('
                    let mut args = Vec::new();
                    
                    // Parse arguments until closing parenthesis
                    while *position < tokens.len() && tokens[*position] != Token::RightParen {
                        args.push(Self::parse_primary(tokens, position)?);
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
            _ => Err(format!("Unexpected token in expression: {:?}", token)),
        }
    }
}