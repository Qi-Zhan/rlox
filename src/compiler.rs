use crate::chunk::Chunk;
use crate::scanner::Token;
use crate::result::InterpretResult;
use crate::value::*;
use crate::opcode::*;

#[derive(Debug)]
pub struct Compiler {

    emiter: ByteEmiter,

}

impl Compiler {
    pub fn new() -> Self {
        Self { 
            emiter: ByteEmiter::new(),
        }
    }

    pub fn compile(&mut self, tokens: impl Iterator<Item = Token>) -> InterpretResult<Chunk> {
        let mut tokens: Vec<Token> = tokens.collect();
        tokens.reverse();
        self.consume_program(&mut tokens)?;
        InterpretResult::Ok(self.emiter.return_chunk())

    }

    /// Consume token with expected token type, if not return error
    fn consume(&mut self, token: Token, tokens: &mut Vec<Token>, message: &str) -> InterpretResult<()> {
        if tokens.pop() == Some(token) {
            InterpretResult::Ok(())
        } else {
            InterpretResult::CompileError(message.to_string())
        }
    }

    fn consume_program(&mut self, tokens: &mut Vec<Token>) -> InterpretResult<()> {
        while !tokens.is_empty() {
            self.consume_declaration(tokens)?;
        }
        InterpretResult::Ok(())
    }

    fn consume_declaration(&mut self, tokens: &mut Vec<Token>) -> InterpretResult<()> {
        let token = tokens.last();
        match token {
            Some(Token::Var) => {
                self.consume(Token::Var, tokens, "Expected 'var' keyword")?;
                let global = self.consume_variable(tokens)?;
                // initial expression or nil
                if tokens.last() == Some(&Token::Equal) {
                    self.consume(Token::Equal, tokens, "Expected '='")?;
                    self.consume_expression(tokens)?;
                } else {
                    self.emiter.emit_byte(OP_NIL);
                }
                self.consume(Token::Semicolon, tokens, "Expected ';'")?;
                self.emiter.emit_bytes(OP_DEFINE_GLOBAL, global);
                InterpretResult::Ok(())
            },
            Some(Token::Class) => {
                InterpretResult::Ok(())
            },
            Some(Token::Fun) => {
                InterpretResult::Ok(())
            },
            _ => { // statement
                self.consume_stmt(tokens)?;
                InterpretResult::Ok(())
            }
        }
    }

    fn consume_stmt(&mut self, tokens: &mut Vec<Token>) -> InterpretResult<()> {
        let token = tokens.last();
        match token {
            Some(Token::Print) => {
                self.consume(Token::Print, tokens, "Expected 'print'")?;
                self.consume_expression(tokens)?;
                self.emiter.emit_byte(OP_PRINT);
                self.consume(Token::Semicolon, tokens, "Expect ';' after expression.")?;
            },
            Some(Token::LeftBrace) => {
                self.consume(Token::LeftBrace, tokens, "Expected '{'")?;
                self.consume_stmt(tokens)?;
                self.consume(Token::RightBrace, tokens, "Expect '}' after block.")?;
            },
            Some(Token::If) => {
                self.consume(Token::If, tokens, "Expected 'if'")?;
                self.consume(Token::LeftParen, tokens, "Expect '(' after 'if'.")?;
                self.consume_expression(tokens)?;
                self.consume(Token::RightParen, tokens, "Expect ')' after condition.")?;
                self.consume_stmt(tokens)?;
                self.consume(Token::Else, tokens, "Expect 'else' after 'if' block.")?;
                self.consume_stmt(tokens)?;
            },
            Some(Token::While) => {
                self.consume(Token::While, tokens, "Expected 'while'")?;
                self.consume(Token::LeftParen, tokens, "Expect '(' after 'while'.")?;
                self.consume_expression(tokens)?;
                self.consume(Token::RightParen, tokens, "Expect ')' after condition.")?;
                self.consume_stmt(tokens)?;
            },
            Some(Token::Return) => {
                self.consume(Token::Return, tokens, "Expected 'return'")?;
                self.consume_expression(tokens)?;
                self.consume(Token::Semicolon, tokens, "Expect ';' after return expression.")?;
                self.emiter.emit_return();
            },
            Some(Token::For) => { // TODO
                self.consume(Token::For, tokens, "Expected 'for'")?;
                self.consume(Token::LeftParen, tokens, "Expect '(' after 'for'.")?;
                self.consume_declaration(tokens)?;
                self.consume_expression(tokens)?;
                self.consume(Token::Semicolon, tokens, "Expect ';' after loop condition.")?;
                self.consume_expression(tokens)?;
                self.consume(Token::RightParen, tokens, "Expect ')' after for clauses.")?;
                self.consume_stmt(tokens)?;
            },
            _ => { // expression statement
                self.consume_expression(tokens)?;
                self.consume(Token::Semicolon, tokens, "Expect ';' after value.")?;
                self.emiter.emit_byte(OP_POP);
            },
        }
        InterpretResult::Ok(())
    }

