use crate::PygoTypes::pygo_type::Type;
use crate::PygoTypes::pygo_function::Function;
use crate::PygoTypes::pygo_instruction::{Instruction, Instruction::*};


use std::collections::HashMap;
use std::collections::VecDeque;
pub struct Interpret<'a> {
    global_variables: HashMap<String, Type>,
	std_lib_functions: HashMap<String, Function>,
    functions: HashMap<String, Function>,
	stack : Vec<Type>,
	code : &'a mut VecDeque<Instruction>,
	index : isize,
}

impl<'a> Interpret<'a>{
    pub fn new(code: &'a mut VecDeque<Instruction>) -> Self {
        Interpret {
            global_variables: HashMap::new(),
            std_lib_functions: HashMap::new(),
            functions: HashMap::new(),
			stack: Vec::new(),
            code,
			index: 0,
        }
    }
	pub fn interpret(&mut self, variables: &mut HashMap<String, Type>) -> Type{
		while let Some(instruction) = self.code.pop_front(){
			match instruction {
				Add | Sub | Mul | Div | Exp | Modulo => self.operations(&instruction),
				Call(_, args, func) => {func.call(args.clone());},
				CustomCall(func,_,_) =>  {
					let new_code = self.functions.get(&func).unwrap();
					self.interpret(variables);
				},
				Load(name, _) => self.stack.push(variables.get(&name).unwrap().clone()),
				Push(val) => self.stack.push(val.clone()),
				SetVar(name, _) => {
					let test = name.clone();
					let vall = self.interpret(variables);
					println!("{:?}, {:?}",test,vall);
					variables.insert(test, vall);
				},
				End => return self.stack.pop().unwrap(),
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