#![allow(warnings)]
#![allow(unused_imports)]

mod test;
mod Interpreter;
use crate::Interpreter::lexer::{Tokenizer, load_file};
use crate::Interpreter::parser::PygoParser;
use crate::Interpreter::interpreter::{Interpret, self};

mod PygoTypes;
use crate::PygoTypes::pygo_type::Type;
use crate::PygoTypes::pygo_context::Context;
use crate::PygoTypes::pygo_context::FunctionDefinition;
use crate::PygoTypes::pygo_token::{*,PygoToken::*};

mod StandardLib;
use crate::StandardLib::standard_library::standard_library;

mod Traits;
use crate::Traits::pygo_token::{PygoTokenVec};

mod Utils;
use crate::Utils::timer::Timer;


use std::time::{Instant};
use std::collections::HashMap;
use std::vec;
use PygoTypes::pygo_instruction::Instruction;
use evalexpr::*;
use nom::Parser;
use std::cell::RefCell;

use std::collections::VecDeque;


use std::iter::Peekable;
use std::slice::Iter;

fn pre_parser<'a>(tokens :&mut Peekable<Iter<'a, PygoToken>>) -> Option<Vec<PygoToken>> {
    let mut output: Vec<PygoToken> = Vec::new();
    let mut operators: Vec<PygoToken> = Vec::new();
    let mut temp_variable: Option<PygoToken> = None;
	let mut func_paren_count: usize = 0;
	let mut inside_paren_count: usize = 0;
    while let Some(token) = tokens.next() {
        match token {
			FUNCTION_NAME(_) => {
				output.push(token.clone());
				func_paren_count += 1;
			}
            INTEGER_LITERAL(_) | FLOATING_POINT_LITERAL(_) | STRING_LITERAL(_) | BOOLEAN_LITERAL(_) => {
                output.push(token.clone());
            }
            OPEN_PAREN => {
                if func_paren_count > 0 {
                    func_paren_count -= 1;
                    output.push(token.clone());
                } else {
					inside_paren_count += 1;
                    operators.push(token.clone());
                }
            }
            CLOSED_PAREN | COMMA | END => {
				while let Some(top_op) = operators.pop() {
                    if OPEN_PAREN == top_op {
                        break;
                    }
                    output.push(top_op);
                }
				if (*token == CLOSED_PAREN && inside_paren_count > 0) {
					inside_paren_count -= 1;
					continue;
				}
                output.push(token.clone());
            }
            _ if token.is_op() || token.is_var() => {
                while let Some(top_op) = operators.last() {
                    if let OPEN_PAREN = top_op {
                        break;
                    }
                    if token.precedence() > top_op.precedence() {
                        break;
                    }
                    output.push(operators.pop().unwrap());
                }
                operators.push(token.clone());
            }
            _ => output.push(token.clone()),
        }
    }

    while let Some(op) = operators.pop() {
        output.push(op);
    }
    if output.is_empty() {
        return None;
    }

    Some(output)
}

fn extract_function_definitions(tokens: &mut Vec<PygoToken>, context: &mut Context) {
    let mut tokens_iter = tokens.iter().peekable();

    let mut to_remove: Vec<usize> = Vec::new();
    let mut index = 0;

    while let Some(token) = tokens_iter.next() {
        if let PygoToken::DEF = token {
            if let Some(PygoToken::FUNCTION_NAME(name)) = tokens_iter.next() {
                let mut args = Vec::new();
                let mut instructions = Vec::new();
                let mut returns = None;

                to_remove.push(index - 1);
                to_remove.push(index);

                if let Some(PygoToken::OPEN_PAREN) = tokens_iter.next() {
                    to_remove.push(index + 1);
                    while let Some(token) = tokens_iter.peek() {
                        match token {
                            PygoToken::VARIABLE_NAME(arg_name) => {
                                args.push(arg_name.clone());
                                tokens_iter.next();
                                to_remove.push(index + 2);
                            }
                            PygoToken::COMMA => {
                                tokens_iter.next();
                                to_remove.push(index + 2);
                            }
                            PygoToken::CLOSED_PAREN => {
                                tokens_iter.next();
                                to_remove.push(index + 2);
                                break;
                            }
                            _ => panic!("Unexpected token in function arguments"),
                        }
                        index += 1;
                    }
                }

                if let Some(PygoToken::COLON) = tokens_iter.next() {
                    to_remove.push(index + 1);
                    while let Some(token) = tokens_iter.peek() {
                        index += 1;
                        match token {
                            PygoToken::RETURN => {
								while let Some(tok) = tokens_iter.next(){

								}
                                tokens_iter.next(); // Consume RETURN token
                                to_remove.push(index);
                                returns = Some(Type::Void); // Default return type is Void
                                break;
                            }
                            _ => {
                                // Extract instructions
                                // if let Some(instr) = Instruction::from_pygo_token(token) {
                                //     instructions.push(instr);
                                // }
                                tokens_iter.next();
                                to_remove.push(index);
                            }
                        }
                    }
                }

                let function_definition = FunctionDefinition {
                    name: name.clone(),
                    args,
                    instructions,
                    returns,
                };

                context.add_function_definition(function_definition);
            }
        }
        index += 1;
    }

    // Remove parsed tokens
    to_remove.reverse();
    for idx in to_remove {
        tokens.remove(idx);
    }
}



/* In the main function, we are opening a file, reading its contents,
   initializing a hashmap for context variables, and passing the contents and 
   hashmap to the evaluate function for further processing. */
fn main() {
	let mut context = Context::new();
	let data = RefCell::new(context);
	let mut tokens: Vec<PygoToken> = vec![];
	
	//let &mut tokens: Vec<PygoToken> = vec![]._load_file("tmp/assignment/test_2.py");
	//test::main2();
	let mut timer = Timer::new();
	tokens._load_file("tmp/assignment/test_2.py");
	timer.elapse("Tokenizer");

	let mut pre_tokens = tokens.pre_parser().unwrap();
	timer.elapse("Preparser");
	
	// Printing tokenizer and preparser
	tokens._debug_print();
	pre_tokens._debug_print();


	//Function definitions
	//extract_function_definitions(&mut pre_tokens,&mut data.borrow_mut());
	//println!("{:?}", data.borrow().functions);


	timer.new_start();
	let mut parser = PygoParser::new(&mut pre_tokens);
	parser.parse();
	timer.elapse("Parser");

	println!("\n{:?}\n", data.borrow().instruction);


	timer.new_start();
	let mut interpreter = Interpret::new();
	interpreter.interpret(&mut data.borrow_mut());
	timer.elapse("Interpreter");

	println!("\n{:?}\n", data.borrow().variables);
	timer.print();
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

