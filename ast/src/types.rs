use std::ops;

use std::rc::Rc;
pub struct Function{
	pub func: Rc<dyn Fn(Type) -> Type>
}

impl Clone for Function {
    fn clone(&self) -> Self {
        Function{
			func: Rc::clone(&self.func)
		}
    }
}

impl Function {
    pub fn new<F>(f: F) -> Self
    where
        F: 'static + Fn(Type) -> Type,
    {
        Function{
			func: Rc::new(f)
		}
    }

    pub fn call(&self, arg: Type) -> Type {
        (self.func)(arg)
    }
}

use std::hash::{Hash, Hasher};

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Float(pub f32);
impl Eq for Float{}
impl Hash for Float {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Convert the float to its raw bit representation and hash it
        let bits = self.0.to_bits();
        bits.hash(state);
    }
}

impl std::ops::Add for Float{
	type Output = Self;
	fn add(self, rhs: Self) -> Self {
		Float(self.0 + rhs.0)
	}
}
impl std::ops::Sub for Float{
	type Output = Self;
	fn sub(self, other: Self) -> Self {
		Float(self.0 - other.0)
	}
}
impl std::ops::Mul for Float{
	type Output = Self;
	fn mul(self, other: Self) -> Self {
		Float(self.0 * other.0)
	}
}
impl std::ops::Div for Float{
	type Output = Self;
	fn div(self, other: Self) -> Self {
		Float(self.0 / other.0)
	}
}
impl std::ops::Rem for Float{
	type Output = Self;
	fn rem(self, other: Self) -> Self {
		Float(self.0 % other.0)
	}
}


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
	Void,
	Bool(bool),
    Int(i64),
    Float(Float),
    Double(Float),
    String(String),
	None,
	Unknown,
    // Add more types as needed
}

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
            (Type::Float(a), Type::Float(b)) => Type::Float(Float(a.0.powf(b.0))),
            (Type::Double(a), Type::Double(b)) => Type::Double(Float(a.0.powf(b.0))),
            _ => panic!("Incompatible types for exponentiation"),
        }
    }
}