    fn consume_expression(&mut self, tokens: &mut Vec<Token>) -> InterpretResult<()> {
        self.consume_assignment(tokens)?;
        InterpretResult::Ok(())
    }

    fn is_assignment(&self, tokens: &Vec<Token>) -> bool {
        if tokens.len() < 2 {
            return false;
        }
        for token in tokens.iter().rev(){
            if *token == Token::Equal {
                return true;
            }
            if *token == Token::Semicolon {
                return false;
            }
        }
        false
    }

    fn consume_assignment(&mut self, tokens: &mut Vec<Token>) -> InterpretResult<()> {
        if self.is_assignment(tokens) { // assignment
            let global = self.consume_variable(tokens)?;
            self.consume(Token::Equal, tokens, "Expected '='")?;
            self.consume_assignment(tokens)?;
            self.emiter.emit_bytes(OP_SET_GLOBAL, global);
        } else {
            self.consume_or(tokens)?;
        }
        InterpretResult::Ok(())
    }

    fn consume_or(&mut self, tokens: &mut Vec<Token>) -> InterpretResult<()> {
        self.consume_and(tokens)?;
        loop {
            let token = tokens.last();
            match token {
                Some(Token::Or) => {
                    self.consume(Token::Or, tokens, "Expected 'or'")?;
                    self.consume_and(tokens)?;
                    self.emiter.emit_byte(OP_OR);
                },
                _ => {
                    break;
                }
            }
        }
        InterpretResult::Ok(())
    }

    fn consume_and(&mut self, tokens: &mut Vec<Token>) -> InterpretResult<()> {
        self.consume_equality(tokens)?;
        loop {
            let token = tokens.last();
            match token {
                Some(Token::And) => {
                    self.consume(Token::And, tokens, "Expected 'and'")?;
                    self.consume_equality(tokens)?;
                    self.emiter.emit_byte(OP_AND);
                },
                _ => {
                    break;
                }
            }
        }
        InterpretResult::Ok(())
    }

    fn consume_equality(&mut self, tokens: &mut Vec<Token>) -> InterpretResult<()> {
        self.consume_comparison(tokens)?;
        loop {
            let token = tokens.last();
            match token {
                Some(Token::BangEqual) => {
                    self.consume(Token::BangEqual, tokens, "Expected '!='")?;
                    self.consume_comparison(tokens)?;
                    self.emiter.emit_byte(OP_NE);
                },
                Some(Token::EqualEqual) => {
                    self.consume(Token::EqualEqual, tokens, "Expected '=='")?;
                    self.consume_comparison(tokens)?;
                    self.emiter.emit_byte(OP_EQUAL);
                },
                _ => {
                    break;
                }
            }
        }
        InterpretResult::Ok(())
    }

    fn consume_comparison(&mut self, tokens: &mut Vec<Token>) -> InterpretResult<()> {
        self.consume_term(tokens)?;
        loop {
            let token = tokens.last();
            match token {
                Some(Token::Greater) => {
                    self.consume(Token::Greater, tokens, "Expected '>'")?;
                    self.consume_term(tokens)?;
                    self.emiter.emit_byte(OP_GT);
                },
                Some(Token::GreaterEqual) => {
                    self.consume(Token::GreaterEqual, tokens, "Expected '>='")?;
                    self.consume_term(tokens)?;
                    self.emiter.emit_byte(OP_GE);
                },
                Some(Token::Less) => {
                    self.consume(Token::Less, tokens, "Expected '<'")?;
                    self.consume_term(tokens)?;
                    self.emiter.emit_byte(OP_LT);
                },
                Some(Token::LessEqual) => {
                    self.consume(Token::LessEqual, tokens, "Expected '<='")?;
                    self.consume_term(tokens)?;
                    self.emiter.emit_byte(OP_LE);
                },
                _ => {
                    break;
                }
            }
        }
        InterpretResult::Ok(())
    }

