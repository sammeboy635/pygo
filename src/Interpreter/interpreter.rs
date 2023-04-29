use crate::PygoTypes::pygo_context::Context;
use crate::PygoTypes::pygo_type::Type;
use crate::PygoTypes::pygo_function::Function;
use crate::PygoTypes::pygo_instruction::{Instruction, Instruction::*};


use std::collections::HashMap;
use std::collections::VecDeque;

use std::iter::Peekable;
use std::slice::Iter;

pub struct Interpret {
    global_variables: HashMap<String, Type>,
	std_lib_functions: HashMap<String, Function>,
    functions: HashMap<String, Function>,
	stack : Vec<Type>,
	index : isize,
}

impl Interpret{
    pub fn new() -> Self {
        Interpret {
            global_variables: HashMap::new(),
            std_lib_functions: HashMap::new(),
            functions: HashMap::new(),
			stack: Vec::new(),
			index: 0,
        }
    }
	pub fn interpret(&mut self, context: &mut Context) -> Type{
		let mut code = context.instruction.iter().peekable();
		while let Some(instruction) = code.next(){
			match instruction {
				Add | Sub | Mul | Div | Exp | Modulo => self.operations(&instruction),
				Load(_name, _type) => self.stack.push(context.variables.get(_name).unwrap().clone()),
				Push(val) => self.stack.push(val.clone()),
				SetVar(_name, _) => {context.variables.insert(_name.clone(), self.stack.pop().unwrap());},
				End => continue,
				_ => todo!(),
			}
		}
		if self.stack.len() == 0 {return Type::Void;}
		return self.stack.pop().unwrap();
	}
	pub fn operations(&mut self, instruction : &Instruction){
		let right = self.stack.pop().unwrap();
		let left = self.stack.pop().unwrap();
		let val = match instruction {
			Add => left + right,
			Sub => left - right,
			Mul => left * right,
			Div => left / right,
			Exp => left.exp(&right),
			Modulo => left % right,
			_ => panic!("Unsupported operation"),
		};
		self.stack.push(val);
	}
}





// #[test]
// fn test_int(){
// 	use crate::standard_library;
// 	fn execute_instruction(instruction: &Instruction) {
// 		match instruction {
// 			Instruction::Call(_, args, func) => {
// 				func.call(args.clone());
// 			}
// 			_ => println!("Unhandled instruction"),
// 		}
// 	}
// 	let print = Instruction::Call(
//         "print".to_string(),
// 		Type::String("Hello_world".to_string()),
// 		MyFunc::new(standard_library::print),
//     );
// 	// Call the print instruction
// 	execute_instruction(&print);
	
// }