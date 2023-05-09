use ast::token::PygoToken;

pub fn pre_parser(token : &mut Vec<PygoToken>) -> Option<Vec<PygoToken>> {
	let mut tokens = token.iter().peekable();
	let mut output: Vec<PygoToken> = Vec::new();
	let mut operators: Vec<PygoToken> = Vec::new();
	//let mut temp_variable: Option<PygoToken> = None;
	let mut func_paren_count: usize = 0;
	let mut inside_paren_count: usize = 0;
	let mut bracket = false;
	while let Some(token) = tokens.next() {
		if bracket{
			if *token == PygoToken::CLOSED_BRACKET{
				bracket = false;
			}
			output.push(token.clone());
			continue;
		}
		match token {
			PygoToken::OPEN_BRACKET => {
				output.push(token.clone());
				bracket = true},
			PygoToken::FUNCTION_NAME(_) => {
				output.push(token.clone());
				func_paren_count += 1;
			}
			PygoToken::INTEGER_LITERAL(_) | PygoToken::FLOATING_POINT_LITERAL(_) | PygoToken::STRING_LITERAL(_) | PygoToken::BOOLEAN_LITERAL(_) |
			 PygoToken::VARIABLE_NAME(_) | PygoToken::LITERAL(_) => {
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