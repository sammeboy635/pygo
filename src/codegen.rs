use crate::ast::{BinaryOp, Expr, Literal};
use crate::interpreter::{Instruction};
use std::collections::HashMap;
pub enum Type {
    Integer,
    Float,
    String,
    Unknown,
}
pub fn generate_code(expr: &Expr, variables: &HashMap<String, Type>) -> Vec<Instruction> {
    match expr {
        Expr::BinaryOp { op, left, right } => {
            let mut code = generate_code(left, variables);
            code.extend(generate_code(right, variables));
            let op_code = match op {
                BinaryOp::Plus => Instruction::Add,
                BinaryOp::Minus => Instruction::Sub,
                BinaryOp::Multiply => Instruction::Mul,
                BinaryOp::Divide => Instruction::Div,
            };
            code.push(op_code);
            code
        }
        Expr::Literal(literal) => {
            let value = match literal {
                Literal::Integer(n) => Instruction::PushInt(*n),
                Literal::Float(f) => Instruction::PushFloat(*f),
                Literal::String(s) => Instruction::PushStr(s.clone()),
            };
            vec![value]
        }
        Expr::Identifier(identifier) => {
            let var_type = match variables.get(identifier) {
                Some(Type::Integer) => Instruction::LoadInt(identifier.clone()),
                Some(Type::Float) => Instruction::LoadFloat(identifier.clone()),
                Some(Type::String) => Instruction::LoadStr(identifier.clone()),
                None | Some(Type::Unknown) => panic!("Undefined or unknown type variable: {}", identifier),
            };
            vec![var_type]
        }
    }
}