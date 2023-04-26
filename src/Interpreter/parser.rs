use crate::PygoTypes::pygo_function::Function;
use crate::PygoTypes::pygo_type::Type;
use crate::PygoTypes::pygo_instruction::Instruction;

use crate::StandardLib::standard_library::StdLibFn;
use crate::StandardLib::standard_library;

use std::collections::HashMap;
use std::mem;

pub struct Parser {
    tokens: Vec<String>,
    position: usize,
    recursion_count: i32,
    parenthesis_count: i32,
	sl: HashMap<String, StdLibFn>,
	pub custom: HashMap<String, Vec<Instruction>>,
}

impl Parser {
    pub fn new(tokens: Vec<String>, sl: HashMap<String, StdLibFn>) -> Self {
        Parser {
            tokens,
            position: 0,
            recursion_count: 0,
            parenthesis_count: 0,
			sl,
			custom: HashMap::new(),
        }
    }

    pub fn advance(&mut self) {
		self.position += 1;
    }

	pub fn advance_count(&mut self, count: usize) {
		self.position += count;
	}

	pub fn advance_comment(&mut self) {
		while let Some(token) = self.peek(0) {
			if token == "\n"  {
				if let Some(next_token) = self.peek(1) {
					if next_token.starts_with("#") {
						self.advance();
						continue;
					}
				}
				break;
			}
			self.advance();
		}
	}

    pub fn back(&mut self) -> Option<String> {
        let prev_token = self.peek(-1).cloned();
        if prev_token.is_some() {
            self.position -= 1;
        }
        prev_token
    }

    pub fn peek(&self, offset: isize) -> Option<&String> {
        let index = (self.position as isize) + offset;

        if index >= 0 && index < self.tokens.len() as isize {
            Some(&self.tokens[index as usize])
        } else {
            None
        }
    }
	pub fn peek_compare(&self, offset: isize, target: &str) -> bool {
		if let Some(token) = self.peek(offset) {
			token == target
		} else {
			false
		}
	}

    pub fn peek_except(&self, offset: isize, exception: &str) -> Option<&String> {
        let token = self.peek(offset)?;
        if token != exception {
            Some(token)
        } else {
            None
        }
    }
	pub fn parser_expects(&self, expected_tokens: &[&str]) -> bool {
		for (i, expected_token) in expected_tokens.iter().enumerate() {
			if let Some(token) = self.peek_except(i as isize, expected_token) {
				if token != *expected_token {
					return false;
				}
			} else {
				return false;
			}
		}
		true
	}

	pub fn peek_stof(&self, offset: isize) -> Option<f64> {
		if let Some(token) = self.peek(offset) {
			match token.parse::<f64>() {
				Ok(num) => return Some(num),
				_ => eprintln!("Failed to parse float: {}", token),
			}
		}
		None
	}
	pub fn peek_stoi(&self, offset: isize) -> Option<i64> {
		if let Some(token) = self.peek(offset) {
			match token.parse::<i64>() {
				Ok(num) => return Some(num),
				_ => eprintln!("Failed to parse integer: {}", token),
			}
		}
		None
	}
	pub fn peek_type(&self, offset: isize) -> Type {
		if let Some(token) = self.peek(offset) {
			match token.as_str() {
				"int" => Type::Int(0),
				"float" => Type::Float(0.0),
				"double" => Type::Double(0.0), // Assuming doubles are also f64 and are distinguished elsewhere
				"string" => Type::String("".to_string()),
				"void" => Type::Void,
				// Add more cases for custom types as needed
				_ => Type::Unknown,
			}
		} else {
			panic!("Should have token at offset");
		}
	}
	
	pub fn peek_function(&self, token: &String) -> Option<&StdLibFn>{
		return self.sl.get(token);
	}

	pub fn peek_operator_priority(&self) -> bool{
		let mut offset = 1;
		for i in 1..4 {
			//println!("{:?}",i);
			if self.precedence(i) > 0{
				offset = i;
				break;
			}
		}
		self.precedence(0) < self.precedence(offset)
	}

