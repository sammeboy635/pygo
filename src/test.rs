macro_rules! parser {
    ($($name:ident -> $pattern:pat => $action:expr),*$(,)?) => {
        fn parse(input: &str) -> Result<String, String> {
            let rules: Vec<Box<dyn Fn(&str) -> Option<String>>> = vec![
                $(Box::new(|input: &str| {
                    if let $pattern = input {
                        Some($action(input))
                    } else {
                        None
                    }
                })),*
            ];

            for rule in rules {
                if let Some(result) = rule(input) {
                    return Ok(result);
                }
            }

            Err(format!("No matching rule found for input '{}'", input))
        }
    };
}

parser! {
    statement -> "let" | "const" => |input| format!("Declaration: {}", input),
    expression -> "42" => |_| "The answer to life, the universe, and everything".to_owned(),
}
#[test]
fn main() {
    let input = "let";
    let result = parse(input);

    match result {
        Ok(value) => {
            println!("Value: {}", value);
        }
        Err(e) => println!("Error: {}", e),
    }
}