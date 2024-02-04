use super::environment::Environment;
use super::eval::expr::*;
use crate::{
    syntax::ast::{Ast, Expression, Statement},
    utils::interpolate_string,
};

// TODO: Create a separate call expr function that just returns an expression
// I'll use it when parsing variables probably
pub fn evaluate(nodes: Vec<Ast>) {
    let mut env = Environment::init();
    for node in nodes {
        match node {
            Ast::Expression(expr) => match expr {
                Expression::CallExpr {
                    ref name,
                    ref params,
                } => eval_call_expr(&expr, &mut env, None),
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
                        eval_call_expr(&value, &mut env, Some(&stmt))
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
                _ => todo!(),
            },
        }
    }
    println!("\n{:#?}", env)
}