    fn consume_term(&mut self, tokens: &mut Vec<Token>) -> InterpretResult<()> {
        self.consume_factor(tokens)?;
        loop {
            match tokens.last() {
                Some(Token::Plus) => {
                    self.consume(Token::Plus, tokens, "Expected '+'")?;
                    self.consume_factor(tokens)?;
                    self.emiter.emit_byte(OP_ADD);
                }
                Some(Token::Minus) => {
                    self.consume(Token::Minus, tokens, "Expected '-'")?;
                    self.consume_factor(tokens)?;
                    self.emiter.emit_byte(OP_SUBTRACT);
                }
                _ => break,
            }
        }

        InterpretResult::Ok(())
    }

    fn consume_factor(&mut self, tokens: &mut Vec<Token>) -> InterpretResult<()> {
        self.consume_unary(tokens)?;
        loop {
            match tokens.last() {
                Some(Token::Star) => {
                    self.consume(Token::Star, tokens, "Expected '*'")?;
                    self.consume_unary(tokens)?;
                    self.emiter.emit_byte(OP_MULTIPLY);
                }
                Some(Token::Slash) => {
                    self.consume(Token::Slash, tokens, "Expected '/'")?;
                    self.consume_unary(tokens)?;
                    self.emiter.emit_byte(OP_DIVIDE);
                }
                _ => break,
            }
        }
        InterpretResult::Ok(())
    }

    fn consume_unary(&mut self, tokens: &mut Vec<Token>) -> InterpretResult<()> {
        match tokens.last() {
            Some(Token::Minus) => {
                self.consume(Token::Minus, tokens, "Expected '-'")?;
                self.consume_unary(tokens)?;
                self.emiter.emit_byte(OP_NEGATE);
            }
            Some(Token::Bang) => {
                self.consume(Token::Bang, tokens, "Expected '!'")?;
                self.consume_unary(tokens)?;
                self.emiter.emit_byte(OP_NOT);
            }
            Some(_) => {
                self.consume_primary(tokens)?
            }
            None => return InterpretResult::CompileError("Expected expression".to_string()),
        }
        InterpretResult::Ok(())
    }

    fn consume_primary(&mut self, tokens: &mut Vec<Token>) -> InterpretResult<()> {
        match tokens.pop() {
            Some(Token::Number(value)) => {
                self.emiter.emit_constant(Value::Number(value));
                InterpretResult::Ok(())
            }
            Some(Token::String(value)) => {
                let obj = Obj::Str(value);
                self.emiter.emit_constant(Value::Obj(obj));
                InterpretResult::Ok(())
            }
            Some(Token::True) => {
                self.emiter.emit_constant(Value::Bool(true));
                InterpretResult::Ok(())
            }
            Some(Token::False) => {
                self.emiter.emit_constant(Value::Bool(false));
                InterpretResult::Ok(())
            }
            Some(Token::Nil) => {
                self.emiter.emit_constant(Value::Nil);
                InterpretResult::Ok(())
            }
            Some(Token::LeftParen) => {
                self.consume_expression(tokens)?;
                self.consume(Token::RightParen, tokens, "Expect ')' after expression.")?;
                InterpretResult::Ok(())
            }
            Some(Token::Identifier(id)) => {
                let id = self.emiter.make_constant(Value::Obj(Obj::Str(id)));
                self.emiter.emit_byte(OP_GET_GLOBAL);
                self.emiter.emit_byte(id);
                InterpretResult::Ok(())      
            }
            _ => InterpretResult::CompileError("Expect expression.".to_string()),
        }

    }

    fn consume_variable(&mut self, tokens: &mut Vec<Token>) -> InterpretResult<u8> {
        match tokens.pop() {
            Some(Token::Identifier(id)) => {
                let id = self.emiter.make_constant(Value::Obj(Obj::Str(id)));
                InterpretResult::Ok(id)
            }
            _ => InterpretResult::CompileError("Expect variable name.".to_string()),
        }
    }

}
    

#[derive(Debug)]
pub struct ByteEmiter {
    chunk: Chunk,
}

impl ByteEmiter {
    fn new() -> Self {
        Self { 
            chunk: Chunk::new(),
        }
    }

    fn return_chunk(&self) -> Chunk {
        self.chunk.clone()
    }

    fn emit_byte(&mut self, byte: u8) {
        self.chunk.write_chunk(byte);
    }

    fn emit_bytes(&mut self, byte1: u8, byte2: u8) {
        self.emit_byte(byte1);
        self.emit_byte(byte2);
    }

    fn emit_return(&mut self) {
        self.emit_byte(OP_RETURN);
    }

    fn emit_constant(&mut self, value: Value) {
        let constant = self.chunk.add_constant(value);
        self.emit_bytes(OP_CONSTANT, constant as u8);
    }

    fn make_constant(&mut self, value: Value) -> u8 {
        let constant = self.chunk.add_constant(value);
        constant as u8
    }


}
