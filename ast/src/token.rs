use std::hash::{Hash, Hasher};
use crate::types::Type;

use super::instruction;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Float32(pub f32);
trait Partialf64EXT {
	fn hash<H: Hasher>(&self, state: &mut H);
}

impl Eq for Float32 {}

impl Hash for Float32 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Convert the float to its raw bit representation and hash it
        let bits = self.0.to_bits();
        bits.hash(state);
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum PygoToken {
    // Keywords
	IMPORT,
    FROM,
    CLASS,
    DEF,
    IF,
    ELIF,
    ELSE,
    WHILE,
    FOR,
    TRY,
    EXCEPT,
    FINALLY,
    WITH,
    PASS,
    BREAK,
    CONTINUE,
    RETURN,
    RAISE,
    YIELD,
    ASYNC,
    AWAIT,
    GLOBAL,
    NONLOCAL,
    ASSERT,
    LAMBDA,

	// Literals
	LITERAL(Type),
    STRING_LITERAL(String),
    INTEGER_LITERAL(i32),
    FLOATING_POINT_LITERAL(Float32),
    COMPLEX_LITERAL,
    BOOLEAN_LITERAL(bool),
    NONE_LITERAL,

	// Operators
	ADDITION,
	SUBTRACTION,
	MULTIPLICATION,
	DIVISION,
	FLOOR_DIVISION,
	MODULO,
	EXPONENT,
	EQUALITY,
	INEQUALITY,
	LESS_THAN,
	GREATER_THAN,
	LESS_THAN_OR_EQUAL_TO,
	GREATER_THAN_OR_EQUAL_TO,
	LOGICAL_AND,
	LOGICAL_OR,
	LOGICAL_NOT,
	BITWISE_AND,
	BITWISE_OR,
	BITWISE_XOR,
	BITWISE_NOT,
	LEFT_SHIFT,
	RIGHT_SHIFT,
	ASSIGNMENT,
	ADDITION_ASSIGNMENT,
	SUBTRACTION_ASSIGNMENT,
	MULTIPLICATION_ASSIGNMENT,
	DIVISION_ASSIGNMENT,
	FLOOR_DIVISION_ASSIGNMENT,
	MODULO_ASSIGNMENT,
	EXPONENTIATION_ASSIGNMENT,
	BITWISE_AND_ASSIGNMENT,
	BITWISE_OR_ASSIGNMENT,
	BITWISE_XOR_ASSIGNMENT,
	LEFT_SHIFT_ASSIGNMENT,
	RIGHT_SHIFT_ASSIGNMENT,

	// Parenthesis
	OPEN_PAREN,
	CLOSED_PAREN,
	
	// Brackets
	OPEN_BRACKET,
	CLOSED_BRACKET,
    
    // Identifiers
	VARIABLE_NAME_ASSIGNMENT_TYPE(String, String),
	VARIABLE_NAME_ASSIGNMENT(String),
    VARIABLE_NAME(String),
	VARIABLE_NAME_TYPE(String, String),
    FUNCTION_NAME(String),
    CLASS_NAME(String),
	IMPORT_NAME(String),
    
	// Format
	COLON,
	END,
	TAB(i32),
	COMMA,
	
	// Unknown
	UNKNOWN,
}

impl PygoToken{
	
