use crate::chunk::Chunk;
use crate::scanner::Token;
use crate::result::InterpretResult;
use crate::value::*;
use crate::opcode::*;

#[derive(Debug)]
pub struct Compiler {

    locals: Vec<Local>,
    scope_depth: usize,
    // functions: Vec<Function>,
    emiter: ByteEmiter,

}

impl Compiler {
    pub fn new() -> Self {
        Self { 
            locals: vec![],
            scope_depth: 0,
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

    fn consume_initializer(&mut self, tokens: &mut Vec<Token>) -> InterpretResult<()> {
        // initial expression or nil
        if tokens.last() == Some(&Token::Equal) {
            self.consume(Token::Equal, tokens, "Expected '='")?;
            self.consume_expression(tokens)?;
        } else {
            self.emiter.emit_byte(OP_NIL);
        }
        InterpretResult::Ok(())
    }

    fn consume_declaration(&mut self, tokens: &mut Vec<Token>) -> InterpretResult<()> {
        let token = tokens.last();
        match token {
            Some(Token::Var) => {
                self.consume(Token::Var, tokens, "Expected 'var' keyword")?;

                if self.scope_depth == 0 { // globally
                    let global = self.consume_global_variable(tokens)?;
                    self.consume_initializer(tokens)?;
                    self.emiter.emit_bytes(OP_DEFINE_GLOBAL, global);
                } else { // locally
                    let local = self.consume_local_variable(tokens)?;
                    self.consume_initializer(tokens)?;
                    for vars in self.locals.iter().rev() {
                        if vars.depth < self.scope_depth {
                            break;
                        }
                        if vars.name == local {
                            return InterpretResult::CompileError("Variable with this name already declared in this scope".to_string());
                        }
                    }
                    self.add_local(local.clone());
                    self.emiter.emit_bytes(OP_SET_LOCAL,self.resolve_local(&local).unwrap())
                }

                self.consume(Token::Semicolon, tokens, "Expected ';'")?;
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
            // print statement
            Some(Token::Print) => {
                self.consume(Token::Print, tokens, "Expected 'print'")?;
                self.consume_expression(tokens)?;
                self.emiter.emit_byte(OP_PRINT);
                self.consume(Token::Semicolon, tokens, "Expect ';' after expression.")?;
            },
            // block statement
            Some(Token::LeftBrace) => {
                self.consume_block(tokens)?;
            },
            // if statement
            Some(Token::If) => {
                self.consume_if(tokens)?;
            },
            // while statement
            Some(Token::While) => {
                self.consume(Token::While, tokens, "Expected 'while'")?;
                self.consume(Token::LeftParen, tokens, "Expect '(' after 'while'.")?;
                self.consume_expression(tokens)?;
                self.consume(Token::RightParen, tokens, "Expect ')' after condition.")?;
                self.consume_stmt(tokens)?;
            },
            // return statment
            Some(Token::Return) => {
                self.consume(Token::Return, tokens, "Expected 'return'")?;
                self.consume_expression(tokens)?;
                self.consume(Token::Semicolon, tokens, "Expect ';' after return expression.")?;
                self.emiter.emit_return();
            },
            // for statment
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
            // expression statement
            _ => { 
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
        if matches!(tokens[tokens.len()-1], Token::Identifier{..}) && tokens[tokens.len()-2] == Token::Equal {
            return true;
        }
        false
    }

    fn consume_assignment(&mut self, tokens: &mut Vec<Token>) -> InterpretResult<()> {
        if self.is_assignment(tokens) { // assignment
            let name = self.consume_local_variable(tokens)?;
            match self.resolve_local(&name) {
                Some(index) => {
                    self.consume(Token::Equal, tokens, "Expected '='")?;
                    self.consume_assignment(tokens)?;
                    self.emiter.emit_bytes(OP_SET_LOCAL, index);
                },
                None => {
                    self.consume(Token::Equal, tokens, "Expected '='")?;
                    self.consume_assignment(tokens)?;
                    let global = self.emiter.make_string(name);
                    self.emiter.emit_bytes(OP_SET_GLOBAL, global);
                }
            }
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
            Some(Token::Identifier(id)) => { // get variable
                if self.scope_depth > 0 {
                    if let Some(index) = self.resolve_local(&id) {
                        self.emiter.emit_byte(OP_GET_LOCAL);
                        self.emiter.emit_byte(index);
                        return InterpretResult::Ok(());
                    }
                }
                let id = self.emiter.make_constant(Value::Obj(Obj::Str(id)));
                self.emiter.emit_byte(OP_GET_GLOBAL);
                self.emiter.emit_byte(id);
                InterpretResult::Ok(())      
            }
            _ => InterpretResult::CompileError("Expect expression.".to_string()),
        }

    }

    fn consume_global_variable(&mut self, tokens: &mut Vec<Token>) -> InterpretResult<u8> {
        match tokens.pop() {
            Some(Token::Identifier(id)) => {
                let id = self.emiter.make_string(id);
                InterpretResult::Ok(id)
            }
            _ => InterpretResult::CompileError("Expect  variable name.".to_string()),
        }
    }

    fn consume_local_variable(&mut self, tokens: &mut Vec<Token>) -> InterpretResult<String> {
        match tokens.pop() {
            Some(Token::Identifier(id)) => {
                InterpretResult::Ok(id)
            }
            _ => InterpretResult::CompileError("Expect variable name.".to_string()),
        }
    }

    fn add_local(&mut self, name: String) {
        self.locals.push(Local::new(name, self.scope_depth));
    }

    fn resolve_local(&self, name: &str) -> Option<u8> {
        for (i, local) in self.locals.iter().enumerate().rev() {
            if local.name == name {
                return Some(i as u8);
            }
        }
        None
    }

    fn consume_block(&mut self, tokens: &mut Vec<Token>) -> InterpretResult<()> {
        self.begin_scope(tokens)?;
        while tokens.last() != Some(&Token::RightBrace) {
            self.consume_declaration(tokens)?;
        }
        self.end_scope(tokens)?;
        InterpretResult::Ok(())
    }

    fn begin_scope(&mut self, tokens: &mut Vec<Token>) -> InterpretResult<()> {
        self.consume(Token::LeftBrace, tokens, "Expected '{'")?;
        self.scope_depth += 1;
        InterpretResult::Ok(())
    }

    fn end_scope(&mut self, tokens: &mut Vec<Token>) -> InterpretResult<()> {
        self.consume(Token::RightBrace, tokens, "Expected '}'")?;
        self.scope_depth -= 1;
        while let Some(local) = self.locals.last() {
            if local.depth > self.scope_depth {
                self.locals.pop();
                self.emiter.emit_byte(OP_POP);
            } else {
                break;
            }
        }
        InterpretResult::Ok(())
    }

    fn consume_if(&mut self, tokens:&mut Vec<Token>) -> InterpretResult<()> {
        // condition expression
        // OP_JUMP_IF_FALSE --
        // OP_POP            |
        // then statement    |
        // OP_JUMP           |  --
        // OP_POP         <==|   |
        // else statement        |
        // continues          <==|
        self.consume(Token::If, tokens, "Expected 'if'")?;

        self.consume(Token::LeftParen, tokens, "Expect '(' after 'if'.")?;
        self.consume_expression(tokens)?;
        self.consume(Token::RightParen, tokens, "Expect ')' after condition.")?;

        let else_jump = self.emiter.emit_jump(OP_JUMP_IF_FALSE);
        self.emiter.emit_byte(OP_POP);

        self.consume_stmt(tokens)?; // then statement
        let end_jump = self.emiter.emit_jump(OP_JUMP);
        self.emiter.patch_jump(else_jump);

        self.emiter.emit_byte(OP_POP);
        if tokens.last() == Some(&Token::Else) {
            self.consume(Token::Else, tokens, "Expect 'else' after 'if' block.")?;
            self.consume_stmt(tokens)?; // else statement
        }
        self.emiter.patch_jump(end_jump);
        InterpretResult::Ok(())
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

    fn emit_jump(&mut self, instruction: u8) -> usize {
        self.emit_byte(instruction);
        self.emit_byte(0xff);
        self.emit_byte(0xff);
        self.chunk.code.len() - 2
    }

    fn patch_jump(&mut self, offset: usize) {
        let jump = self.chunk.code.len() - offset - 2;
        self.chunk.code[offset] = ((jump >> 8) & 0xff) as u8;
        self.chunk.code[offset + 1] = (jump & 0xff) as u8;
    }

    fn emit_constant(&mut self, value: Value) {
        let constant = self.chunk.add_constant(value);
        self.emit_bytes(OP_CONSTANT, constant as u8);
    }

    fn make_string(&mut self, value: String) -> u8 {
        let constant = self.chunk.add_constant(Value::Obj(Obj::Str(value)));
        constant as u8
    }

    fn make_constant(&mut self, value: Value) -> u8 {
        let constant = self.chunk.add_constant(value);
        constant as u8
    }


}

#[derive(Debug)]
struct Local {
    name: String,
    depth: usize,
}

impl Local {
    fn new (name: String, depth: usize) -> Self {
        Self {
            name,
            depth,
        }
    }
}
