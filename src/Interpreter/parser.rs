use crate::PygoTypes::pygo_function::Function;
use crate::PygoTypes::pygo_type::Type;
use crate::PygoTypes::pygo_token::{*,PygoToken::*};
use crate::PygoTypes::pygo_instruction::Instruction;
use crate::PygoTypes::pygo_context::Context;

use crate::StandardLib::standard_library::StdLibFn;
use crate::StandardLib::standard_library;

use hashbrown::HashMap;
use std::iter::Peekable;
use std::slice::Iter;
use std::mem;
pub struct PygoParser<'a> {
    tokens: Peekable<Iter<'a, PygoToken>>,

}

impl<'a> PygoParser<'a> {
    pub fn new(tokens: &'a Vec<PygoToken>) -> PygoParser<'a> {
        PygoParser {
            tokens: tokens.iter().peekable(),
        }
    }
	pub fn parse(&mut self, context: &mut Context){
		while let Some(token) = self.tokens.next() {
			match token {
				KEYWORD(..) => println!("key: {:?}", token),
				OPERATOR(operator) => println!("op: {:?}", token),
				LITERAL(literal) => self.literal(literal, context),
				VARIABLE_NAME(var_name) => self.variable(var_name, context),
				_ => println!("other: {:?}", token),
			}
			
		}
		
	}
	pub fn operator(&mut self, operator: &PygoOp, context: &mut Context){
		println!("op: {:?}", operator);
		match operator {
			PygoOp::ADDITION => (),
			_ => (),
			
		}
	}
	pub fn literal(&mut self, literal: &PygoLiteral, context: &mut Context){
		println!("lit: {:?}", literal);
		match literal {
			PygoLiteral::INTEGER_LITERAL(val) => (),
			PygoLiteral::FLOATING_POINT_LITERAL(val) => (),
			_ => (),
			
		}
	}
	pub fn variable(&mut self, var_name: &String, context: &mut Context){
		println!("var_name: {:?}", var_name);
		if let Some(next_token) = self.tokens.peek(){
			match next_token {
				
			}
		}
		context.instruction.append(Instruction::SetVar(var_name, ()))
	}
}