	pub fn precedence(&self, offset: isize) -> i32{
		let token = match self.peek(offset) {
				Some(t) => t,
				None => return 0,
		};
		//println!("{:?}", token);
		match token.as_str() {
			"(" => 4,
			"^" => 3,
			"*" | "/" | "%" => 2,
			"+" | "-" | ")" => 1,
			_ => 0,
		}
	}

	pub fn is_variable_name(&self , offset: isize) -> bool{
		let mut chars = self.peek(offset).unwrap().chars();
		let ret = match chars.next() {
			Some(ch) => ch.is_ascii_alphanumeric() || ch == '_',
			None => false,
		} && chars.all(|ch| ch.is_ascii_alphanumeric() || ch == '_');
		ret
	}

	pub fn is_variable_string(&self , offset: isize) -> bool {
		let token = self.peek(offset).unwrap();
		if token.len() >= 2 && token.starts_with('"') && token.ends_with('"') {
			let mut prev_char = None;
			for (index, c) in token[1..].char_indices() {
				if index == token.len() - 2 {
					break;
				}
				if c == '"' && prev_char != Some('\\') {
					return false;
				}
				prev_char = Some(c);
			}
			true
		} else {
			false
		}
	}
	pub fn is_variable_function(&self, offset: isize) -> bool{
		let token = match self.peek(offset) {
			Some(t) => t,
			None => return false,
		};

		match self.sl.get(token){
			Some(_) => true,
			None => return false,
		}
	}

	pub fn is_variable_number(&self, offset: isize) -> bool {
		self.peek(offset).unwrap().parse::<i64>().is_ok() || self.peek(offset).unwrap().parse::<f64>().is_ok()
	}
	pub fn is_variable_float(&self, offset: isize) -> bool {
        self.peek(offset).unwrap().parse::<f64>().is_ok()
    }
	pub fn is_variable_int(&self, offset: isize) -> bool {
        self.peek(offset).unwrap().parse::<i64>().is_ok()
    }
	
	pub fn get_recursion(&mut self) -> i32{
        self.recursion_count
    }
    pub fn inc_recursion(&mut self) {
        self.recursion_count += 1;
    }
    pub fn dec_recursion(&mut self) {
        self.recursion_count -= 1;
    }

	pub fn get_parenthesis(&mut self) -> i32{
        self.parenthesis_count
    }
    pub fn inc_parenthesis(&mut self) {
        self.parenthesis_count += 1;
    }
    pub fn dec_parenthesis(&mut self) {
        self.parenthesis_count -= 1;
    }

	pub fn is_ok(&mut self){
		if self.parenthesis_count > 0 {
			panic!("Parenthesis not closed");
		}else if self.parenthesis_count < 0 {
			panic!("Parenthesis not Opened");
		}
	}

}

pub fn parse_expression(parser: &mut Parser, instructions: &mut Vec<Instruction>) {
	parser.inc_recursion();
	while let Some(cur_token) = parser.peek(0) {
		// println!("{:?}", cur_token);
		// println!("{:?}", instructions);
	
		match cur_token.as_str()  {
			"(" => {parser.inc_parenthesis(); parser.advance(); parse_expression(parser, instructions);},
			")" => {parser.dec_parenthesis(); parser.dec_recursion(); return;}
			"+" => {parse_pemdas(parser,instructions); instructions.push(Instruction::Add);},
			"-" => {parse_pemdas(parser,instructions); instructions.push(Instruction::Sub);},
			"*" => {parse_pemdas(parser,instructions); instructions.push(Instruction::Mul);},
			"/" => {parse_pemdas(parser,instructions); instructions.push(Instruction::Div);},
			"%" => {parse_pemdas(parser,instructions); instructions.push(Instruction::Modulo)},
			"^" => {parse_pemdas(parser,instructions); instructions.push(Instruction::Exp);},
			"=" => (),
			"\n" => {
				let mut should_return = false;
				if parser.get_recursion() > 1{
					should_return = true;
				}else if !instructions.is_empty(){
					match instructions.last().unwrap() {
						Instruction::End => (),
						_ => instructions.push(Instruction::End),
					}
				}
				if should_return{
					return;
				}
			},
			";" => {parser.dec_recursion(); return;},//Type defintion or end of function.
			"#" => parser.advance_comment(),
			"def" => parse_definition(parser),
			_ if parser.is_variable_number(0) => parse_value(parser, instructions, 0),
			_ if parser.is_variable_name(0) => parse_var(parser, instructions, 0),
			_ if parser.is_variable_string(0) => instructions.push(Instruction::Push(Type::String(cur_token.to_string()))),
			_ => (),//println!("Unknown token{:?}", cur_token),
		}
		parser.advance();
	}
	parser.dec_recursion();
	parser.is_ok();
}
pub fn parse_pemdas(parser: &mut Parser, instructions: &mut Vec<Instruction>){
	if parser.peek_operator_priority(){ // Check next operator Prority
		parser.advance();
		parse_expression(parser, instructions);
		
	}else{ // Grab the the next value. But this wont work with functions.
		parser.advance();
		parse_value(parser, instructions, 0);
	}
}

