use crate::syntax::ast::Expression;
use crate::syntax::lexer::{Token, TokenType};
use crate::syntax::parse::Parser;

impl Parser {
    pub fn parse_expression(tokens: Vec<Token>) -> Expression {
        let mut ops_stack: Vec<TokenType> = Vec::new();
        let mut expr_stack: Vec<Expression> = Vec::new();
        let mut i = 0;

        while i < tokens.len() {
            match tokens[i].token_type {
                TokenType::Add | TokenType::Sub | TokenType::Mul | TokenType::Div => {
                    while let Some(&top_op) = ops_stack.last() {
                        if Self::precedence(&top_op) >= Self::precedence(&tokens[i].token_type) {
                            // Pop the top operator from the stack and apply it to the operands
                            let rhs_expr = expr_stack.pop().unwrap();
                            let lhs_expr = expr_stack.pop().unwrap();
                            let op = ops_stack.pop().unwrap();
                            let new_expr = Expression::BinaryOp {
                                lhs: Box::new(lhs_expr),
                                op,
                                rhs: Box::new(rhs_expr),
                            };
                            // Push the result back to the expression stack
                            expr_stack.push(new_expr);
                        } else {
                            break;
                        }
                    }
                    // Push the current operator to the stack
                    ops_stack.push(tokens[i].token_type.clone());
                }
                TokenType::Identifier => {
                    // We are adding a variable
                    let num = tokens[i].lexeme.clone();
                    expr_stack.push(Expression::Identifier(num));
                }
                _ => {
                    let num = tokens[i].lexeme.clone().parse::<f32>();
                    if num.is_ok() {
                        expr_stack.push(Expression::Float(num.unwrap()));
                    }
                }
            }
            i += 1;
        }

        // Pop any remaining operators from the stack and apply them
        while let Some(op) = ops_stack.pop() {
            let rhs_expr = expr_stack.pop().unwrap();
            let lhs_expr = expr_stack.pop().unwrap();
            let new_expr = Expression::BinaryOp {
                lhs: Box::new(lhs_expr),
                op,
                rhs: Box::new(rhs_expr),
            };
            expr_stack.push(new_expr);
        }

        // The result should be the last expression left on the stack
        expr_stack.pop().unwrap()
    }

    // Function to determine precedence of operators
    fn precedence(op: &TokenType) -> i32 {
        match op {
            TokenType::Add | TokenType::Sub => 1,
            TokenType::Mul | TokenType::Div => 2,
            _ => 0, // Parentheses don't have precedence in this implementation
        }
    }
}
