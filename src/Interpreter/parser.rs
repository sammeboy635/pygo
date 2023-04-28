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
			match token {
				ASSIGNMENT => (),
				VARIABLE_NAME(var_name) => context.instruction.push(self.variable(&var_name)),
				VARIABLE_NAME_TYPE(var_name, var_type) => context.instruction.push(self.variable_type(var_name, var_type)),
				STRING_LITERAL(value) => context.instruction.push(Instruction::Push(Type::String(value.to_owned()))),
				INTEGER_LITERAL(value) => context.instruction.push(Instruction::Push(Type::Int(*value as i64))),
				FLOATING_POINT_LITERAL(value) => context.instruction.push(Instruction::Push(Type::Float(value.0 as f64))),
				BOOLEAN_LITERAL(value) => context.instruction.push(Instruction::Push(Type::Bool(*value))),
				NONE_LITERAL => (),
				_ if token.is_keyword() => (),
				_ if token.is_op() => self.operator(&token, context),
				_ if token.is_literal() => (),
				_ => println!("other: {:?}", token),
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

	pub fn operator(&self, operator: &PygoToken, context: &mut Context){
		println!("op: {:?}", operator);
		match operator {
			PygoToken::ADDITION => context.instruction.push(Instruction::Add),
			PygoToken::SUBTRACTION => context.instruction.push(Instruction::Sub),
			PygoToken::MULTIPLICATION => context.instruction.push(Instruction::Mul),
			PygoToken::DIVISION => context.instruction.push(Instruction::Div),
			PygoToken::MODULO => context.instruction.push(Instruction::Modulo),
			PygoToken::EXPONENT => context.instruction.push(Instruction::Exp),
			_ => (),
			
		}
	}
	pub fn literal(&self, literal: &PygoToken, context: &mut Context){
		println!("lit: {:?}", literal);
		match literal {
			PygoToken::INTEGER_LITERAL(val) => (),
			PygoToken::FLOATING_POINT_LITERAL(val) => (),
			_ => (),
			
		}
	}
	pub fn variable(&self, var_name: &String)-> Instruction {
		println!("var_name: {:?}", var_name);
		return Instruction::SetVar(var_name.to_owned(), Type::Unknown);
	}
	pub fn variable_type(&self, var_name: &String, var_type: &String) -> Instruction{
		println!("var_name{:?}, type {:?}",var_name,var_type);
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

