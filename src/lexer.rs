use regex::Regex;

pub fn lex2(input: &str, operators: &Vec<&str>) -> Vec<String> {
    let mut tokens: Vec<String> = Vec::new();

    // Construct regular expression from list of operators
    let mut op_regex = String::new();
    for op in operators {
        op_regex.push_str(&format!("{}|", regex::escape(op)));
    }
    op_regex.pop(); // Remove trailing "|"

    // Update the format! string to include the op_regex correctly
    let re_str = format!(
        "([a-zA-Z0-9_]+)|([=,])|({})|(\"[^\"]*\")",
        op_regex
    );
    let re = Regex::new(&re_str).unwrap();

    // Iterator for finding matches
    let mut it = re.find_iter(input);

    while let Some(matched) = it.next() {
        // Add matched token to list of tokens
        let token = matched.as_str().to_string();
        tokens.push(token);
    }

    tokens
}

pub fn lex(input: &str, operators: &Vec<&str>) -> Vec<String> {
    let mut tokens: Vec<String> = Vec::new();
    let mut current_token = String::new();
    let input_chars: Vec<char> = input.chars().collect();

    for &ch in input_chars.iter() {
        if ch.is_whitespace() {
            if !current_token.is_empty() {
                tokens.push(current_token.clone());
                current_token.clear();
            }
        } else if operators.contains(&ch.to_string().as_str()) {
            if !current_token.is_empty() {
                tokens.push(current_token.clone());
                current_token.clear();
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lex() {
        let operators = vec!["+", "-", "*", "/", "%", ":", "(", ")"];
        let input = "test2 = 3 + (9 * (5 + 8)) * 3 / 5".to_string();

        let tokens = lex(&input, &operators);

        let expected_tokens = vec![
            "test2", "=", "3", "+", "(", "9", "*", "(", "5", "+", "8", ")", ")", "*", "3", "/", "5",
        ];

        assert_eq!(tokens, expected_tokens);
    }
}