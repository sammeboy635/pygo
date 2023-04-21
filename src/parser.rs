use crate::interpreter::{Instruction, Type};
pub struct Parser {
    tokens: Vec<String>,
    position: usize,
    recursion_count: i32,
    parenthesis_count: i32,
}

impl Parser {
    pub fn new(tokens: Vec<String>) -> Self {
        Parser {
            tokens,
            position: 0,
            recursion_count: 0,
            parenthesis_count: 0,
        }
    }

    pub fn advance(&mut self) -> Option<&String> {
		self.position += 1;
        self.peek(0)
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

    pub fn peek_except(&self, offset: isize, exception: &str) -> Option<&String> {
        let token = self.peek(offset)?;
        if token != exception {
            Some(token)
        } else {
            None
        }
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
	pub fn is_variable_number(&self, offset: isize) -> bool {
		self.peek(offset).unwrap().parse::<i64>().is_ok() || self.peek(offset).unwrap().parse::<f64>().is_ok()
	}
	pub fn is_variable_float(&self, offset: isize) -> bool {
        self.peek(offset).unwrap().parse::<i64>().is_ok()
    }
	pub fn is_variable_int(&self, offset: isize) -> bool {
        self.peek(offset).unwrap().parse::<f64>().is_ok()
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
	while let Some(cur_token) = parser.peek(0) {
		match cur_token.as_str()  {
			"(" => {parser.inc_parenthesis(); parser.advance(); parse_expression(parser, instructions);},
			")" => {parser.dec_parenthesis(); return;}
			"+" | "-" | "*" | "/" | "%" | "^" => parse_pemdas(parser,instructions, 0),
			"=" => (),
			_ if parser.is_variable_number(0) => parse_value(parser, instructions, 0),
			_ if parser.is_variable_name(0) => parse_var(parser, instructions, 0),
			_ if parser.is_variable_string(0) => (),
			_ => (),//println!("Unknown token{:?}", cur_token),
		}
		parser.advance();
	}
	parser.is_ok();
}
pub fn parse_pemdas(parser: &mut Parser, instructions: &mut Vec<Instruction>, offset: isize){
	let cur_token = match parser.peek(offset){
		Some(token) => token.to_owned(),
		None => return,
	};
	//println!("{:?}", parser.peek_operator_priority());
	if parser.peek_operator_priority(){
		{
			parser.advance();
			parse_expression(parser, instructions);
		}
	}else{
		parser.advance();
		parse_value(parser, instructions, 0);
		//println!("{:?}", instructions);
	}

	match cur_token.as_str() {
		"+" => instructions.push(Instruction::Add),
		"-" => instructions.push(Instruction::Sub),
		"*" => instructions.push(Instruction::Mul),
		"/" => instructions.push(Instruction::Div),
		"%" => instructions.push(Instruction::Modulo),
		"^" => instructions.push(Instruction::Exp),
		_ => unimplemented!("{:?}", cur_token)
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
		_ =>  unimplemented!("{:?}", cur_token),
	};

	instructions.push(ret_instruction);
}
pub fn parse_var(parser: &mut Parser, instructions: &mut Vec<Instruction>, offset: isize){
	let cur_token = match parser.peek(offset){
		Some(token) => token,
		None => return,
	};
	let next_token = match parser.peek(offset+1){ //TODO! Probably dont want to manualy check for =
		Some(token) => token,
		None => return,
	};

	let ret_instruction = match cur_token.as_str() {
		_ if next_token == "=" => Instruction::SetVar(cur_token.clone(), Type::Unknown), //TODO! Set type at the end.
		_ => Instruction::Load(cur_token.clone(), Type::Unknown), // TODO! look up Type here to see if it exsists
	};

	instructions.push(ret_instruction);
}
