use std::collections::HashMap;
use Instruction::*;

#[derive(Debug)]
pub enum Instruction {
    Add,
    Sub,
    Mul,
    Div,
    PushInt(i32),
    PushFloat(f64),
    PushStr(String),
    LoadInt(String),
    LoadFloat(String),
    LoadStr(String),
	SetVar(String),
}

pub fn interpret(code: &[Instruction], variables: &HashMap<String, f64>) -> f64 {
    let mut stack = Vec::new();

    for instruction in code {
        match instruction {
			Add | Sub | Mul | Div => {
				let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
				operations(instruction, &mut stack, left, right)
			}
            PushInt(n) => stack.push(*n as f64),
            PushFloat(f) => stack.push(*f),
            PushStr(_) => panic!("String operations are not supported"),
            LoadInt(var) | LoadFloat(var) | LoadStr(var) => {
                let var_value = variables.get(var).unwrap();
                stack.push(*var_value);
            },
			SetVar(var) => print!("test"),
        }
    }
    stack.pop().unwrap()
}
fn operations(instruction: &Instruction,  stack: &mut Vec<f64>, left: f64, right: f64){
	match instruction {
		Add => stack.push(left + right),
		Sub => stack.push(left - right),
		Mul => stack.push(left * right),
		Div => stack.push(left / right),
		_ => panic!("Unsupported operation"),
	}
}