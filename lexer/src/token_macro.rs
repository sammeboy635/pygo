use ast::token::{PygoToken, Float32};

#[macro_export]
macro_rules! is_word_break_char {
    ($c:expr) => {
        matches!(
            $c,
            ' ' | '\t' | '\n' | '\r' | '\x0c' | '(' | ')' | '[' | ']' | '{' | '}' | ',' | ';' | ':' | '.' | '#' |
            '+' | '-' | '*' | '/' | '%' | '^' | '&' | '|' | '~' | '<' | '>' | '=' | '!'
        )
    };
}

#[macro_export]
macro_rules! is_op {
    ($op:expr) => {
        match $op.as_ref() {
            "+" => Some(PygoToken::ADDITION),
            "-" => Some(PygoToken::SUBTRACTION),
            "*" => Some(PygoToken::MULTIPLICATION),
            "/" => Some(PygoToken::DIVISION),
            "//" => Some(PygoToken::FLOOR_DIVISION),
            "%" => Some(PygoToken::MODULO),
            "**" => Some(PygoToken::EXPONENT),
            "==" => Some(PygoToken::EQUALITY),
            "!=" => Some(PygoToken::INEQUALITY),
            "<" => Some(PygoToken::LESS_THAN),
            ">" => Some(PygoToken::GREATER_THAN),
            "<=" => Some(PygoToken::LESS_THAN_OR_EQUAL_TO),
            ">=" => Some(PygoToken::GREATER_THAN_OR_EQUAL_TO),
            "and" => Some(PygoToken::LOGICAL_AND),
            "or" => Some(PygoToken::LOGICAL_OR),
            "not" => Some(PygoToken::LOGICAL_NOT),
            "&" => Some(PygoToken::BITWISE_AND),
            "|" => Some(PygoToken::BITWISE_OR),
            "^" => Some(PygoToken::BITWISE_XOR),
            "~" => Some(PygoToken::BITWISE_NOT),
            "<<" => Some(PygoToken::LEFT_SHIFT),
            ">>" => Some(PygoToken::RIGHT_SHIFT),
            "=" => Some(PygoToken::ASSIGNMENT),
            "+=" => Some(PygoToken::ADDITION_ASSIGNMENT),
            "-=" => Some(PygoToken::SUBTRACTION_ASSIGNMENT),
            "*=" => Some(PygoToken::MULTIPLICATION_ASSIGNMENT),
            "/=" => Some(PygoToken::DIVISION_ASSIGNMENT),
            "//=" => Some(PygoToken::FLOOR_DIVISION_ASSIGNMENT),
            "%=" => Some(PygoToken::MODULO_ASSIGNMENT),
            "**=" => Some(PygoToken::EXPONENTIATION_ASSIGNMENT),
            "&=" => Some(PygoToken::BITWISE_AND_ASSIGNMENT),
            "|=" => Some(PygoToken::BITWISE_OR_ASSIGNMENT),
            "^=" => Some(PygoToken::BITWISE_XOR_ASSIGNMENT),
            "<<=" => Some(PygoToken::LEFT_SHIFT_ASSIGNMENT),
            ">>=" => Some(PygoToken::RIGHT_SHIFT_ASSIGNMENT),
            _ => None,
        }
    };
}
#[macro_export]
macro_rules! is_keyword {
    ($cur_str:expr) => {
        match $cur_str.as_ref() {
            "import" => Some(PygoToken::IMPORT),
            "from" => Some(PygoToken::FROM),
            "class" => Some(PygoToken::CLASS),
            "def" => Some(PygoToken::DEF),
            "if" => Some(PygoToken::IF),
            "elif" => Some(PygoToken::ELIF),
            "else" => Some(PygoToken::ELSE),
            "while" => Some(PygoToken::WHILE),
            "for" => Some(PygoToken::FOR),
            "try" => Some(PygoToken::TRY),
            "except" => Some(PygoToken::EXCEPT),
            "finally" => Some(PygoToken::FINALLY),
            "with" => Some(PygoToken::WITH),
            "pass" => Some(PygoToken::PASS),
            "break" => Some(PygoToken::BREAK),
            "continue" => Some(PygoToken::CONTINUE),
            "return" => Some(PygoToken::RETURN),
            "raise" => Some(PygoToken::RAISE),
            "yield" => Some(PygoToken::YIELD),
            "async" => Some(PygoToken::ASYNC),
            "await" => Some(PygoToken::AWAIT),
            "global" => Some(PygoToken::GLOBAL),
            "nonlocal" => Some(PygoToken::NONLOCAL),
            "assert" => Some(PygoToken::ASSERT),
            "lambda" => Some(PygoToken::LAMBDA),
            _ => None,
        }
    };
}
#[macro_export]
macro_rules! is_literal {
    ($cur_str:expr) => {
        match $cur_str.as_ref() {
            "True" => Some(PygoToken::BOOLEAN_LITERAL(true)),
            "False" => Some(PygoToken::BOOLEAN_LITERAL(false)),
            "None" => Some(PygoToken::NONE_LITERAL),
            _ => None,
        }
    };
}