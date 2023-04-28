fn precedence(op: char) -> i32 {
    match op {
        '+' | '-' => 1,
        '*' | '/' => 2,
        '^' => 3,
        _ => -1,
    }
}

fn to_postfix(expr: &str) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    let mut operators: Vec<char> = Vec::new();
    let mut number = String::new();

    for ch in expr.chars() {
        if ch.is_digit(10) {
            number.push(ch);
        } else {
            if !number.is_empty() {
                output.push(number);
                number = String::new();
            }
            if ch == '(' {
                operators.push(ch);
            } else if ch == ')' {
                while let Some(top_op) = operators.pop() {
                    if top_op == '(' {
                        break;
                    }
                    output.push(top_op.to_string());
                }
            } else if "+-*/^".contains(ch) {
                while let Some(&top_op) = operators.last() {
                    if top_op == '(' || precedence(ch) > precedence(top_op) {
                        break;
                    }
                    output.push(operators.pop().unwrap().to_string());
                }
                operators.push(ch);
            }
        }
    }

    if !number.is_empty() {
        output.push(number);
    }

    while let Some(op) = operators.pop() {
        output.push(op.to_string());
    }

    output
}
#[test]
fn main() {
    let test = "0 - 1 * (2 - 3) * 4";
    println!("{:?}", to_postfix(test));
}