
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
pub enum PygoKeyword{
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
}
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum PygoLiteral {
	// Literals
    STRING_LITERAL(String),
    INTEGER_LITERAL(i32),
    FLOATING_POINT_LITERAL(Float32),
    COMPLEX_LITERAL,
    BOOLEAN_LITERAL(bool),
    NONE_LITERAL,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum PygoOp{
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
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum PygoToken {
    // Keywords
	KEYWORD(PygoKeyword),

	// Literals
	LITERAL(PygoLiteral),

	// Operators
	OPERATOR(PygoOp),
    
    // Identifiers
    VARIABLE_NAME(String),
    FUNCTION_NAME(String),
    CLASS_NAME(String),
	IMPORT_NAME(String),
    
	// Format
	END,
	TAB(i32),
	COMMA,

	// Unknown
	UNKNOWN,
}

impl PygoKeyword{
	pub fn _is_keyword(cur_str : &str) -> Option<PygoToken>{
		match cur_str{
		"import" => Some(PygoToken::KEYWORD(PygoKeyword::IMPORT)),
		"from" => Some(PygoToken::KEYWORD(PygoKeyword::FROM)),
		"class" => Some(PygoToken::KEYWORD(PygoKeyword::CLASS)),
		"def" => Some(PygoToken::KEYWORD(PygoKeyword::DEF)),
		"if" => Some(PygoToken::KEYWORD(PygoKeyword::IF)),
		"elif" => Some(PygoToken::KEYWORD(PygoKeyword::ELIF)),
		"else" => Some(PygoToken::KEYWORD(PygoKeyword::ELSE)),
		"while" => Some(PygoToken::KEYWORD(PygoKeyword::WHILE)),
		"for" => Some(PygoToken::KEYWORD(PygoKeyword::FOR)),
		"try" => Some(PygoToken::KEYWORD(PygoKeyword::TRY)),
		"except" => Some(PygoToken::KEYWORD(PygoKeyword::EXCEPT)),
		"finally" => Some(PygoToken::KEYWORD(PygoKeyword::FINALLY)),
		"with" => Some(PygoToken::KEYWORD(PygoKeyword::WITH)),
		"pass" => Some(PygoToken::KEYWORD(PygoKeyword::PASS)),
		"break" => Some(PygoToken::KEYWORD(PygoKeyword::BREAK)),
		"continue" => Some(PygoToken::KEYWORD(PygoKeyword::CONTINUE)),
		"return" => Some(PygoToken::KEYWORD(PygoKeyword::RETURN)),
		"raise" => Some(PygoToken::KEYWORD(PygoKeyword::RAISE)),
		"yield" => Some(PygoToken::KEYWORD(PygoKeyword::YIELD)),
		"async" => Some(PygoToken::KEYWORD(PygoKeyword::ASYNC)),
		"await" => Some(PygoToken::KEYWORD(PygoKeyword::AWAIT)),
		"global" => Some(PygoToken::KEYWORD(PygoKeyword::GLOBAL)),
		"nonlocal" => Some(PygoToken::KEYWORD(PygoKeyword::NONLOCAL)),
		"assert" => Some(PygoToken::KEYWORD(PygoKeyword::ASSERT)),
		"lambda" => Some(PygoToken::KEYWORD(PygoKeyword::LAMBDA)),
		_ => None,
		}
	}
}
impl PygoLiteral {
	pub fn _is_literal(cur_str : &str) -> Option<PygoToken>{
		match cur_str {
			"True" => Some(PygoToken::LITERAL(PygoLiteral::BOOLEAN_LITERAL(true))),
			"False" => Some(PygoToken::LITERAL(PygoLiteral::BOOLEAN_LITERAL(false))),
			"None" => Some(PygoToken::LITERAL(PygoLiteral::NONE_LITERAL)),
			_ => None,
		}
	}
}
impl PygoOp{
	pub fn _is_op(cur_str : &str) -> Option<PygoToken>{
		match cur_str {
			"+" => Some(PygoToken::OPERATOR(PygoOp::ADDITION)),
			"-" => Some(PygoToken::OPERATOR(PygoOp::SUBTRACTION)),
			"*" => Some(PygoToken::OPERATOR(PygoOp::MULTIPLICATION)),
			"/" => Some(PygoToken::OPERATOR(PygoOp::DIVISION)),
			"//" => Some(PygoToken::OPERATOR(PygoOp::FLOOR_DIVISION)),
			"%" => Some(PygoToken::OPERATOR(PygoOp::MODULO)),
			"**" => Some(PygoToken::OPERATOR(PygoOp::EXPONENT)),
			"==" => Some(PygoToken::OPERATOR(PygoOp::EQUALITY)),
			"!=" => Some(PygoToken::OPERATOR(PygoOp::INEQUALITY)),
			"<" => Some(PygoToken::OPERATOR(PygoOp::LESS_THAN)),
			">" => Some(PygoToken::OPERATOR(PygoOp::GREATER_THAN)),
			"<=" => Some(PygoToken::OPERATOR(PygoOp::LESS_THAN_OR_EQUAL_TO)),
			">=" => Some(PygoToken::OPERATOR(PygoOp::GREATER_THAN_OR_EQUAL_TO)),
			"and" => Some(PygoToken::OPERATOR(PygoOp::LOGICAL_AND)),
			"or" => Some(PygoToken::OPERATOR(PygoOp::LOGICAL_OR)),
			"not" => Some(PygoToken::OPERATOR(PygoOp::LOGICAL_NOT)),
			"&" => Some(PygoToken::OPERATOR(PygoOp::BITWISE_AND)),
			"|" => Some(PygoToken::OPERATOR(PygoOp::BITWISE_OR)),
			"^" => Some(PygoToken::OPERATOR(PygoOp::BITWISE_XOR)),
			"~" => Some(PygoToken::OPERATOR(PygoOp::BITWISE_NOT)),
			"<<" => Some(PygoToken::OPERATOR(PygoOp::LEFT_SHIFT)),
			">>" => Some(PygoToken::OPERATOR(PygoOp::RIGHT_SHIFT)),
			"=" => Some(PygoToken::OPERATOR(PygoOp::ASSIGNMENT)),
			"+=" => Some(PygoToken::OPERATOR(PygoOp::ADDITION_ASSIGNMENT)),
			"-=" => Some(PygoToken::OPERATOR(PygoOp::SUBTRACTION_ASSIGNMENT)),
			"*=" => Some(PygoToken::OPERATOR(PygoOp::MULTIPLICATION_ASSIGNMENT)),
			"/=" => Some(PygoToken::OPERATOR(PygoOp::DIVISION_ASSIGNMENT)),
			"//=" => Some(PygoToken::OPERATOR(PygoOp::FLOOR_DIVISION_ASSIGNMENT)),
			"%=" => Some(PygoToken::OPERATOR(PygoOp::MODULO_ASSIGNMENT)),
			"**=" => Some(PygoToken::OPERATOR(PygoOp::EXPONENTIATION_ASSIGNMENT)),
			"&=" => Some(PygoToken::OPERATOR(PygoOp::BITWISE_AND_ASSIGNMENT)),
			"|=" => Some(PygoToken::OPERATOR(PygoOp::BITWISE_OR_ASSIGNMENT)),
			"^=" => Some(PygoToken::OPERATOR(PygoOp::BITWISE_XOR_ASSIGNMENT)),
			"<<=" => Some(PygoToken::OPERATOR(PygoOp::LEFT_SHIFT_ASSIGNMENT)),
			">>=" => Some(PygoToken::OPERATOR(PygoOp::RIGHT_SHIFT_ASSIGNMENT)),
			_ => None,
		}
	}
	pub fn _precedence(&self) -> Option<usize> {
        match self {
            PygoOp::ADDITION | PygoOp::SUBTRACTION => Some(1),
            PygoOp::MULTIPLICATION | PygoOp::DIVISION | PygoOp::MODULO => Some(2),
            PygoOp::FLOOR_DIVISION => Some(3),
            PygoOp::EXPONENT => Some(4),
            PygoOp::EQUALITY | PygoOp::INEQUALITY | PygoOp::LESS_THAN |
                PygoOp::GREATER_THAN | PygoOp::LESS_THAN_OR_EQUAL_TO | PygoOp::GREATER_THAN_OR_EQUAL_TO => Some(5),
            PygoOp::LOGICAL_AND => Some(6),
            PygoOp::LOGICAL_OR => Some(7),
            PygoOp::LOGICAL_NOT => Some(8),
            PygoOp::BITWISE_AND => Some(9),
            PygoOp::BITWISE_OR => Some(10),
            PygoOp::BITWISE_XOR => Some(11),
            PygoOp::BITWISE_NOT => Some(12),
            PygoOp::LEFT_SHIFT | PygoOp::RIGHT_SHIFT => Some(13),
            PygoOp::ASSIGNMENT | PygoOp::ADDITION_ASSIGNMENT | PygoOp::SUBTRACTION_ASSIGNMENT |
                PygoOp::MULTIPLICATION_ASSIGNMENT | PygoOp::DIVISION_ASSIGNMENT | PygoOp::FLOOR_DIVISION_ASSIGNMENT |
                PygoOp::MODULO_ASSIGNMENT | PygoOp::EXPONENTIATION_ASSIGNMENT | PygoOp::BITWISE_AND_ASSIGNMENT |
                PygoOp::BITWISE_OR_ASSIGNMENT | PygoOp::BITWISE_XOR_ASSIGNMENT | PygoOp::LEFT_SHIFT_ASSIGNMENT |
                PygoOp::RIGHT_SHIFT_ASSIGNMENT => Some(14),
            _ => None,
        }
    }
}

impl PygoToken{
	pub fn _is_keyword(cur_str: &str) -> Option<PygoToken>{
		PygoKeyword::_is_keyword(cur_str)
	}
	pub fn _is_literal(cur_str: &str) -> Option<PygoToken>{
		PygoLiteral::_is_literal(cur_str)
	}
	pub fn _is_op(cur_str: &str) -> Option<PygoToken>{
		PygoOp::_is_op(cur_str)
	}
}