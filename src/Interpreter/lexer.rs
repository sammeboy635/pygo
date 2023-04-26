

pub fn lex(input: &str) -> Vec<String> {
    let operaters = vec!["+", "-", "*", "/", "%", ":", "(", ")", ",", "\n", "\t","#",";"];
    let mut tokens: Vec<String> = Vec::new();
    let mut current_token = String::new();

    for ch in input.chars() {
        if ch.is_ascii_whitespace() || ch == '\t' || ch == '\n' {
            if !current_token.is_empty() {
                tokens.push(current_token.drain(..).collect());
            }
            if ch == '\n' || ch == '\t'{
                tokens.push(ch.to_string());
            }
        } else if operaters.contains(&ch.to_string().as_str()) {
            if !current_token.is_empty() {
                tokens.push(current_token.drain(..).collect());
            }
            tokens.push(ch.to_string());
        } else {
            current_token.push(ch);
        }
    }
    if !current_token.is_empty() {
        tokens.push(current_token);
    }

    tokens
}

#[test]
fn test_lex() {
	let input = "test2 = 3 + (9 * (5 + 8)) * 3 / 5".to_string();

	let tokens = lex(&input);

	let expected_tokens = vec![
		"test2", "=", "3", "+", "(", "9", "*", "(", "5", "+", "8", ")", ")", "*", "3", "/", "5",
	];

	assert_eq!(tokens, expected_tokens);
}

#[test]
fn lex_test2(){
	//let input = "test2 = 3.0 + (9.0 * (5.0 + 8.0)) * 3.0 / 5.0".to_string();
	let input = "def here():\n\ttest3 = 5".to_string();
	let tokens = lex(&input);
	println!("{:?}",tokens);
}