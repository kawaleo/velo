mod expr;
mod stmt;

use super::ast::Expression;
use super::ast::*;
use super::lexer::{KeywordMap, Token, TokenType, KEYWORDS};
use crate::error::{ErrorType::ParseError, VeloError, ERROR_INDICATOR};

use std::process;

#[derive(Debug)]
pub struct Parser {
    pub tokens: Vec<Token>,
    pub cursor: usize,
    pub nodes: Vec<Ast>,
    pub errors: Vec<VeloError>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            cursor: 0,
            nodes: Vec::new(),
            errors: Vec::new(),
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Ast>, VeloError> {
        while !self.tokens.is_empty() {
            match self.tokens[0].token_type {
                TokenType::Immut => {
                    self.variable_assignment(false, None, true, false);
                }
                TokenType::Function => self.function_declaration(),
                TokenType::Identifier => {
                    match self.tokens[1].token_type {
                        TokenType::ColonEq => {
                            self.variable_assignment(false, None, false, false);
                        }
                        TokenType::LParen => {
                            self.call_expr();
                        }
                        TokenType::Eq => unimplemented!(), // for not reassignment
                        _ => unimplemented!(),
                    }
                }
                TokenType::Import => {
                    self.import_path();
                }
                TokenType::Semicolon => {
                    self.tokens.remove(0);
                }
                TokenType::EOF => {
                    self.nodes.push(Ast::Expression(Expression::Null));
                    self.tokens.remove(0);
                }
                _ => {
                    println!(
                        "incomplete\n{:#?}\n{:#?}",
                        self.tokens[0].token_type,
                        self.tokens[0].lexeme.clone()
                    );
                    process::exit(1)
                }
            };
        }

        let mut ast_nodes = Vec::new();

        for node in &self.nodes {
            ast_nodes.push(node.clone())
        }
        if self.errors.len() > 0 {
            for error in self.errors.iter() {
                println!("{}", error.message);
                println!("  [filename goes here]:{}\n\n", error.line);
                println!("TODO: Potential Fixes");

                match error.error_type {
                    ParseError => println!("This error is found to be of type 'ParseError'"),
                    _ => unreachable!(),
                }
            }
            process::exit(1);
        }

        Ok(ast_nodes)
    }

    fn parse_literal(&mut self, token: Token, cursor: Option<usize>) -> Expression {
        match cursor.is_some() {
            true => self.cursor = cursor.unwrap(),
            _ => {}
        }
        match token.token_type {
            TokenType::True => Expression::Bool(true),
            TokenType::False => Expression::Bool(false),
            TokenType::String => Expression::StringLiteral(token.lexeme.clone()),
            TokenType::NumericLiteral | TokenType::Identifier => {
                if self.tokens.get(self.cursor + 1).is_some() {
                    self.parse_binary()
                } else {
                    let message = format!(
                        "{} \x1b[1mUnexpected EOF when parsing file",
                        ERROR_INDICATOR
                    );
                    self.throw_error(self.tokens[0].line_num, message);

                    Expression::Null
                }
            }
            _ => {
                let message = format!(
                    "{} \x1b[1mCannot assign items of type {:#?} to variables\x1b[0m",
                    ERROR_INDICATOR, token.token_type
                );
                self.throw_error(self.tokens[0].line_num, message);
                self.cursor = 0;

                Expression::Null
            }
        }
    }

    fn parse_binary(&mut self) -> Expression {
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

        /*
        match v {
            // Check if it's a float and return Float if so
            val if val.is_sign_positive() && val.fract() != 0.0 => {
                (Expression::Literal(Literal::Float(val)), Some(Type::Float))
            }
            // Check if it's a whole number and fits into i16
            val if val.fract() == 0.0 && (val as i16 as f32 == val) => (
                Expression::Literal(Literal::Short(val as i16)),
                Some(Type::Short),
            ),
            // Check if it's a whole number and fits into i32
            val if val.fract() == 0.0 && (val as i32 as f32 == val) => (
                Expression::Literal(Literal::Int(val as i32)),
                Some(Type::Int),
            ),
            // For values larger than i32 or with decimal parts, use i64 (Large)
            val if val.fract() == 0.0 && (val as i64 as f32 == val) => (
                Expression::Literal(Literal::Large(val as i64)),
                Some(Type::Large),
            ),
            _ => (Expression::Literal(Literal::Float(v)), Some(Type::Float)), // todo: throw error
        }
        */

        let keyword_error_msg = format!(
            "{} \x1b[1mExpected ';' after expression, found keyword '{}'\x1b[0m",
            ERROR_INDICATOR,
            TokenType::to_string(keyword_fault),
        );

        self.cursor = current_index - 1;

        match keyword_error {
            false => {
                let res = Self::parse_expression(to_eval);
                res
            }
            _ => {
                self.throw_error(self.tokens[1].line_num, keyword_error_msg);
                return Expression::Float(0.0);
            }
        }
    }

    fn import_path(&mut self) {
        let mut path = String::new();
        self.cursor += 1;
        if let Some(token) = self.tokens.get(self.cursor) {
            if token.token_type == TokenType::String {
                self.cursor += 1;
                path = token.lexeme.clone();
            }
        }
        self.tokens.drain(0..self.cursor);
        self.cursor = 0;
        self.nodes.push(Ast::Statement(Statement::Import(path)));
    }

    pub fn throw_error(&mut self, line: usize, message: String) {
        self.errors
            .push(VeloError::error(line, &message, ParseError));
    }
}
