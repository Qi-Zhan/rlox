#![allow(dead_code)]

use crate::scanner::Token;
use crate::error::InterpretResult;

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

pub struct Parser {
    current: usize,
    previous: usize,
    had_error: bool,
    panic_mode: bool,
}

impl Parser {
    pub fn new() -> Self {
        Self { current: 0, previous: 0, had_error: false, panic_mode: false }
    }

    pub fn advance(&mut self) {
        self.previous = self.current;
        loop {
            self.current += 1;
            break;
            // todo
        }
    }

    pub fn expression(&mut self) {
        self.parse_precedence(Precedence::Assignment);
    }

    fn parse_precedence(&mut self, precedence: Precedence) {
        // todo
    }


}