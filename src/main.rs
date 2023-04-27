#![allow(warnings)]
#![allow(unused_imports)]

mod test;
mod Interpreter;
use crate::Interpreter::lexer::{Tokenizer, load_file, print_tokens};
use crate::Interpreter::parser::PygoParser;
use crate::Interpreter::interpreter::Interpret;

mod PygoTypes;
use crate::PygoTypes::pygo_type::Type;
use crate::PygoTypes::pygo_context::Context;

mod StandardLib;
use crate::StandardLib::standard_library::standard_library;

mod Utils;
use crate::Utils::timer::Timer;


use std::time::{Instant};
use std::collections::HashMap;
use evalexpr::*;


use std::collections::VecDeque;

/* In the main function, we are opening a file, reading its contents,
   initializing a hashmap for context variables, and passing the contents and 
   hashmap to the evaluate function for further processing. */
fn main() {
	//test::main2();
	let binding = load_file("tmp/assignment/test_2.py");
    let code = binding.as_str();

    let mut tokenizer = Tokenizer::new(code);
	let mut timer = Timer::new();
    let tokens = tokenizer.tokenize();
	timer.end_us("Tokenizer");

	
	
	print_tokens(&tokens);
	//timer.end_us("Print Tokens");
	let mut parser = PygoParser::new(&tokens);
	let mut context = Context::new();
	parser.parse(&mut context);
	timer.print("Parser");
}
/* The evaluate function takes a string input and a mutable reference to a hashmap of context variables.
 	It first initializes a standard library and a timer, 
 	The lexer takes the input string to obtain tokens, and passes the tokens to the parser. 
  	The parser generates a vector of instructions that are then passed to the interpreter.
	The interpreter then interprets the instructions while also updating the context variables hashmap. 
	
	Finally, the function prints the context variables hashmap 
	and the time taken for each step in the processing pipeline. */
fn evaluate(context_varibles: &mut HashMap<String, Type>){
	let input = load_file("tmp/main.py");
	
	let mut timer = Timer::new();
	let sl = standard_library();
	

	// let tokens = lex(&input);
	// timer.end_us("Lexing");

	// println!("{:?}", tokens);
	
	// let mut parser = Parser::new(tokens, sl);
	// let mut instructions = vec![];
	// parse_expression(&mut parser, &mut instructions);
	// timer.end_us("Parsing");

	// println!("{:?}", instructions);
	// let mut deque = VecDeque::new();
	// deque.extend(instructions);
	// let mut interp = Interpret::new(&mut deque);
	// let out = interp.interpret(context_varibles);
	// timer.end_us("Interpreting");

	// timer.print(&format!("{:?}", out).to_string());
	// println!();

}

