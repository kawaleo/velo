use super::super::environment::Environment;
use crate::syntax::ast::{ConditionType, Expression, Statement};
use crate::syntax::lexer::TokenType;
use crate::utils::interpolate_string;

use std::io;

#[allow(unused)]
pub fn eval_call_expr(call_expr: &Expression, env: &mut Environment, var: Option<&Statement>) {
    let mut nm = String::new();
    let mut pm: Vec<Expression> = Vec::new();

    match call_expr {
        Expression::CallExpr { name, params } => {
            nm = name.to_string();
            pm = params.to_vec();
        }
        _ => unreachable!(),
    }
    let mut is_lib = false;

    let name = nm.clone();
    let params = pm.clone();

    for lib in env.lib_functions.iter() {
        if name == lib.name {
            is_lib = true
        } else {
            continue;
        }
    }

    if is_lib {
        match name.as_str() {
            "print" | "println" => {
                let mut line = false;
                if name.as_str() == "println" {
                    line = true
                }
                match &params[0] {
                    Expression::Identifier(ident) => {
                        if let Some(expr) = env.variables.get(ident) {
                            match expr {
                                Expression::StringLiteral(str) => {
                                    if line {
                                        let interpolated = interpolate_string(str, env);
                                        println!("{}", interpolated)
                                    } else {
                                        let interpolated = interpolate_string(str, env);
                                        print!("{}", interpolated)
                                    }
                                }
                                _ => {
                                    println!("{:#?}", expr);
                                    todo!()
                                }
                            }
                        } else {
                            std::process::exit(1)
                        }
                    }
                    Expression::StringLiteral(ident) => {
                        if line {
                            let interpolated = interpolate_string(ident, env);
                            println!("{}", interpolated)
                        } else {
                            let interpolated = interpolate_string(ident, env);
                            print!("{}", interpolated)
                        }
                    }
                    _ => todo!(),
                }
            }
            "input" => {
                if var.is_none() {
                    println!("`input` requires a variable to store to");
                }
                let mut buffer = String::new();
                io::stdin()
                    .read_line(&mut buffer)
                    .expect("Failed to read line");
                let buffer = buffer.trim().to_string();

                let var = var.unwrap();
                match var {
                    Statement::VariableAssignment {
                        constant,
                        name,
                        value: _,
                    } => env.declare_variable(
                        format!("{}", &name),
                        Expression::StringLiteral(buffer),
                        *constant,
                    ),
                    _ => {
                        println!("Idk how this error happens, but if someone gets it, explain what you did please");
                        std::process::exit(1)
                    }
                };
            }
            _ => unimplemented!(),
        }
    }
}

pub fn evaluate_binary(expr: &Expression, env: &Environment) -> f32 {
    match expr {
        Expression::Float(val) => *val,
        Expression::Identifier(val) => match env.variables.get(val) {
            Some(Expression::Float(val)) => *val,
            _ => todo!(),
        },
        Expression::BinaryOp { lhs, op, rhs } => {
            let lhs = evaluate_binary(lhs, env);
            let rhs = evaluate_binary(rhs, env);
            match op {
                TokenType::Add => lhs + rhs,
                TokenType::Sub => lhs - rhs,
                TokenType::Mul => lhs * rhs,
                TokenType::Div => lhs / rhs,
                _ => unreachable!(),
            }
        }
        _ => {
            println!("{:#?}", expr);
            unreachable!()
        }
    }
}

pub fn evaluate_conditional(expr: &Expression, env: &Environment) -> Expression {
    match expr {
        Expression::Conditional { lhs, op, rhs } => {
            let lhs = evaluate_binary(lhs, env);
            let rhs = evaluate_binary(rhs, env);
            match op {
                ConditionType::Equal => Expression::Bool(lhs == rhs),
                ConditionType::NotEqual => Expression::Bool(lhs != rhs),
                ConditionType::Unary => todo!(),
            }
        }
        _ => unreachable!(),
    }
}
