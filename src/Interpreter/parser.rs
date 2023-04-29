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

	pub fn parse(&mut self,context: &mut Context){
		while let Some(token) = self.tokens.next() {
		//println!("{:?}", context.instruction);
			let instruction = match token {
				ASSIGNMENT => None,
				VARIABLE_NAME(var_name) => Some(Instruction::Load(var_name.to_owned(), Type::Unknown)),
				VARIABLE_NAME_TYPE(var_name, var_type) => Some(self.variable_type(var_name, var_type)),
				VARIABLE_NAME_ASSIGNMENT(var_name) => Some(Instruction::SetVar(var_name.to_owned(), Type::Unknown)),
				VARIABLE_NAME_ASSIGNMENT_TYPE(var_name, var_type) => Some(self.variable_type(var_name, var_type)),
				STRING_LITERAL(value) => Some(Instruction::Push(Type::String(value.to_owned()))),
				INTEGER_LITERAL(value) => Some(Instruction::Push(Type::Int(*value as i64))),
				FLOATING_POINT_LITERAL(value) => Some(Instruction::Push(Type::Float(value.0 as f64))),
				BOOLEAN_LITERAL(value) => Some(Instruction::Push(Type::Bool(*value))),
				NONE_LITERAL => None,
				END => Some(Instruction::End),
				_ if token.is_keyword() => None,
				_ if token.is_op() => self.operator(&token, context),
				_ if token.is_literal() => None,
				_ => {//println!("other: {:?}", token); 
				None},
			};
			if let Some(cur_instruction) = instruction{
				context.instruction.push(cur_instruction);
			}
		}
		
	}
	fn to_postfix(&mut self) -> Option<Vec<PygoToken>> {
		let mut output: Vec<PygoToken> = Vec::new();
		let mut operators: Vec<PygoToken> = Vec::new();
	
		while let Some(token) = self.tokens.next() {
			match token {
				INTEGER_LITERAL(_) | FLOATING_POINT_LITERAL(_) | STRING_LITERAL(_) | BOOLEAN_LITERAL(_) => {
					output.push(token.clone());
				}
				OPEN_PAREN => {
					operators.push(token.clone());
				}
				CLOSED_PAREN | COMMA => {
					while let Some(top_op) = operators.pop() {
						if let OPEN_PAREN = top_op {
							break;
						}
						output.push(top_op);
					}
				}
				END => {
					break;
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
		if output.is_empty(){
			return None;
		}

		Some(output)
	}

	pub fn operator(&self, operator: &PygoToken, context: &mut Context)-> Option<Instruction>{
		//println!("op: {:?}", operator);
		match operator {
			PygoToken::ADDITION => Some(Instruction::Add),
			PygoToken::SUBTRACTION => Some(Instruction::Sub),
			PygoToken::MULTIPLICATION => Some(Instruction::Mul),
			PygoToken::DIVISION => Some(Instruction::Div),
			PygoToken::MODULO => Some(Instruction::Modulo),
			PygoToken::EXPONENT => Some(Instruction::Exp),
			_ => None,	
		}
	}
	pub fn literal(&self, literal: &PygoToken, context: &mut Context){
		//println!("lit: {:?}", literal);
		match literal {
			PygoToken::INTEGER_LITERAL(val) => (),
			PygoToken::FLOATING_POINT_LITERAL(val) => (),
			_ => (),
			
		}
	}
	pub fn variable(&self, var_name: &String)-> Instruction {
		//println!("var_name: {:?}", var_name);
		return Instruction::SetVar(var_name.to_owned(), Type::Unknown);
	}
	pub fn variable_type(&self, var_name: &String, var_type: &String) -> Instruction{
		//println!("var_name{:?}, type {:?}",var_name,var_type);
		let _type = match var_type.as_str() { // Change this to be not hard codded maybe change type to have option
			"int" => Type::Int(0),
			"float" => Type::Float(0.0),
			"string" => Type::String("".to_string()),
			"double" => Type::Double(0.0),
			_ => Type::Void,
		};
		return Instruction::SetVar(var_name.to_owned(), _type);
	}
}

