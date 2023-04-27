
# Rust Quickstart Guide

This repository contains Rust code for [insert project name here]. Follow these steps to get started:

## Installation

1. Install Rust by following the instructions on the [Rust website](https://www.rust-lang.org/tools/install).
2. Clone this repository to your local machine.
3. Navigate to the cloned directory.
4. Run the project using the following command:
```cmd
cargo run
```

# Lexer
The lexer in this interpreter is responsible for breaking down the input string into tokens that can be processed by the parser. The lex function in main.rs performs this task.

The lexer works by iterating through the characters in the input string. For each character, it checks if it is a whitespace character or an operator. If it is a whitespace character, the current token is pushed onto the token vector and a new token is started. If it is an operator, the current token is pushed onto the token vector and the operator is added as a new token. Otherwise, the character is added to the current token.

The list of operators is defined in the operaters vector at the start of the lex function. It contains the following operators: +, -, *, /, %, :, (, ), ,, \n, \t, #, and ;.

The lexer returns a vector of strings, where each string represents a token identified by the lexer. The tokens can be processed by the parser to build an abstract syntax tree (AST) that represents the structure of the program.

If you want to modify or extend the lexer, you can modify the lex function in main.rs. You can add new operators to the operaters vector, or modify the logic for identifying tokens. However, be careful to ensure that the modifications do not break the lexer's ability to correctly tokenize input strings.

# Parser
The parser takes in a list of tokens from the lexer and produces a set of instructions that can be executed by the interpreter. The parser is implemented as a struct called Parser.

Fields
The Parser struct has the following fields:

tokens: a vector of strings representing the tokens produced by the lexer.
position: an index representing the current position in the token vector.
recursion_count: a counter that keeps track of the current recursion level.
parenthesis_count: a counter that keeps track of the current number of open parenthesis.
sl: a hashmap of standard library functions.
custom: a hashmap of custom functions defined by the user.

# Interpreter
It supports basic arithmetic operations such as addition, subtraction, multiplication, division, exponentiation, and modulo. The interpreter can also handle user-defined functions and variables, and supports custom function calls. It is implemented in Rust and uses a stack-based approach for evaluating expressions. The interpreter is designed to be extensible, allowing new functions and operations to be easily added.



# Notes
# Write Test Cases
Multiple variable declarations on one line, separated by commas: a = 1, b = 2, c = 3
Variable declarations with different types on one line: a = 1, b = "hello", c = 3.14
Variable declarations with no initial value: a = None
Re-assignment of an existing variable: a = 1, a = 2
Variables with names that are keywords in your language: if = 1
Variables with names that clash with built-in functions or other identifiers: print = 1
Variables with names that start with an underscore or contain special characters: _a = 1, a! = 1

### Optional type annotation
a: int = 1

### No type annotation
b = "hello"

### Type checking for integer
g: int = 1
h: int = "hello"  # Raises a type error

### Type checking for custom types
class CustomType:
    def __init__(self, value: int):
        self.value = value
        
i: CustomType = CustomType("hello")  # Raises a type error

### Type coercion
k: int = 1
l: float = k  # Coerces the integer 1 to a float
m: str = k  # Raises a type error

### Type annotation specific to external library
import numpy as np
n: np.ndarray = np.array([1, 2, 3])

### Syntax for declaring types:
q = p as str

# All tokens possible

import
from
class
def
if
elif
else
while
for
try
except
finally
with
pass
break
continue
return
raise
yield
async
await
global
nonlocal
assert
lambda
Identifiers:

Variable names (e.g., my_variable)
Function names (e.g., my_function)
Class names (e.g., MyClass)
Literals:

String literals (e.g., "Hello, World!", 'Hello, World!')
Integer literals (e.g., 42, 0b101010, 0o52, 0x2a)
Floating-point literals (e.g., 3.14, 1.2e-3)
Complex literals (e.g., 1+2j, 3.14j)
Boolean literals (e.g., True, False)
None literal (e.g., None)
Operators:

Arithmetic operators (+, -, *, /, //, %, **)
Comparison operators (==, !=, <, >, <=, >=)
Logical operators (and, or, not)
Bitwise operators (&, |, ^, ~, <<, >>)
Assignment operators (=, +=, -=, *=, /=, %=, **=, //=, &=, |=, ^=, <<=, >>=)
Keep in mind that this list is not exhaustive, as Python has many more language constructs. Also, note that comments and whitespace can precede any of these tokens. To parse Python code effectively, I recommend using an existing parsing library like python-syntax or lalrpop, which can handle these tokens and the parsing rules for you.