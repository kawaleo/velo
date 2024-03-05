use super::environment::Environment;
use super::eval::expr::*;
use crate::{
    syntax::ast::{Ast, Expression, Statement},
    syntax::{lexer::Lexer, parse::Parser},
    utils::{expand_tilde, interpolate_string},
};

pub fn evaluate(nodes: Vec<Ast>, debug: bool, env: &mut Environment) {
    for node in nodes {
        match node {
            Ast::Expression(expr) => match expr {
                Expression::CallExpr {
                    name: _,
                    params: _,
                    /*
                    ref name,
                    ref params,
                    */
                } => eval_call_expr(&expr, env, None),
                Expression::Null => {}
                _ => unimplemented!(), // sticking out your gyat
            },
            Ast::Statement(stmt) => match stmt {
                Statement::VariableAssignment {
                    constant,
                    ref name,
                    ref value,
                } => match value {
                    Expression::CallExpr { name: _, params: _ } => {
                        eval_call_expr(&value, env, Some(&stmt))
                    }
                    #[allow(unused)]
                    Expression::BinaryOp { lhs, op, rhs } => {
                        let eval = evaluate_binary(&value, env);
                        env.declare_variable(
                            name.to_string(),
                            Expression::Float(eval.clone()),
                            constant,
                        );
                    }
                    #[allow(unused)]
                    Expression::Conditional { lhs, op, rhs } => {
                        let eval = evaluate_conditional(&value, env);
                        env.declare_variable(name.to_string(), eval.clone(), constant);
                    }
                    _ => {
                        let v = match value {
                            Expression::StringLiteral(str) => {
                                let parsed = interpolate_string(str, &env);
                                Expression::StringLiteral(parsed)
                            }
                            _ => value.clone(),
                        };
                        env.declare_variable(name.to_string(), v.clone(), constant);
                    }
                },
                Statement::Import(path) => {
                    println!("Importing file: {}", path);
                    let full_path = expand_tilde(&path);

                    let contents = std::fs::read_to_string(&full_path).unwrap();
                    let mut lexer = Lexer::new(&contents);
                    let tokens = lexer.tokenize().tokens;

                    let mut parser = Parser::new(tokens);
                    let _ = parser.parse();

                    evaluate(parser.nodes, debug, env);
                    continue;
                }
                _ => todo!(),
            },
        }
    }
    if debug {
        println!("\n{:#?}", env);
    } else {
        println!("\0")
    }
}
