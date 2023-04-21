use crate::interpreter::Instruction::*;
use std::{collections::HashMap};
#[derive(Debug, Clone)]
pub enum Type {
    Int(i64),
    Float(f64),
    Double(f64),
    String(String),
	Unknown,
    // Add more types as needed
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Add,
	Sub,
	Div,
    Mul,
	Modulo,
	Exp,
    Load(String, Type),
	Push(Type),
	SetVar(String, Type),
}

pub fn interpret(code: &[Instruction], variables: &mut HashMap<String, Type>) -> Type {
    let mut stack = Vec::new();

    for instruction in code {
        match instruction {
			Add | Sub | Mul | Div | Exp => {
				let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
				operations(instruction, &mut stack, left, right)
			}
            Push(val) => push(&mut stack, val),
			SetVar(name, val) => {
				let test = name.clone();
				let vall = interpret(&code[1..], variables);
				variables.insert(test, vall);
			},
			_ => todo!(),
        }
    }
    return Type::Float(stack.pop().unwrap());
}
fn push(stack: &mut Vec<f64>, val : &Type){
	match val {
		Type::Int(x) => stack.push(*x as f64),
		Type::Float(x) => stack.push(*x),
		Type::Double(x) => stack.push(*x),
		//Type::String(x) => stack.push(*x),
		_ => panic!("Unsupported type"),
	}
}
fn operations(instruction: &Instruction,  stack: &mut Vec<f64>, left: f64, right: f64){
	match instruction {
		Add => stack.push(left + right),
		Sub => stack.push(left - right),
		Mul => stack.push(left * right),
		Div => stack.push(left / right),
		Exp => stack.push(left.powf(right)),
		Modulo => stack.push(left % right),
		_ => panic!("Unsupported operation"),
	}
}