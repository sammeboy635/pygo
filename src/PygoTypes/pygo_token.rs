
use hashbrown::HashMap;
use std::hash::{Hash, Hasher};


#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Float32(pub f32);

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

	pub fn _is_keyword(cur_str : &str) -> Option<PygoToken>{
		match cur_str{
		"import" => Some(PygoToken::IMPORT),
		"from" => Some(PygoToken::FROM),
		"class" => Some(PygoToken::CLASS),
		"def" => Some(PygoToken::DEF),
		"if" => Some(PygoToken::IF),
		"elif" => Some(PygoToken::ELIF),
		"else" => Some(PygoToken::ELSE),
		"while" => Some(PygoToken::WHILE),
		"for" => Some(PygoToken::FOR),
		"try" => Some(PygoToken::TRY),
		"except" => Some(PygoToken::EXCEPT),
		"finally" => Some(PygoToken::FINALLY),
		"with" => Some(PygoToken::WITH),
		"pass" => Some(PygoToken::PASS),
		"break" => Some(PygoToken::BREAK),
		"continue" => Some(PygoToken::CONTINUE),
		"return" => Some(PygoToken::RETURN),
		"raise" => Some(PygoToken::RAISE),
		"yield" => Some(PygoToken::YIELD),
		"async" => Some(PygoToken::ASYNC),
		"await" => Some(PygoToken::AWAIT),
		"global" => Some(PygoToken::GLOBAL),
		"nonlocal" => Some(PygoToken::NONLOCAL),
		"assert" => Some(PygoToken::ASSERT),
		"lambda" => Some(PygoToken::LAMBDA),
		_ => None,
		}
	}

	pub fn _is_literal(cur_str : &str) -> Option<PygoToken>{
		match cur_str {
			"True" => Some(PygoToken::BOOLEAN_LITERAL(true)),
			"False" => Some(PygoToken::BOOLEAN_LITERAL(false)),
			"None" => Some(PygoToken::NONE_LITERAL),
			_ => None,
		}
	}
	pub fn _is_op(cur_str : &str) -> Option<PygoToken>{
		match cur_str {
			"+" => Some(PygoToken::ADDITION),
			"-" => Some(PygoToken::SUBTRACTION),
			"*" => Some(PygoToken::MULTIPLICATION),
			"/" => Some(PygoToken::DIVISION),
			"//" => Some(PygoToken::FLOOR_DIVISION),
			"%" => Some(PygoToken::MODULO),
			"**" => Some(PygoToken::EXPONENT),
			"==" => Some(PygoToken::EQUALITY),
			"!=" => Some(PygoToken::INEQUALITY),
			"<" => Some(PygoToken::LESS_THAN),
			">" => Some(PygoToken::GREATER_THAN),
			"<=" => Some(PygoToken::LESS_THAN_OR_EQUAL_TO),
			">=" => Some(PygoToken::GREATER_THAN_OR_EQUAL_TO),
			"and" => Some(PygoToken::LOGICAL_AND),
			"or" => Some(PygoToken::LOGICAL_OR),
			"not" => Some(PygoToken::LOGICAL_NOT),
			"&" => Some(PygoToken::BITWISE_AND),
			"|" => Some(PygoToken::BITWISE_OR),
			"^" => Some(PygoToken::BITWISE_XOR),
			"~" => Some(PygoToken::BITWISE_NOT),
			"<<" => Some(PygoToken::LEFT_SHIFT),
			">>" => Some(PygoToken::RIGHT_SHIFT),
			"=" => Some(PygoToken::ASSIGNMENT),
			"+=" => Some(PygoToken::ADDITION_ASSIGNMENT),
			"-=" => Some(PygoToken::SUBTRACTION_ASSIGNMENT),
			"*=" => Some(PygoToken::MULTIPLICATION_ASSIGNMENT),
			"/=" => Some(PygoToken::DIVISION_ASSIGNMENT),
			"//=" => Some(PygoToken::FLOOR_DIVISION_ASSIGNMENT),
			"%=" => Some(PygoToken::MODULO_ASSIGNMENT),
			"**=" => Some(PygoToken::EXPONENTIATION_ASSIGNMENT),
			"&=" => Some(PygoToken::BITWISE_AND_ASSIGNMENT),
			"|=" => Some(PygoToken::BITWISE_OR_ASSIGNMENT),
			"^=" => Some(PygoToken::BITWISE_XOR_ASSIGNMENT),
			"<<=" => Some(PygoToken::LEFT_SHIFT_ASSIGNMENT),
			">>=" => Some(PygoToken::RIGHT_SHIFT_ASSIGNMENT),
			_ => None,
		}
	}

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
			PygoToken::BOOLEAN_LITERAL(..) => true,
			PygoToken::NONE_LITERAL => true,
			_ => false,
		}
	}
	pub fn is_var(&self) -> bool{
		match self {
			PygoToken::VARIABLE_NAME_ASSIGNMENT(..) => true,
			PygoToken::VARIABLE_NAME_ASSIGNMENT_TYPE(..) => true,
			_ => false,
		}
	}
}