use crate::error::ERROR_INDICATOR;
use crate::syntax::ast::Expression;
use crate::syntax::lexer::{KeywordMap, Token, TokenType, KEYWORDS};
use crate::syntax::parse::{ConditionType, Parser};

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
                TokenType::LParen,
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

        let mut index = 0;
        while index < tokens.len() {
            if tokens[index].token_type == TokenType::EqEq {
                let lhs_tokens = tokens[..index].to_vec();
                let rhs_tokens = tokens[(index + 1)..].to_vec();

                let lhs_expr = Self::parse_expression(lhs_tokens);
                let rhs_expr = Self::parse_expression(rhs_tokens);

                return Expression::Conditional {
                    lhs: Box::new(lhs_expr),
                    op: ConditionType::Equal,
                    rhs: Box::new(rhs_expr),
                };
            }
            index += 1;
        }

        while i < tokens.len() {
            match tokens[i].token_type {
                TokenType::LParen => {
                    ops_stack.push(TokenType::LParen);
                    expr_stack.push(Expression::Null);
                }
                TokenType::RParen => {
                    while let Some(&top_op) = ops_stack.last() {
                        if top_op == TokenType::LParen {
                            ops_stack.pop().unwrap();
                            break;
                        } else {
                            let rhs_expr = expr_stack.pop().unwrap();
                            let lhs_expr = expr_stack.pop().unwrap();
                            let op = ops_stack.pop().unwrap();
                            let new_expr = Expression::BinaryOp {
                                lhs: Box::new(lhs_expr),
                                op,
                                rhs: Box::new(rhs_expr),
                            };
                            expr_stack.push(new_expr);
                        }
                    }
                }
                TokenType::Add | TokenType::Sub | TokenType::Mul | TokenType::Div => {
                    while let Some(&top_op) = ops_stack.last() {
                        if Self::precedence(&top_op) >= Self::precedence(&tokens[i].token_type)
                            && top_op != TokenType::LParen
                        {
                            let rhs_expr = expr_stack.pop().unwrap();
                            let lhs_expr = expr_stack.pop().unwrap();
                            let op = ops_stack.pop().unwrap();
                            let new_expr = Expression::BinaryOp {
                                lhs: Box::new(lhs_expr),
                                op,
                                rhs: Box::new(rhs_expr),
                            };
                            expr_stack.push(new_expr);
                        } else {
                            break;
                        }
                    }
                    ops_stack.push(tokens[i].token_type.clone());
                }
                TokenType::Identifier => {
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

        while !ops_stack.is_empty() {
            let rhs_expr = expr_stack.pop().unwrap();
            let lhs_expr = expr_stack.pop().unwrap();
            let op = ops_stack.pop().unwrap();
            let new_expr = Expression::BinaryOp {
                lhs: Box::new(lhs_expr),
                op,
                rhs: Box::new(rhs_expr),
            };
            expr_stack.push(new_expr);
        }

        expr_stack.pop().unwrap()
    }

    fn precedence(op: &TokenType) -> i32 {
        match op {
            TokenType::Add | TokenType::Sub => 1,
            TokenType::Mul | TokenType::Div => 2,
            TokenType::LParen => 3,
            _ => 0,
        }
    }
}