pub fn parse_value(parser: &mut Parser, instructions: &mut Vec<Instruction>, offset: isize){
	let cur_token = match parser.peek(offset){
		Some(token) => token,
		None => return,
	};
	let ret_instruction = match cur_token.as_str() {
		_ if parser.is_variable_int(offset) => Instruction::Push(Type::Int(parser.peek_stoi(offset).unwrap())),
		_ if parser.is_variable_float(offset) => Instruction::Push(Type::Float(parser.peek_stof(offset).unwrap())),
		_ if parser.is_variable_name(offset) => {parse_var(parser,instructions,offset); return;},
		_ => panic!("here"),
	};

	instructions.push(ret_instruction);
}

pub fn parse_var(parser: &mut Parser, instructions: &mut Vec<Instruction>, offset: isize){
	let cur_token = match parser.peek(offset){
		Some(token) => token.clone(),
		None => return,
	};
	
	let next_token = match parser.peek(offset+1){ //TODO! Probably dont want to manualy check for =
		Some(token) => token.clone(),
		None => {instructions.push(Instruction::Load(cur_token.clone(), Type::Unknown)); return;},
	};
	
	println!("{:?}", parser.peek(0));
	let ret_instruction = match next_token.as_str() {
		":" => {parser.advance(); Instruction::SetVar(cur_token.clone(), parser.peek_type(offset))},
		"=" => {parser.advance(); Instruction::SetVar(cur_token.clone(), Type::Unknown)}, //TODO! Set type at the end.
		"(" => parse_func(parser, &cur_token),
		_ => Instruction::Load(cur_token.clone(), Type::Unknown), // TODO! look up Type here to see if it exsists
	};

	instructions.push(ret_instruction);

}

pub fn parse_func(parser: &mut Parser, func : &String) -> Instruction{
	parser.inc_parenthesis();
	println!("{:?}",parser.custom);
	if parser.custom.contains_key(func){
		let mut args: Vec<Instruction> = vec![];
		parse_expression(parser, &mut args);
		return Instruction::CustomCall(func.clone(), mem::take(&mut args), vec![])
	}
	
	let mut args = vec![];
	parse_expression(parser, &mut args);

	return Instruction::Call(
        func.clone(),
		Type::String("Hello_world".to_string()),
		Function::new(standard_library::print),
	);
}

pub fn parse_definition(parser: &mut Parser){
	parser.advance();
	if parser.is_variable_name(0){
		let func = parser.peek(0).unwrap().to_owned();
		parser.advance();
		let mut args: Vec<Instruction> = vec![];
		parse_expression(parser, &mut args);
		println!("args{:?}",args);
		let mut instructions: Vec<Instruction> = vec![];
		parse_expression(parser, &mut instructions);
		println!("{:?}", instructions);
		parser.custom.insert(func, instructions);
		println!("{:?}",parser.custom);
	}
	//println!("{:?}",parser.peek(0).unwrap());
	// if(parser.is_variable_name(1)){
	// 	let func = parser.peek(1).unwrap();
		//let args = parser.grab_args();
		//let args_return = parser.grab_return();
		//let instructions = parser.parse_expression
	//}
}