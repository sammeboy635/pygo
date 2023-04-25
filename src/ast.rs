
use std::rc::Rc;
pub struct MyFunc(Rc<dyn Fn(Type) -> Type>);

impl Clone for MyFunc {
    fn clone(&self) -> Self {
        MyFunc(Rc::clone(&self.0))
    }
}

impl MyFunc {
    pub fn new<F>(f: F) -> Self
    where
        F: 'static + Fn(Type) -> Type,
    {
        MyFunc(Rc::new(f))
    }

    pub fn call(&self, arg: Type) -> Type {
        (self.0)(arg)
    }
}


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

#[derive(Debug, Clone)]
pub enum Type {
	Void,
    Int(i64),
    Float(f64),
    Double(f64),
    String(String),
	Unknown,
    // Add more types as needed
}

use std::ops;
impl ops::Add for Type {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match (self, other) {
            (Type::Int(a), Type::Int(b)) => Type::Int(a + b),
            (Type::Float(a), Type::Float(b)) => Type::Float(a + b),
            (Type::Double(a), Type::Double(b)) => Type::Double(a + b),
            (Type::String(a), Type::String(b)) => Type::String(a + &b),
            _ => panic!("Incompatible types for addition"),
        }
    }
}
impl ops::Sub for Type {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        match (self, other) {
            (Type::Int(a), Type::Int(b)) => Type::Int(a - b),
            (Type::Float(a), Type::Float(b)) => Type::Float(a - b),
            (Type::Double(a), Type::Double(b)) => Type::Double(a - b),
            _ => panic!("Incompatible types for addition"),
        }
    }
}
impl ops::Mul for Type {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        match (self, other) {
            (Type::Int(a), Type::Int(b)) => Type::Int(a * b),
            (Type::Float(a), Type::Float(b)) => Type::Float(a * b),
            (Type::Double(a), Type::Double(b)) => Type::Double(a * b),
            _ => panic!("Incompatible types for addition"),
        }
    }
}
impl ops::Div for Type {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        match (self, other) {
            (Type::Int(a), Type::Int(b)) => Type::Int(a / b),
            (Type::Float(a), Type::Float(b)) => Type::Float(a / b),
            (Type::Double(a), Type::Double(b)) => Type::Double(a / b),
            _ => panic!("Incompatible types for addition"),
        }
    }
}

impl ops::Rem for Type {
    type Output = Self;

    fn rem(self, other: Self) -> Self {
        match (self, other) {
            (Type::Int(a), Type::Int(b)) => Type::Int(a % b),
            (Type::Float(a), Type::Float(b)) => Type::Float(a % b),
            (Type::Double(a), Type::Double(b)) => Type::Double(a % b),
            _ => panic!("Incompatible types for addition"),
        }
    }
}

impl Type {
    pub fn exp(&self, other: &Self) -> Self {
        match (self, other) {
            (Type::Int(a), Type::Int(b)) => Type::Int(a.pow(*b as u32)),
            (Type::Float(a), Type::Float(b)) => Type::Float(a.powf(*b)),
            (Type::Double(a), Type::Double(b)) => Type::Double(a.powf(*b)),
            _ => panic!("Incompatible types for exponentiation"),
        }
    }
}