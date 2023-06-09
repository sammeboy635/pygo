use ast::context::Context;
use ast::types::{Type,Float};
use ast::token::{PygoToken, PygoToken::*};

use ast::instruction::{Instruction};



use std::iter::Peekable;
use std::slice::Iter;
use std::{vec};
pub struct PygoParser<'a> {
    tokens: Peekable<Iter<'a, PygoToken>>,
	//context: Context,
}

impl <'a>PygoParser<'a> {
    pub fn new(tokens: &'a Vec<PygoToken>) -> PygoParser {
        PygoParser {
            tokens: tokens.iter().peekable(),
			//context: Context::new(),
        }
    }

	pub fn parse(&mut self,  context: &mut Context){
		while let Some(token) = self.tokens.next() {
		//println!("{:?}", context.instruction);
			let instruction = match token {
				ASSIGNMENT => None,
				VARIABLE_NAME(var_name) => Some(Instruction::Load(var_name.to_owned(), Type::Unknown)),
				VARIABLE_NAME_TYPE(var_name, var_type) => Some(self.variable_type(&var_name, &var_type)),
				VARIABLE_NAME_ASSIGNMENT(var_name) => Some(Instruction::SetVar(var_name.to_owned(), Type::Unknown)),
				VARIABLE_NAME_ASSIGNMENT_TYPE(var_name, var_type) => Some(self.variable_type(&var_name, &var_type)),
				LITERAL(val) => Some(Instruction::Push(val.clone())),
				END => Some(Instruction::End),
				_ if token.is_keyword() => self.keyword(&token),
				_ if token.is_op() => self.operator(&token),
				_ if token.is_literal() => None,
				_ => {//println!("other: {:?}", token); 
				None},
			};
			if let Some(cur_instruction) = instruction{
				context.instruction.push(cur_instruction);
			}
		}
		
	}
	pub fn keyword(&mut self, keyword: &PygoToken)-> Option<Instruction>{
		if *keyword == DEF {
			let func_name = self.tokens.next().expect("No Function Token");
			self.tokens.next_if(|&x| x == &OPEN_PAREN).expect("error");
			let mut index = 0;
			let mut args: Vec<Vec<PygoToken>> = vec![vec![]];
			while let Some(val) = self.tokens.next(){
				match val {
					COMMA => {args.push(vec![]);index += 1},
					CLOSED_PAREN => break,
					_ => args[index].push(val.clone()),
				}
			}
			println!("{:?}", args);
		}
		return None;
	}
	pub fn operator(&self, operator: &PygoToken)-> Option<Instruction>{
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

	pub fn literal(&self, literal: &PygoToken){
		//println!("lit: {:?}", literal);
		match literal {
			PygoToken::INTEGER_LITERAL(val) => (),
			PygoToken::FLOATING_POINT_LITERAL(val) => (),
			_ => (),
			
		}
	}

	pub fn variable_type(&self, var_name: &String, var_type: &String) -> Instruction{
		//println!("var_name{:?}, type {:?}",var_name,var_type);
		let _type = match var_type.as_str() { // Change this to be not hard codded maybe change type to have option
			"int" => Type::Int(0),
			"float" => Type::Float(Float(0.0)),
			"string" => Type::String("".to_string()),
			"double" => Type::Double(Float(0.0)),
			_ => Type::Void,
		};
		return Instruction::SetVar(var_name.to_owned(), _type);
	}

}

