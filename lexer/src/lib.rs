pub mod tokenizer;
mod token_macro;
pub mod pre_parser;



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {

        let mut tok = tokenizer::Tokenizer::new("tmp\\test_1.py");
		let mut tokens = vec![];
		tok.tokenize(&mut tokens);
    }
}
