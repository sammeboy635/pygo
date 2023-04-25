
use crate::ast::{Instruction, Type};
use crate::ast::Instruction::*;

use std::{collections::HashMap};


pub fn interpret(code: &[Instruction], variables: &mut HashMap<String, Type>, custom: &HashMap<String, Vec<Instruction>>, index: &mut isize) -> Type {
	println!("{:?}",code);
	if code.len() == 0{return Type::Void;}
    let mut stack: Vec<Type> = Vec::new();
	while let Some(instruction) = code.get(*index as usize){
		*index += 1;
        match instruction {
			Add | Sub | Mul | Div | Exp | Modulo => {
				let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
				stack.push(operations(instruction, left, right));
			}
			Call(_, args, func) => {
				func.call(args.clone());
			},
			CustomCall(func,_,_) =>  {
				let new_code = custom.get(func).unwrap();
				interpret(new_code, variables, custom, index);},
			Load(name, _) => stack.push(variables.get(name).unwrap().clone()),
            Push(val) => stack.push(val.clone()),
			SetVar(name, _) => {
				let test = name.clone();
				let vall = interpret(&code, variables, custom, index);
				println!("{:?}, {:?}",test,vall);
				variables.insert(test, vall);
			},
			End => return stack.pop().unwrap(),
			_ => todo!(),
        }
    }
	if stack.len() == 0 {return Type::Void;}
    return stack.pop().unwrap();
}
fn operations(instruction: &Instruction, left: Type, right: Type) -> Type{
	match instruction {
		Add => left + right,
		Sub => left - right,
		Mul => left * right,
		Div => left / right,
		Exp => left.exp(&right),
		Modulo => left % right,
		_ => panic!("Unsupported operation"),
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