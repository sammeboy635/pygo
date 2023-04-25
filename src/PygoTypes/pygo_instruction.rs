use crate::PygoTypes::pygo_type::Type;
use crate::PygoTypes::pygo_function::MyFunc;

#[derive()]
pub enum Instruction {
    Add,
	Sub,
	Div,
    Mul,
	Modulo,
	Exp,
	End,
	Arg(Vec<Type>),
	Call(String, Type, MyFunc),
    Load(String, Type),
	Push(Type),
	SetVar(String, Type),
	CustomCall(String, Vec<Instruction>, Vec<Instruction>)
}

use std::fmt;

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Add => write!(f, "Add"),
            Instruction::Sub => write!(f, "Sub"),
            Instruction::Div => write!(f, "Div"),
            Instruction::Mul => write!(f, "Mul"),
            Instruction::Modulo => write!(f, "Modulo"),
            Instruction::Exp => write!(f, "Exp"),
			Instruction::End => write!(f, "End"),
            Instruction::Arg(args) => write!(f, "Arg({:?})", args),
            Instruction::Call(name, arg, _) => write!(f, "Call({}, {:?})", name, arg),
            Instruction::Load(name, val) => write!(f, "Load({}, {:?})", name, val),
            Instruction::Push(val) => write!(f, "Push({:?})", val),
            Instruction::SetVar(name, val) => write!(f, "SetVar({}, {:?})", name, val),
			Instruction::CustomCall(name, instruction,_) => write!(f, "CustomCall({:?}, {:?})", name, instruction)
        }
    }
}