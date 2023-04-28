#![allow(warnings)]
#![allow(unused_imports)]

mod test;
mod Interpreter;
use crate::Interpreter::lexer::{Tokenizer, load_file, print_tokens};
use crate::Interpreter::parser::PygoParser;
use crate::Interpreter::interpreter::{Interpret, self};

mod PygoTypes;
use crate::PygoTypes::pygo_type::Type;
use crate::PygoTypes::pygo_context::Context;

mod StandardLib;
use crate::StandardLib::standard_library::standard_library;

mod Utils;
use crate::Utils::timer::Timer;


use std::time::{Instant};
use std::collections::HashMap;
use PygoTypes::pygo_instruction::Instruction;
use evalexpr::*;
use std::cell::RefCell;

use std::collections::VecDeque;

use std::iter::Peekable;
use std::slice::Iter;
use crate::PygoTypes::pygo_token::{*,PygoToken::*};

// fn to_postfix<'a>(tokens :&mut Peekable<Iter<'a, PygoToken>>) -> Option<Vec<PygoToken>> {
//     let mut output: Vec<PygoToken> = Vec::new();
//     let mut operators: Vec<PygoToken> = Vec::new();
//     let mut func_paren_count: usize = 0;

//     while let Some(token) = tokens.next() {
//         match token {
//             FUNCTION_NAME(_) => {
//                 output.push(token.clone());
//                 func_paren_count += 1;
//             }
//             OPEN_PAREN => {
//                 if func_paren_count > 0 {
//                     func_paren_count -= 1;
//                     output.push(token.clone());
//                 } else {
//                     operators.push(token.clone());
//                 }
//             }
//             CLOSED_PAREN | COMMA => {
//                 while let Some(top_op) = operators.pop() {
//                     if let OPEN_PAREN = top_op {
//                         break;
//                     }
//                     output.push(top_op);
//                 }
//                 if *token == CLOSED_PAREN && func_paren_count > 0 {
//                     output.push(token.clone());
//                     func_paren_count += 1;
//                 } else if *token == COMMA {
//                     output.push(token.clone());
//                 }
//             }
//             END => {
//                 break;
//             }
//             ASSIGNMENT | VARIABLE_NAME(_) => {
//                 operators.push(token.clone());
//             }
//             _ if token.is_op() || token.is_var() => {
//                 while let Some(top_op) = operators.last() {
//                     if let OPEN_PAREN = top_op {
//                         break;
//                     }
//                     if token.precedence() > top_op.precedence() {
//                         break;
//                     }
//                     output.push(operators.pop().unwrap());
//                 }
//                 operators.push(token.clone());
//             }
//             _ => output.push(token.clone()),
//         }
//     }

//     while let Some(op) = operators.pop() {
//         output.push(op);
//     }
//     if output.is_empty() {
//         return None;
//     }

//     Some(output)
// }
fn to_postfix<'a>(tokens :&mut Peekable<Iter<'a, PygoToken>>) -> Option<Vec<PygoToken>> {
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

/* In the main function, we are opening a file, reading its contents,
   initializing a hashmap for context variables, and passing the contents and 
   hashmap to the evaluate function for further processing. */
fn main() {
	let mut context = Context::new();
	let data = RefCell::new(context);

	//test::main2();
	let binding = load_file("tmp/assignment/test_2.py");
    let code = binding.as_str();

    let mut tokenizer = Tokenizer::new(code);
	let mut timer = Timer::new();
    let tokens = tokenizer.tokenize();
	timer.end_us("Tokenizer");

	
	
	print_tokens(&tokens);
	let mut tok = tokens.iter().peekable();
	let mut new_token = to_postfix(&mut tok);
	timer.end_us("post");
	print_tokens(&new_token.unwrap());
	//timer.end_us("Print Tokens");
	// let mut parser = PygoParser::new(&tokens);
	
	// parser.parse(&mut data.borrow_mut());
	
	// timer.end_us("Parser");
	

	// let mut deque = VecDeque::new();
	// deque.extend(data.borrow().instruction.clone());
	// let mut interp = Interpret::new(&mut deque);
	// let out = interp.interpret(&mut data.borrow_mut());
	timer.print("Interpret");


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

