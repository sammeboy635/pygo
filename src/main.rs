#![allow(warnings)]
#![allow(unused_imports)]


mod Interpreter;
use crate::Interpreter::lexer::lex;
use crate::Interpreter::parser::{Parser, parse_expression};
use crate::Interpreter::interpreter::interpret;

mod PygoTypes;
use crate::PygoTypes::pygo_type::Type;

mod StandardLib;
use crate::StandardLib::standard_library::standard_library;

mod Utils;
use crate::Utils::timer::Timer;


use std::time::{Instant};
use std::collections::HashMap;
use evalexpr::*;
use std::fs::File;
use std::io::prelude::*;


/* In the main function, we are opening a file, reading its contents,
   initializing a hashmap for context variables, and passing the contents and 
   hashmap to the evaluate function for further processing. */
fn main() {
	let mut file = File::open("tmp/main.py").expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");

 	let mut context_varibles: HashMap<String, Type> = HashMap::new();

	evaluate(contents, &mut context_varibles);
	println!("{:?}", context_varibles);
}
/* The evaluate function takes a string input and a mutable reference to a hashmap of context variables.
 	It first initializes a standard library and a timer, 
 	The lexer takes the input string to obtain tokens, and passes the tokens to the parser. 
  	The parser generates a vector of instructions that are then passed to the interpreter.
	The interpreter then interprets the instructions while also updating the context variables hashmap. 
	
	Finally, the function prints the context variables hashmap 
	and the time taken for each step in the processing pipeline. */
fn evaluate(input : String, context_varibles: &mut HashMap<String, Type>){
	
	let sl = standard_library();
	let mut timer = Timer::new();

	let tokens = lex(&input);
	timer.end_us("Lexing");

	println!("{:?}", tokens);
	
	let mut parser = Parser::new(tokens, sl);
	let mut instructions = vec![];
	parse_expression(&mut parser, &mut instructions);
	timer.end_us("Parsing");
	println!("{:?}", instructions);
	let mut index = 0;
	let interp = interpret(&instructions, context_varibles, &parser.custom, &mut index);
	//println!("interpreting: {:?}", );
	timer.end_us("Interpreting");

	timer.print(&format!("{:?}", interp).to_string());
	println!();

}

