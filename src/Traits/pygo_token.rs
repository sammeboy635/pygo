use crate::Interpreter::lexer::{Tokenizer, load_file};

use crate::PygoTypes::pygo_token::*;
use crate::PygoTypes::pygo_instruction::*;

pub trait PygoTokenVec {
    fn _debug_print(&self);
	fn _load_file(&mut self, filename :&str);
	fn _to_instructions(&self) -> Vec<Instruction>;
	fn pre_parser(&mut self) -> Option<Vec<PygoToken>>;
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
	fn _load_file(&mut self, filename :&str){
		let code = load_file(filename);
		let mut lexer = Tokenizer::new(code.as_str());
		lexer.tokenize(self);
		
	}
	fn _to_instructions(&self) -> Vec<Instruction> {
        println!("test");
        return vec![];
    }

	fn pre_parser(&mut self) -> Option<Vec<PygoToken>> {
		let mut tokens = self.iter().peekable();
		let mut output: Vec<PygoToken> = Vec::new();
		let mut operators: Vec<PygoToken> = Vec::new();
		let mut temp_variable: Option<PygoToken> = None;
		let mut func_paren_count: usize = 0;
		let mut inside_paren_count: usize = 0;
		while let Some(token) = tokens.next() {
			match token {
				PygoToken::FUNCTION_NAME(_) => {
					output.push(token.clone());
					func_paren_count += 1;
				}
				PygoToken::INTEGER_LITERAL(_) | PygoToken::FLOATING_POINT_LITERAL(_) | PygoToken::STRING_LITERAL(_) | PygoToken::BOOLEAN_LITERAL(_) |
				 PygoToken::VARIABLE_NAME(_) => {
					output.push(token.clone());
				}
				PygoToken::OPEN_PAREN => {
					if func_paren_count > 0 {
						func_paren_count -= 1;
						output.push(token.clone());
					} else {
						inside_paren_count += 1;
						operators.push(token.clone());
					}
				}
				PygoToken::CLOSED_PAREN | PygoToken::COMMA | PygoToken::END => {
					while let Some(top_op) = operators.pop() {
						if PygoToken::OPEN_PAREN == top_op {
							break;
						}
						output.push(top_op);
					}
					if (*token == PygoToken::CLOSED_PAREN && inside_paren_count > 0) {
						inside_paren_count -= 1;
						continue;
					}
					output.push(token.clone());
				}
				_ if token.is_op() || token.is_var() => {
					while let Some(top_op) = operators.last() {
						if let PygoToken::OPEN_PAREN = top_op {
							break;
						}
						if token.precedence() > top_op.precedence() {
							break;
						}
						output.push(operators.pop().unwrap());
					}
					operators.push(token.clone());
				}
				_ => output.push(token.clone()),
			}
		}
	
		while let Some(op) = operators.pop() {
			output.push(op);
		}
		if output.is_empty() {
			return None;
		}
	
		Some(output)
	}
	

}