	pub fn precedence(&self) -> Option<usize> {
		match self {
			PygoToken::VARIABLE_NAME_ASSIGNMENT_TYPE(..) | PygoToken::VARIABLE_NAME_ASSIGNMENT(..) => Some(0),
			PygoToken::ASSIGNMENT | PygoToken::ADDITION_ASSIGNMENT | PygoToken::SUBTRACTION_ASSIGNMENT |
				PygoToken::MULTIPLICATION_ASSIGNMENT | PygoToken::DIVISION_ASSIGNMENT | PygoToken::FLOOR_DIVISION_ASSIGNMENT |
				PygoToken::MODULO_ASSIGNMENT | PygoToken::EXPONENTIATION_ASSIGNMENT | PygoToken::BITWISE_AND_ASSIGNMENT |
				PygoToken::BITWISE_OR_ASSIGNMENT | PygoToken::BITWISE_XOR_ASSIGNMENT | PygoToken::LEFT_SHIFT_ASSIGNMENT |
				PygoToken::RIGHT_SHIFT_ASSIGNMENT => Some(1),
			PygoToken::ADDITION | PygoToken::SUBTRACTION => Some(2),
			PygoToken::MULTIPLICATION | PygoToken::DIVISION | PygoToken::MODULO => Some(3),
			PygoToken::FLOOR_DIVISION => Some(4),
			PygoToken::EXPONENT => Some(5),
			PygoToken::EQUALITY | PygoToken::INEQUALITY | PygoToken::LESS_THAN |
				PygoToken::GREATER_THAN | PygoToken::LESS_THAN_OR_EQUAL_TO | PygoToken::GREATER_THAN_OR_EQUAL_TO => Some(6),
			PygoToken::LOGICAL_AND => Some(7),
			PygoToken::LOGICAL_OR => Some(8),
			PygoToken::LOGICAL_NOT => Some(9),
			PygoToken::BITWISE_AND => Some(10),
			PygoToken::BITWISE_OR => Some(11),
			PygoToken::BITWISE_XOR => Some(12),
			PygoToken::BITWISE_NOT => Some(13),
			PygoToken::LEFT_SHIFT | PygoToken::RIGHT_SHIFT => Some(14),
			_ => None,
		}
	}
	pub fn is_keyword(&self) -> bool{
		match self{
			PygoToken::IMPORT => true,
			PygoToken::FROM => true,
			PygoToken::CLASS => true,
			PygoToken::DEF => true,
			PygoToken::IF => true,
			PygoToken::ELIF => true,
			PygoToken::ELSE => true,
			PygoToken::WHILE => true,
			PygoToken::FOR => true,
			PygoToken::TRY => true,
			PygoToken::EXCEPT => true,
			PygoToken::FINALLY => true,
			PygoToken::WITH => true,
			PygoToken::PASS => true,
			PygoToken::BREAK => true,
			PygoToken::CONTINUE => true,
			PygoToken::RETURN => true,
			PygoToken::RAISE => true,
			PygoToken::YIELD => true,
			PygoToken::ASYNC => true,
			PygoToken::AWAIT => true,
			PygoToken::GLOBAL => true,
			PygoToken::NONLOCAL => true,
			PygoToken::ASSERT => true,
			PygoToken::LAMBDA => true,
		_ => false,
		}
	}
	pub fn is_op(&self) -> bool{
		match self {
			PygoToken::ADDITION => true,
			PygoToken::SUBTRACTION => true,
			PygoToken::MULTIPLICATION => true,
			PygoToken::DIVISION => true,
			PygoToken::FLOOR_DIVISION => true,
			PygoToken::MODULO => true,
			PygoToken::EXPONENT => true,
			PygoToken::EQUALITY => true,
			PygoToken::INEQUALITY => true,
			PygoToken::LESS_THAN => true,
			PygoToken::GREATER_THAN => true,
			PygoToken::LESS_THAN_OR_EQUAL_TO => true,
			PygoToken::GREATER_THAN_OR_EQUAL_TO => true,
			PygoToken::LOGICAL_AND => true,
			PygoToken::LOGICAL_OR => true,
			PygoToken::LOGICAL_NOT => true,
			PygoToken::BITWISE_AND => true,
			PygoToken::BITWISE_OR => true,
			PygoToken::BITWISE_XOR => true,
			PygoToken::BITWISE_NOT => true,
			PygoToken::LEFT_SHIFT => true,
			PygoToken::RIGHT_SHIFT => true,
			PygoToken::ASSIGNMENT => true,
			PygoToken::ADDITION_ASSIGNMENT => true,
			PygoToken::SUBTRACTION_ASSIGNMENT => true,
			PygoToken::MULTIPLICATION_ASSIGNMENT => true,
			PygoToken::DIVISION_ASSIGNMENT => true,
			PygoToken::FLOOR_DIVISION_ASSIGNMENT => true,
			PygoToken::MODULO_ASSIGNMENT => true,
			PygoToken::EXPONENTIATION_ASSIGNMENT => true,
			PygoToken::BITWISE_AND_ASSIGNMENT => true,
			PygoToken::BITWISE_OR_ASSIGNMENT => true,
			PygoToken::BITWISE_XOR_ASSIGNMENT => true,
			PygoToken::LEFT_SHIFT_ASSIGNMENT => true,
			PygoToken::RIGHT_SHIFT_ASSIGNMENT => true,
			_ => false,
		}
	}
	pub fn is_literal(&self) -> bool{
		match self {
			PygoToken::BOOLEAN_LITERAL(..) => true,
			PygoToken::NONE_LITERAL => true,
			_ => false,
		}
	}
	pub fn is_var(&self) -> bool{
		match self {
			PygoToken::VARIABLE_NAME(..) => true,
			PygoToken::VARIABLE_NAME_TYPE(..) => true,
			PygoToken::VARIABLE_NAME_ASSIGNMENT(..) => true,
			PygoToken::VARIABLE_NAME_ASSIGNMENT_TYPE(..) => true,
			_ => false,
		}
	}
	pub fn is_delimiters(&self)-> bool{
		match self {
			PygoToken::COMMA => true,
			PygoToken::OPEN_PAREN => true,
			//PygoToken::{[Brackets]}
			PygoToken::COLON => true,
			_ => false,
		}
	}
	//Comments
}

pub trait PygoTokenVec {
    fn _debug_print(&self);

}
// Implement the trait for Vec<PygoToken>
impl PygoTokenVec for Vec<PygoToken> {
	fn _debug_print(&self) {
		let mut output_buffer = String::new();
		output_buffer.push_str("\n");
		for token in self {
			if let PygoToken::TAB(val) = token {
				for _ in 0..*val {
					output_buffer.push('\t');
				}
			} else {
				output_buffer.push_str(&format!("{:?} ", token));
			}
			if let PygoToken::END = token {
				output_buffer.push('\n');
			}
		}
		output_buffer.push_str("\n");
		print!("{}", output_buffer);
	}
}
