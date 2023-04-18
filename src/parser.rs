use crate::ast::{BinaryOp, Expr, Literal};
use crate::lexer::{Token, TokenType};
use TokenType::*;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    pub fn parse(&mut self) -> Option<Expr> {
        self.parse_expr()
    }

    fn parse_expr(&mut self) -> Option<Expr> {
        let left = self.parse_term()?;
        self.parse_expr_rest(left)
    }

    fn parse_expr_rest(&mut self, left: Expr) -> Option<Expr> {
        if let Some(token) = self.peek() {
            match token.token_type {
                Plus | Minus => {
                    let op = match token.token_type {
                        Plus => BinaryOp::Plus,
                        Minus => BinaryOp::Minus,
                        _ => unreachable!(),
                    };
                    self.advance();
                    let right = self.parse_term()?;
                    let expr = Expr::BinaryOp {
                        op,
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                    self.parse_expr_rest(expr)
                }
                _ => Some(left),
            }
        } else {
            Some(left)
        }
    }

    fn parse_term(&mut self) -> Option<Expr> {
        let left = self.parse_factor()?;
        self.parse_term_rest(left)
    }

    fn parse_term_rest(&mut self, left: Expr) -> Option<Expr> {
        if let Some(token) = self.peek() {
            match token.token_type {
                Multiply | Divide => {
                    let op = match token.token_type {
                        Multiply => BinaryOp::Multiply,
                        Divide => BinaryOp::Divide,
                        _ => unreachable!(),
                    };
                    self.advance();
                    let right = self.parse_factor()?;
                    let expr = Expr::BinaryOp {
                        op,
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                    self.parse_term_rest(expr)
                }
                _ => Some(left),
            }
        } else {
            Some(left)
        }
    }

    fn parse_factor(&mut self) -> Option<Expr> {
		if let Some(token) = self.peek() {
			let token_type = token.token_type.clone(); // Save the token_type
			let token_value = token.value.clone(); // Save the token value
			match token_type {
				Integer => {
					self.advance();
					token_value.parse::<i32>().ok().map(Literal::Integer).map(Expr::Literal)
				}
				Float => {
					self.advance();
					token_value.parse::<f64>().ok().map(Literal::Float).map(Expr::Literal)
				}
				String => {
					self.advance();
					let value = token_value[1..token_value.len() - 1].to_string();
					Some(Expr::Literal(Literal::String(value)))
				}
				Identifier => {
					self.advance();
					Some(Expr::Identifier(token_value))
				}
				_ => None,
			}
		} else {
			None
		}
	}

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

}

 pub fn test_parser(tokens: Vec<Token>) {
	let mut parser = Parser::new(tokens);
    let ast = parser.parse();
    println!("{:?}", ast);
 }