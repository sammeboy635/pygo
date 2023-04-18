
use std::string::String;
use TokenType::*;
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
    Go,
    Async,
    For,
    Equal,
    Or,
    And,
    Not,
    Function,
    Var,
    In,
    Plus,
    Minus,
    Multiply,
    Divide,
    Integer,
    Float,
    String,
    Identifier,
    Unknown,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
    pub start_pos: usize,
    pub end_pos: usize,
}



pub struct Lexer {
    input: std::string::String,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.to_string(),
        }
    }

    pub fn lex(&self) -> Vec<Token> {
        let mut tokens = vec![];
        let words: Vec<&str> = self.input.split_whitespace().collect();
        let mut current_pos = 0;

        for word in words {
            let start_pos = self.input[current_pos..].find(word).unwrap() + current_pos;
            let end_pos = start_pos + word.len() - 1;

            let token_type = match word {
                "go" => Go,
                "async" => Async,
                "for" => For,
                "=" => Equal,
                "||" | "or" => Or,
                "&&" | "and" => And,
                "!" | "not" => Not,
                "def" => Function,
                "var" => Var,
                "in" => In,
                "+" => Plus,
                "-" => Minus,
                "*" => Multiply,
                "/" => Divide,
                _ if word.parse::<i32>().is_ok() => Integer,
                _ if word.parse::<f64>().is_ok() => Float,
                _ if word.starts_with("\"") && word.ends_with("\"") => String,
                _ => Identifier,
            };

            tokens.push(Token {
                token_type,
                value: word.to_string(),
                start_pos,
                end_pos,
            });

            current_pos = end_pos + 1;
        }

        tokens
    }
}

pub fn test_lexer() -> Vec<Token>{
	let input = "go async for = || && ! func var in + - * / 42 3.14 \"string\" identifier";
    let lexer = Lexer::new(input);
    let tokens = lexer.lex();
    for token in &tokens {
        println!("{:?}", token);
    }
	return tokens;
}