#![allow(unused)]
use super::super::Parser;
use crate::error::ERROR_INDICATOR;
use crate::syntax::ast::{Ast, Expression, Statement};
use crate::syntax::lexer::{Token, TokenType, Type};

impl Parser {
    pub fn variable_assignment(
        &mut self,
        ret: bool,
        cursor: Option<usize>,
        mk_const: bool,
        in_fn: bool,
    ) -> Option<Statement> {
        let mut name = self.parse_var_name(mk_const, in_fn);

        let mut infer_type = true;

        if cursor.is_some() {
            self.cursor = cursor.unwrap();
        } else {
            self.cursor = self.cursor;
        }

        self.cursor += 2; // Move cursor past '=' and literal_index

        let mut final_type = Type::Void;

        //FIX ME: parse_literal panics when type for tuple is invalid
        //i.e: x: (this)
        //above panics ^^^
        let mut value = Expression::Null;
        let tok = match in_fn {
            true => self.tokens[self.cursor - 1].clone(),
            _ => self.tokens[self.cursor].clone(),
        };

        if in_fn {
            self.cursor -= 1
        }

        if let Some(next_token) = self.tokens.get(self.cursor) {
            match next_token.token_type {
                TokenType::Identifier => {
                    if let Some(next_next_token) = self.tokens.get(self.cursor + 1) {
                        match next_next_token.token_type {
                            TokenType::LParen => value = self.call_expr_as_var(),
                            _ => value = self.parse_literal(tok, Some(self.cursor)),
                        }
                    }
                }
                _ => value = self.parse_literal(tok, Some(self.cursor)),
            }
        }
        let message = format!(
            "{} \x1b[1mExpected semicolon following variable '{}', found {}\x1b[0m",
            ERROR_INDICATOR,
            name,
            self.tokens[self.cursor].lexeme.clone()
        );

        let variable = Statement::VariableAssignment {
            constant: mk_const,
            name,
            value,
        };

        let mut res = None;

        match ret {
            false => self.nodes.push(Ast::Statement(variable)),
            true => res = Some(variable),
        }
        if !in_fn {
            self.reset_cursor()
        }

        res
    }
    pub fn parse_var_name(&mut self, is_const: bool, in_fn: bool) -> String {
        if !is_const {
            let name = self.tokens[self.cursor].lexeme.clone();
            name
        } else {
            let name = self.tokens[1].lexeme.clone(); // todo handle case where no name
            self.cursor += 1;
            name
        }
    }
}
