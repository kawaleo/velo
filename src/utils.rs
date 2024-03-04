use crate::runtime::environment::Environment;
use crate::syntax::ast::Expression;
use std::path::PathBuf;

#[allow(deprecated)]
pub fn expand_tilde(path: &str) -> PathBuf {
    if path.starts_with('~') {
        if let Some((home, rest)) = std::env::home_dir().and_then(|h| Some((h, &path[1..]))) {
            return [home.to_str().unwrap(), rest].iter().collect();
        }
    }
    path.into()
}

pub fn interpolate_string(input: &str, env: &Environment) -> String {
    let mut result = String::new();
    let mut src: Vec<char> = input.chars().collect();

    while !src.is_empty() {
        match src[0] {
            '$' => {
                if src[1] == '{' {
                    src.remove(0);
                    src.remove(0);
                    let mut var_name = String::new();
                    while src[0] != '}' {
                        var_name.push(src.remove(0))
                    }
                    src.remove(0);

                    if let Some(var) = env.variables.get(&var_name) {
                        match var {
                            Expression::StringLiteral(str) => result.push_str(str),
                            //TODO: Actually evaluate the expression
                            Expression::BinaryOp {
                                lhs: _,
                                op: _,
                                rhs: _,
                            } => result.push_str(&format!("{:#?}", var)),
                            Expression::Float(val) => result.push_str(&format!("{:#?}", val)),
                            _ => todo!(),
                        }
                    } else {
                        eprintln!("Cannot locate variable `{}`", var_name);
                        std::process::exit(1)
                    }
                } else {
                    result.push(src.remove(0))
                }
            }
            _ => result.push(src.remove(0)),
        }
    }

    result
}
