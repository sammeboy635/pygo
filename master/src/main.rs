#![allow(warnings)]
#![allow(unused_imports)]
use lexer::{tokenizer, pre_parser::pre_parser};
use utils::{timer::Timer};
use ast::{context::Context, token::PygoToken, token::PygoTokenVec};
use parser::PygoParser;
use interpreter::Interpret;
use std::cell::RefCell;
fn main() {
    //lexer::tokenizer::main();

	let mut context = Context::new();
	let data = RefCell::new(context);
	let mut tokens: Vec<PygoToken> = vec![];
	
	let content = tokenizer::load_file("tmp\\assignment\\test_1.py");
	let mut tok = tokenizer::Tokenizer::new(content.as_str());
	let mut tokens = vec![];
	let mut timer = Timer::new();

	tok.tokenize(&mut tokens);
	timer.elapse("Tokenizer");

	let mut pre_tokens = pre_parser(&mut tokens).unwrap();
	timer.elapse("Preparser");

	let mut parser = PygoParser::new(&mut pre_tokens);
	parser.parse(&mut data.borrow_mut());
	timer.elapse("Parser");

	let mut interpreter = Interpret::new();
	interpreter.interpret(&mut data.borrow_mut());
	timer.elapse("Interpreter");

	println!("\nInstructions: \n{:?}\n", data.borrow().instruction);
	pre_tokens._debug_print();
	println!("\n{:?}\n", data.borrow().variables);
	timer.print();

}
