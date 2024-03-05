use super::super::Parser;
use crate::syntax::ast::{Ast, Statement};
use crate::syntax::lexer::{Token, TokenType};

impl Parser {
    pub fn if_statement(&mut self) {
        let (condition_tokens, body_tokens) = self.collect_if_tokens();
        self.parse_if_statement(condition_tokens, body_tokens);
        self.reset_cursor();
    }

    fn collect_if_tokens(&mut self) -> (Vec<Token>, Vec<Token>) {
        let mut condition_tokens = Vec::new();
        self.cursor += 1;

        while self.tokens[self.cursor].token_type != TokenType::LBrace {
            condition_tokens.push(self.tokens[self.cursor].clone());
            self.cursor += 1;
        }

        // {
        self.cursor += 1;

        let mut body_tokens = Vec::new();
        while self.tokens[self.cursor].token_type != TokenType::RBrace {
            body_tokens.push(self.tokens[self.cursor].clone());
            self.cursor += 1;
        }

        self.cursor += 1;

        (condition_tokens, body_tokens)
    }

    fn parse_if_statement(&mut self, condition_tokens: Vec<Token>, body_tokens: Vec<Token>) {
        let condition = Self::parse_expression(condition_tokens);
        let body = Parser::new(body_tokens)
            .parse()
            .expect("Error parsing if statement");

        let statement = Statement::IfStatement { condition, body };
        self.nodes.push(Ast::Statement(statement));
    }
}
