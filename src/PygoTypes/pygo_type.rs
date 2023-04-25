use std::ops;
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