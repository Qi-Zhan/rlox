#![allow(dead_code)]

use crate::compiler::ByteEmiter;
use crate::scanner::Token;
use crate::result::InterpretResult;

// pub const RULES: [(&str, fn(&str) -> InterpretResult<()>); 1] = [
   
// ];

pub struct ParseRule {
    prefix: Option<fn(&mut Parser) -> InterpretResult<()>>,
    infix: Option<fn(&mut Parser, bool) -> InterpretResult<()>>,
    precedence: Precedence,
}

enum Precedence {
    None,
    Assignment, // =
    Or,         // or
    And,        // and
    Equality,   // == !=
    Comparison, // < > <= >=
    Term,       // + -
    Factor,     // * /
    Unary,      // ! -
    Call,       // . () []
    Primary,
}

impl From<usize> for Precedence {
    fn from(value: usize) -> Self {
        match value {
            0 => Precedence::None,
            1 => Precedence::Assignment,
            2 => Precedence::Or,
            3 => Precedence::And,
            4 => Precedence::Equality,
            5 => Precedence::Comparison,
            6 => Precedence::Term,
            7 => Precedence::Factor,
            8 => Precedence::Unary,
            9 => Precedence::Call,
            10 => Precedence::Primary,
            _ => panic!("Invalid precedence"),
        }
    }
}

#[derive(Debug)]
pub struct Parser {
    stack:  Vec<Token>,
}

impl Parser {
    pub fn new() -> Self {
        Self { 
            stack: vec![],
        }
    }

    pub fn parse(&mut self, tokens: impl Iterator<Item = Token>, emiter: &mut ByteEmiter) -> InterpretResult<()> {
        // first parese expression
        self.consume_expression(tokens, emiter);
        InterpretResult::Ok(())
    }

    pub fn advance(&mut self) {
        todo!()
    }

    fn parse_precedence(&mut self, precedence: Precedence) {
        // todo
        todo!()
    }

    fn consume(&mut self, token: Token, message: &str) -> InterpretResult<()> {
        todo!()
    }

    fn consume_if (&mut self, tokens: impl Iterator<Item = Token>, emiter: &mut ByteEmiter ) -> InterpretResult<()> {
        todo!()
    }

    fn consume_expression(&mut self, tokens: impl Iterator<Item = Token>, emiter: &mut ByteEmiter) {
        todo!()
    }



}