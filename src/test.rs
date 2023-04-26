use std::collections::HashMap;
use std::str::Chars;
use std::iter::Peekable;

#[derive(Debug, PartialEq, Clone)]
enum PygoToken {
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
    
    // Identifiers
    VARIABLE_NAME(String),
    FUNCTION_NAME(String),
    CLASS_NAME(String),
	IMPORT_NAME(String),
    
    // Literals
    STRING_LITERAL(String),
    INTEGER_LITERAL(i32),
    FLOATING_POINT_LITERAL(f32),
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
}


pub struct Tokenizer<'a> {
    input: Peekable<Chars<'a>>,
    keywords: HashMap<&'a str, PygoToken>,
    operators: HashMap<&'a str, PygoToken>,
	indentation_level : i64,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
		let keywords: HashMap<&str, PygoToken> = [
			("import", PygoToken::IMPORT),
			("from", PygoToken::FROM),
			("class", PygoToken::CLASS),
			("def", PygoToken::DEF),
			("if", PygoToken::IF),
			("elif", PygoToken::ELIF),
			("else", PygoToken::ELSE),
			("while", PygoToken::WHILE),
			("for", PygoToken::FOR),
			("try", PygoToken::TRY),
			("except", PygoToken::EXCEPT),
			("finally", PygoToken::FINALLY),
			("with", PygoToken::WITH),
			("pass", PygoToken::PASS),
			("break", PygoToken::BREAK),
			("continue", PygoToken::CONTINUE),
			("return", PygoToken::RETURN),
			("raise", PygoToken::RAISE),
			("yield", PygoToken::YIELD),
			("async", PygoToken::ASYNC),
			("await", PygoToken::AWAIT),
			("global", PygoToken::GLOBAL),
			("nonlocal", PygoToken::NONLOCAL),
			("assert", PygoToken::ASSERT),
			("lambda", PygoToken::LAMBDA),
		]
		.iter()
		.cloned()
		.collect();

		let operators: HashMap<&str, PygoToken> = [
			("+", PygoToken::ADDITION),
			("-", PygoToken::SUBTRACTION),
			("*", PygoToken::MULTIPLICATION),
			("/", PygoToken::DIVISION),
			("//", PygoToken::FLOOR_DIVISION),
			("%", PygoToken::MODULO),
			("**", PygoToken::EXPONENT),
			("==", PygoToken::EQUALITY),
			("!=", PygoToken::INEQUALITY),
			("<", PygoToken::LESS_THAN),
			(">", PygoToken::GREATER_THAN),
			("<=", PygoToken::LESS_THAN_OR_EQUAL_TO),
			(">=", PygoToken::GREATER_THAN_OR_EQUAL_TO),
			("and", PygoToken::LOGICAL_AND),
			("or", PygoToken::LOGICAL_OR),
			("not", PygoToken::LOGICAL_NOT),
			("&", PygoToken::BITWISE_AND),
			("|", PygoToken::BITWISE_OR),
			("^", PygoToken::BITWISE_XOR),
			("~", PygoToken::BITWISE_NOT),
			("<<", PygoToken::LEFT_SHIFT),
			(">>", PygoToken::RIGHT_SHIFT),
			("=", PygoToken::ASSIGNMENT),
			("+=", PygoToken::ADDITION_ASSIGNMENT),
			("-=", PygoToken::SUBTRACTION_ASSIGNMENT),
			("*=", PygoToken::MULTIPLICATION_ASSIGNMENT),
			("/=", PygoToken::DIVISION_ASSIGNMENT),
			("//=", PygoToken::FLOOR_DIVISION_ASSIGNMENT),
			("%=", PygoToken::MODULO_ASSIGNMENT),
			("**=", PygoToken::EXPONENTIATION_ASSIGNMENT),
			("&=", PygoToken::BITWISE_AND_ASSIGNMENT),
			("|=", PygoToken::BITWISE_OR_ASSIGNMENT),
			("^=", PygoToken::BITWISE_XOR_ASSIGNMENT),
			("<<=", PygoToken::LEFT_SHIFT_ASSIGNMENT),
			(">>=", PygoToken::RIGHT_SHIFT_ASSIGNMENT),
		]
		.iter()
		.cloned()
		.collect();

        Tokenizer {
            input: input.chars().peekable(),
            keywords,
            operators,
			indentation_level : 0,
        }
    }
	pub fn tokenize(&mut self) -> Vec<PygoToken> {
        let mut tokens = Vec::new();

        while let Some(&c) = self.input.peek() {
            match c {
                // Implement tokenization logic here.
                // For example:
                ' ' | '\n' | '\t' => {
                    self.input.next();
                }
                _ => {
                    if let Some(token) = self.parse_identifier_or_keyword(tokens.last()) {
                        tokens.push(token);
                    } else if let Some(token) = self.parse_operator() {
                        tokens.push(token);
					}else if let Some(token) = self.parse_literal() {
						tokens.push(token);
					}else {
                        // Handle other cases (e.g., literals, delimiters) or unknown tokens
						println!("{:?}", c);
                        self.input.next();
                    }
                }
            }
        }

        tokens
    }
	fn parse_identifier_or_keyword(&mut self, last_token : Option<&PygoToken>) -> Option<PygoToken> {
        let mut identifier = String::new();

        while let Some(&c) = self.input.peek() {
            if c.is_alphabetic() || c == '_' || (!identifier.is_empty() && c.is_digit(10)) {
                identifier.push(c);
                self.input.next();
            } else {
                break;
            }
        }

        if identifier.is_empty() {
            None
        } else if let Some(keyword_token) = self.keywords.get(&identifier[..]) {
            Some(keyword_token.clone())
        } else {
			// In a complete tokenizer, you should differentiate between
            // VARIABLE_NAME, FUNCTION_NAME, and CLASS_NAME based on context
			if *last_token.unwrap_or(&PygoToken::NONE_LITERAL) == PygoToken::IMPORT {
				return Some(PygoToken::IMPORT_NAME(identifier));
			}else if *last_token.unwrap_or(&PygoToken::NONE_LITERAL) == PygoToken::CLASS {
				return Some(PygoToken::CLASS_NAME(identifier));
			}
            
            // Peek the next non-whitespace character
			while let Some(&c) = self.input.peek() {
				if !c.is_whitespace() {
					break;
				}
				self.input.next();
			}

			let &next_char = self.input.peek().unwrap_or(&'\0');
			if next_char == '(' {
				Some(PygoToken::FUNCTION_NAME(identifier))
			} else {
				Some(PygoToken::VARIABLE_NAME(identifier))
			}
        }
    }

    fn parse_operator(&mut self) -> Option<PygoToken> {
        let mut possible_operator = String::new();
        let mut current_operator = String::new();
        let mut current_token = None;

        while let Some(&c) = self.input.peek() {
            possible_operator.push(c);
            if let Some(token) = self.operators.get(&possible_operator[..]) {
                current_operator = possible_operator.clone();
                current_token = Some(token.clone());
                self.input.next();
            } else {
                break;
            }
        }

        current_token
    }

    fn parse_literal(&mut self) -> Option<PygoToken> {
        let &next_char = self.input.peek()?;

        match next_char {
            '\'' | '\"' => self.parse_string_literal(),
            '0'..='9' => self.parse_number_literal(),
            // Add more literal types here, like BOOLEAN_LITERAL and NONE_LITERAL
            _ => None,
        }
    }

    fn parse_string_literal(&mut self) -> Option<PygoToken> {
        // In a complete tokenizer, you should handle single and double quotes,
        // as well as escape sequences and multi-line strings.
        // This implementation is a simplified example.
        let quote = self.input.next()?;
        let mut string_literal = String::new();

        while let Some(&c) = self.input.peek() {
            if c != quote {
                string_literal.push(c);
                self.input.next();
            } else {
                self.input.next();
                break;
            }
        }

        Some(PygoToken::STRING_LITERAL(string_literal))
    }

    fn parse_number_literal(&mut self) -> Option<PygoToken> {
        // In a complete tokenizer, you should handle integer, floating-point,
        // and complex literals. This implementation is a simplified example.
        let mut number_literal = String::new();

        while let Some(&c) = self.input.peek() {
            if c.is_digit(10) || c == '.' {
                number_literal.push(c);
                self.input.next();
            } else {
                break;
            }
        }

        if let Ok(val) = number_literal.parse::<f32>() {
            Some(PygoToken::FLOATING_POINT_LITERAL(val))
        } else if let Ok(val) = number_literal.parse::<i32>(){
            Some(PygoToken::INTEGER_LITERAL(val))
        }else{
			Some(PygoToken::NONE_LITERAL)
		}
    }
	fn parse_handle_endline(&mut self){
		self.input.next();
		let mut current_indentation = 0;
		while let Some(&next_char) = self.input.peek() {
			if next_char == '\t' {
				current_indentation += 1;
				self.input.next();
			} else {
				break;
			}
		}
		if current_indentation != self.indentation_level {
			panic!(
				"Incorrect indentation: expected {} tabs, found {}",
				self.indentation_level, current_indentation
			);
		}
	}
}
use crate::Utils::timer::Timer;
#[test]
fn main2() {
    let code = r#"
		#test
        import math
        def my_function(x):
            if x > 0:
                return x ** 2
            else:
                return -x
		test=5
    "#;
	
    let mut tokenizer = Tokenizer::new(code);
	let mut timer = Timer::new();
    let tokens = tokenizer.tokenize();
	timer.print("Done");

    println!("{:?}", tokens);
}