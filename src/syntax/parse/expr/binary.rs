use crate::error::ERROR_INDICATOR;
use crate::syntax::ast::Expression;
use crate::syntax::lexer::{KeywordMap, Token, TokenType, KEYWORDS};
use crate::syntax::parse::Parser;

impl Parser {
    pub fn parse_binary(&mut self) -> Expression {
        let mut to_eval: Vec<Token> = Vec::new();

        let mut keyword_error = false;
        let mut keyword_fault = TokenType::Null;
        let mut current_index = self.cursor + 1;

        to_eval.push(self.tokens[current_index - 1].clone());

        while let Some(next_token) = self.tokens.get(current_index) {
            if [
                TokenType::NumericLiteral,
                TokenType::Identifier,
                TokenType::Add,
                TokenType::Sub,
                TokenType::Mul,
                TokenType::Div,
                TokenType::EqEq, // Include DoubleEqual for truthy evaluation
            ]
            .contains(&next_token.token_type)
                || KeywordMap::get(&KEYWORDS, &next_token.lexeme).is_some()
            {
                if KEYWORDS.get(&next_token.lexeme).is_some() {
                    keyword_error = true;
                    keyword_fault = next_token.token_type;
                    println!("keyword: {:#?}", KEYWORDS.get(&next_token.lexeme))
                }
                if next_token.token_type == TokenType::Identifier {
                    to_eval.push(next_token.clone());
                    current_index += 1;
                } else {
                    to_eval.push(next_token.clone());
                    current_index += 1;
                }
            } else {
                break;
            }
        }

        let keyword_error_msg = format!(
            "{} \x1b[1mExpected ';' after expression, found keyword '{}'\x1b[0m",
            ERROR_INDICATOR,
            TokenType::to_string(keyword_fault),
        );

        self.cursor = current_index - 1;

        match keyword_error {
            false => {
                let res = Self::parse_expression(to_eval); // Modified to capture truthy result
                res
            }
            _ => {
                self.throw_error(self.tokens[1].line_num, keyword_error_msg);
                Expression::Float(0.0)
            }
        }
    }
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
