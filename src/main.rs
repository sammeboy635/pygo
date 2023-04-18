

mod lexer;
mod parser;
mod ast;
mod codegen;
mod interpreter;

#[allow(unused_imports)]
use lexer::{Lexer, Token, TokenType,  test_lexer};
use parser::{Parser};
use codegen::{Type, generate_code};
use interpreter::{interpret};
use std::collections::HashMap;


fn main() {
	//test_lexer();
	//let token = Lexer::new("x + 1 + 2 * 3 + 2").lex();
	let token = Lexer::new("5 + 10").lex();
	println!("{:?}", token);
	//test_parser(token);
	let mut parser = Parser::new(token.clone());
	let ast = parser.parse().unwrap();
	println!("{:?}", ast);
	let variables = vec![
        ("x".to_string(), Type::Integer),
        ("y".to_string(), Type::Float),
    ]
    .into_iter()
    .collect();

	let instructions =  generate_code(&ast, &variables);
	println!("{:?}", instructions);

    let variabless: HashMap<String, f64> = vec![
        ("x".to_string(), 10.0),
        ("y".to_string(), 5.0),
    ]
    .into_iter()
    .collect();

    let result = interpret(&instructions, &variabless);
    println!("Result: {}", result);
}