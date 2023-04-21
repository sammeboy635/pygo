#![allow(warnings)]
#![allow(unused_imports)]

mod lexer;
mod parser;

mod interpreter;
mod timer;

use crate::lexer::lex;
use crate::parser::{Parser, parse_expression};
use crate::interpreter::interpret;
use crate::timer::Timer;

use std::time::{Instant};
use std::collections::HashMap;
use evalexpr::*;
//Lexing: 874 us
//Parsing: 1279 us
fn main() {
	let operaters = vec!["+", "-", "*", "/", "%",":","(",")"];
	let input = "test2 = 3 + (9 * (5 + 8)) * 3 / 5".to_string();

	let mut timer = Timer::new();
	let tokens = lex(&input, &operaters);
	timer.end("Lexing");
	println!("{:?}", tokens);

	let mut vari: HashMap<String, interpreter::Type> = HashMap::new();
	let mut parser = Parser::new(tokens);
	let mut instructions = vec![];
	parse_expression(&mut parser, &mut instructions);
	timer.end("Parsing");

	println!("interpreting: {:?}", interpret(&instructions, &mut vari));
	timer.end("Interpreting");
	

	timer.start();
	let expression = "3 + (9 * (5 + 8)) * 3 / 5";
    match eval(expression) {
        Ok(result) => println!("The result of the expression is: {}", result),
        Err(error) => println!("Error: {}", error),
    }
	timer.end("Evaluating");

	println!("{:?}", vari);
}

