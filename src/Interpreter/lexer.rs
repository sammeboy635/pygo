use std::fmt::Error;
use std::str::Chars;
use std::iter::Peekable;
use hashbrown::HashMap;
use std::hash::{Hash, Hasher};

use crate::PygoTypes::pygo_token::{*, PygoToken::*};

pub struct Tokenizer<'a> {
    input: Peekable<Chars<'a>>,
	indentation_level : i64,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        Tokenizer {
            input: input.chars().peekable(),
			indentation_level : 0,
        }
    }
	pub fn tokenize(&mut self) -> Vec<PygoToken> {
        let mut tokens = Vec::new();

        while let Some(&c) = self.input.peek() {
			// println!("{:?}",tokens);
			// println!("{:?}",c);

            match c {
				'?' => {
					self.input.next();
					continue;
				},
				'\n' => {
					// Checks for empty list and if End was the last Token
					if (!tokens.is_empty() && (*tokens.last().unwrap_or(&PygoToken::UNKNOWN) != PygoToken::END)) {
						tokens.push(PygoToken::END);
					}
					// Handles the \n
					if let Some(new_token) = self.parse_handle_endline_checks() {
						tokens.push(new_token);
					}
				},
                ' ' | '\t' | '\r' | ':' => {self.input.next();},
				'#' => self.skip_comment(),
				'(' => {
					self.input.next();
					tokens.push(PygoToken::OPEN_PAREN);
				},
				')' => {
					self.input.next();
					tokens.push(PygoToken::CLOSED_PAREN);
				},
				',' => {
					self.input.next();
					tokens.push(PygoToken::COMMA);
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
        } else if let Some(keyword_token) = PygoToken::_is_keyword(&identifier[..]) {
            Some(keyword_token)
		} else if let Some(literal_token) = PygoToken::_is_literal(&identifier[..]){
			Some(literal_token)
        } else {
			// In a complete tokenizer, you should differentiate between
            // VARIABLE_NAME, FUNCTION_NAME, and CLASS_NAME based on context
			if *last_token.unwrap_or(&PygoToken::UNKNOWN) == PygoToken::IMPORT {
				return Some(PygoToken::IMPORT_NAME(identifier));
			}else if *last_token.unwrap_or(&PygoToken::UNKNOWN) == PygoToken::CLASS {
				return Some(PygoToken::CLASS_NAME(identifier));
			}
            
            // Peek the next non-whitespace character
			let mut count = 0;
			while let Some(&c) = self.input.peek() {
				if (c == '\n' || !c.is_whitespace()) {
					break;
				}
				self.input.next();
			}

			let &next_char = self.input.peek().unwrap_or(&'\0');
			if next_char == '(' {
				Some(PygoToken::FUNCTION_NAME(identifier))
			} else {
				// HANDLE VARIABLES
				let ident_type = self.parse_variable_type();

				// Check for assignment operator
				let mut is_assignment = false;
				let mut temp_input = self.input.clone();
				while let Some(&c) = temp_input.peek() {
					if c.is_whitespace() {
						temp_input.next();
					} else {
						if c == '=' {
							is_assignment = true;
						}
						break;
					}
				}
		
				if is_assignment {
					if let Some(ident_type) = ident_type {
						Some(PygoToken::VARIABLE_NAME_ASSIGNMENT_TYPE(identifier, ident_type))
					} else {
						Some(PygoToken::VARIABLE_NAME_ASSIGNMENT(identifier))
					}
				} else {
					if let Some(ident_type) = ident_type {
						Some(PygoToken::VARIABLE_NAME_TYPE(identifier, ident_type))
					} else {
						Some(PygoToken::VARIABLE_NAME(identifier))
					}
				}
			}
        }
    }

    fn parse_operator(&mut self) -> Option<PygoToken> {
        let mut possible_operator = String::new();
        let mut current_operator = String::new();
        let mut current_token = None;

        while let Some(&c) = self.input.peek() {
            possible_operator.push(c);
            if let Some(token) = PygoToken::_is_op(&possible_operator[..]) {
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
            Some(PygoToken::FLOATING_POINT_LITERAL(Float32(val)))
        } else if let Ok(val) = number_literal.parse::<i32>(){
            Some(PygoToken::INTEGER_LITERAL(val))
        }else{
			Some(PygoToken::NONE_LITERAL)
		}
    }
	fn parse_variable_type(&mut self) -> Option<String> {
		let mut found_colon = false;
	
		// Skip any whitespaces and newlines before the colon
		while let Some(&c) = self.input.peek() {
			if c == ' ' {
				self.input.next();
			} else if c == ':' {
				self.input.next();
				found_colon = true;
				break;
			} else {
				break;
			}
		}
	
		if found_colon {
			let mut variable_type = String::new();
	
			// Parse the type (only alphabetic characters, digits, and underscores are allowed)
			while let Some(&c) = self.input.peek() {
				if c == ' '{
					self.input.next();
				}else if c.is_alphabetic() || c == '_' || (!variable_type.is_empty() && c.is_digit(10)) {
					variable_type.push(c);
					self.input.next();
				} else {
					break;
				}
			}
	
			if !variable_type.is_empty() {
				return Some(variable_type);
			}
		}
	
		None
	}
	fn parse_handle_endline_checks(&mut self) -> Option<PygoToken>{
		self.input.next();
		let mut current_indentation = 0;
		while let Some(&next_char) = self.input.peek() {
			if next_char == '\t' {
				current_indentation += 1;
			} else if next_char == '\n'{
				current_indentation = 0;
			} else {
				break;
			}
			self.input.next();
		}
		if current_indentation == 0{
			return None;
		}
		return Some(PygoToken::TAB(current_indentation));
	}
	fn skip_comment(&mut self) {
        while let Some(&c) = self.input.peek() {
            if c == '\n' {
                break;
            } else {
                self.input.next();
            }
        }
    }
}

use std::fs::File;
use std::io::prelude::*;

pub fn load_file(file_name: &str) -> String{
	let mut file = File::open(file_name).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");
	return contents;
}

pub fn print_tokens(tokens: &Vec<PygoToken>) {
    let mut output_buffer = String::new();
    output_buffer.push_str("\n");
    for token in tokens {
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

impl PygoToken {
	
}



use crate::Utils::timer::Timer;

#[test]
pub fn main2() {
	let binding = load_file("tmp/main.py");
    let code = binding.as_str();
	
    let mut tokenizer = Tokenizer::new(code);
	let mut timer = Timer::new();
    let tokens = tokenizer.tokenize();
	print_tokens(&tokens);
}