use std::vec;

use crate::scanner::Token;
use crate::result::InterpretResult;
use crate::value::*;
use crate::opcode::*;

#[derive(Debug)]
pub struct Compiler {

    locals: Vec<Local>,
    scope_depth: usize,

    function: Function,
    type_: FunctionType,

}

impl Compiler {
    pub fn new(name: String, type_: FunctionType) -> Self {
        Self { 
            locals: vec![],
            scope_depth: 0,
            function: Function::new(name,0),
            type_,
        }
    }

    pub fn compile(&mut self, tokens: Vec<Token>) -> InterpretResult<Function> {
        let mut tokens = tokens;
        tokens.reverse();
        self.consume_program(&mut tokens)?;
        // main function always returns nil
        self.function.emit_bytes(OP_NIL, OP_RETURN); 
        InterpretResult::Ok(self.function.clone())

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
            self.function.emit_byte(OP_NIL);
        }
        InterpretResult::Ok(())
    }

    fn consume_var_declaration(&mut self, tokens: &mut Vec<Token>) -> InterpretResult<()> {
        self.consume(Token::Var, tokens, "Expected 'var' keyword")?;

        if self.scope_depth == 0 { // globally
            let global = self.consume_global_variable(tokens)?;
            self.consume_initializer(tokens)?;
            self.function.emit_bytes(OP_DEFINE_GLOBAL, global);
        } else { // locally
            let local = self.consume_identifier(tokens)?;
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
            self.function.emit_bytes(OP_SET_LOCAL,self.resolve_local(&local).unwrap())
        }

        self.consume(Token::Semicolon, tokens, "Expected ';'")?;
        InterpretResult::Ok(())
    }

    fn consume_declaration(&mut self, tokens: &mut Vec<Token>) -> InterpretResult<()> {
        let token = tokens.last();
        match token {
            Some(Token::Var) => {
                self.consume_var_declaration(tokens)
            },
            Some(Token::Class) => {
                InterpretResult::Ok(())
            },
            Some(Token::Fun) => {
                self.consume_function_declaration(tokens, FunctionType::Function)
            },
            _ => { // statement
                self.consume_stmt(tokens)?;
                InterpretResult::Ok(())
            }
        }
    }

    fn consume_function_declaration(&mut self, tokens: &mut Vec<Token>, type_: FunctionType) -> InterpretResult<()> {
        self.consume(Token::Fun, tokens, "Expected 'fun' keyword")?;
        let name = self.consume_identifier(tokens)?;
        self.consume_function(tokens, name.clone(), type_)?;
        if self.scope_depth == 0 {
            let global = self.function.make_string(name);
            self.function.emit_bytes(OP_DEFINE_GLOBAL, global);
            // self.function.emit_bytes(OP_DEFINE_GLOBAL, global);
        } else {
            self.add_local(name.clone());
            self.function.emit_bytes(OP_SET_LOCAL, self.resolve_local(&name).unwrap());
        }
        
        InterpretResult::Ok(())
    }

    fn consume_function(&mut self, tokens: &mut Vec<Token>, name: String, type_: FunctionType) -> InterpretResult<()> {
        let mut compiler = Compiler::new(name.clone(), type_);
        compiler.begin_scope();
        compiler.add_local(name.clone());
        compiler.consume(Token::LeftParen, tokens, "Expected '('")?;
        let mut arity:u32 = 0;
        let mut paras = vec![];
        if tokens.last() != Some(&Token::RightParen) {
            loop {
                let local = compiler.consume_identifier(tokens)?;
                if paras.contains(&local) {
                    return InterpretResult::CompileError("Duplicated parameter name".to_string());
                } else {
                    paras.push(local.clone());
                }
                compiler.add_local(local);
                arity += 1;
                if arity > 255 {
                    return InterpretResult::CompileError("Cannot have more than 255 parameters".to_string());
                }
                if tokens.last() == Some(&Token::Comma) {
                    compiler.consume(Token::Comma, tokens, "Expected ','")?;
                } else {
                    break;
                }
            }
        }
        compiler.function.arity = arity as u8;
        compiler.consume(Token::RightParen, tokens, "Expected ')'")?;
        compiler.consume_block(tokens)?;
        // implicit return
        compiler.function.emit_bytes(OP_NIL, OP_RETURN); 
        println!("function:\n{}", compiler.function.chunk);
        let function = compiler.function;
        let index = self.function.make_constant(Value::Function(function));
        self.function.emit_bytes(OP_CONSTANT, index);
        InterpretResult::Ok(())
    }

    fn consume_stmt(&mut self, tokens: &mut Vec<Token>) -> InterpretResult<()> {
        let token = tokens.last();
        match token {
            // print statement
            Some(Token::Print) => {
                self.consume(Token::Print, tokens, "Expected 'print'")?;
                self.consume_expression(tokens)?;
                self.function.emit_byte(OP_PRINT);
                self.consume(Token::Semicolon, tokens, "Expect ';' after expression.")
            }
            // block statement
            Some(Token::LeftBrace) => {
                self.consume_block(tokens)
            }
            // if statement
            Some(Token::If) => {
                self.consume_if(tokens)
            }
            // while statement
            Some(Token::While) => {
                self.consume_while(tokens)
            }
            // return statment
            Some(Token::Return) => {
                if self.type_ == FunctionType::Script {
                    return InterpretResult::CompileError("Can't return from top-level code.".to_string());
                }
                self.consume(Token::Return, tokens, "Expected 'return'")?;
                if tokens.last() == Some(&Token::Semicolon) {
                    self.function.emit_byte(OP_NIL);
                } else {
                    self.consume_expression(tokens)?;
                }
                self.function.emit_return();
                self.consume(Token::Semicolon, tokens, "Expect ';' after return expression.")
            }
            // for statment
            Some(Token::For) => { 
                self.consume_for(tokens)
            }
            // expression statement
            _ => { 
                self.consume_exprstmt(tokens)
            },
        }
    }

    fn consume_exprstmt(&mut self, tokens: &mut Vec<Token>) -> InterpretResult<()> {
        self.consume_expression(tokens)?;
        self.consume(Token::Semicolon, tokens, "Expect ';' after value.")?;
        self.function.emit_byte(OP_POP);
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
            let name = self.consume_identifier(tokens)?;
            match self.resolve_local(&name) {
                Some(index) => {
                    self.consume(Token::Equal, tokens, "Expected '='")?;
                    self.consume_assignment(tokens)?;
                    self.function.emit_bytes(OP_SET_LOCAL, index);
                },
                None => {
                    self.consume(Token::Equal, tokens, "Expected '='")?;
                    self.consume_assignment(tokens)?;
                    let global = self.function.make_string(name);
                    self.function.emit_bytes(OP_SET_GLOBAL, global);
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
                    self.function.emit_byte(OP_OR);
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
                    self.function.emit_byte(OP_AND);
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
                    self.function.emit_byte(OP_NE);
                },
                Some(Token::EqualEqual) => {
                    self.consume(Token::EqualEqual, tokens, "Expected '=='")?;
                    self.consume_comparison(tokens)?;
                    self.function.emit_byte(OP_EQUAL);
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
                    self.function.emit_byte(OP_GT);
                },
                Some(Token::GreaterEqual) => {
                    self.consume(Token::GreaterEqual, tokens, "Expected '>='")?;
                    self.consume_term(tokens)?;
                    self.function.emit_byte(OP_GE);
                },
                Some(Token::Less) => {
                    self.consume(Token::Less, tokens, "Expected '<'")?;
                    self.consume_term(tokens)?;
                    self.function.emit_byte(OP_LT);
                },
                Some(Token::LessEqual) => {
                    self.consume(Token::LessEqual, tokens, "Expected '<='")?;
                    self.consume_term(tokens)?;
                    self.function.emit_byte(OP_LE);
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
                    self.function.emit_byte(OP_ADD);
                }
                Some(Token::Minus) => {
                    self.consume(Token::Minus, tokens, "Expected '-'")?;
                    self.consume_factor(tokens)?;
                    self.function.emit_byte(OP_SUBTRACT);
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
                    self.function.emit_byte(OP_MULTIPLY);
                }
                Some(Token::Slash) => {
                    self.consume(Token::Slash, tokens, "Expected '/'")?;
                    self.consume_unary(tokens)?;
                    self.function.emit_byte(OP_DIVIDE);
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
                self.function.emit_byte(OP_NEGATE);
            }
            Some(Token::Bang) => {
                self.consume(Token::Bang, tokens, "Expected '!'")?;
                self.consume_unary(tokens)?;
                self.function.emit_byte(OP_NOT);
            }
            Some(_) => {
                self.consume_call(tokens)?
            }
            None => return InterpretResult::CompileError("Expected expression".to_string()),
        }
        InterpretResult::Ok(())
    }

    fn consume_primary(&mut self, tokens: &mut Vec<Token>) -> InterpretResult<()> {
        match tokens.pop() {
            Some(Token::Number(value)) => {
                self.function.emit_constant(Value::Number(value));
                InterpretResult::Ok(())
            }
            Some(Token::String(value)) => {
                self.function.emit_constant(Value::String(value));
                InterpretResult::Ok(())
            }
            Some(Token::True) => {
                self.function.emit_constant(Value::Bool(true));
                InterpretResult::Ok(())
            }
            Some(Token::False) => {
                self.function.emit_constant(Value::Bool(false));
                InterpretResult::Ok(())
            }
            Some(Token::Nil) => {
                self.function.emit_constant(Value::Nil);
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
                        self.function.emit_byte(OP_GET_LOCAL);
                        self.function.emit_byte(index);
                        return InterpretResult::Ok(());
                    }
                }
                let id = self.function.make_constant(Value::String(id));
                self.function.emit_byte(OP_GET_GLOBAL);
                self.function.emit_byte(id);
                InterpretResult::Ok(())      
            }
            _ => InterpretResult::CompileError("Expect expression.".to_string()),
        }

    }

    fn consume_global_variable(&mut self, tokens: &mut Vec<Token>) -> InterpretResult<u8> {
        match tokens.pop() {
            Some(Token::Identifier(id)) => {
                let id = self.function.make_string(id);
                InterpretResult::Ok(id)
            }
            _ => InterpretResult::CompileError("Expect variable name.".to_string()),
        }
    }

    fn consume_identifier(&mut self, tokens: &mut Vec<Token>) -> InterpretResult<String> {
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
        self.begin_scope()?;
        self.consume(Token::LeftBrace, tokens, "Expected '{'")?;
        while tokens.last() != Some(&Token::RightBrace) {
            self.consume_declaration(tokens)?;
        }
        self.consume(Token::RightBrace, tokens, "Expected '}'")?;
        self.end_scope()?;
        InterpretResult::Ok(())
    }

    fn begin_scope(&mut self) -> InterpretResult<()> {
        self.scope_depth += 1;
        InterpretResult::Ok(())
    }

    fn end_scope(&mut self) -> InterpretResult<()> {
        self.scope_depth -= 1;
        while let Some(local) = self.locals.last() {
            if local.depth > self.scope_depth {
                self.locals.pop();
                self.function.emit_byte(OP_POP);
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

        let else_jump = self.function.emit_jump(OP_JUMP_IF_FALSE);
        self.function.emit_byte(OP_POP);

        self.consume_stmt(tokens)?; // then statement
        let end_jump = self.function.emit_jump(OP_JUMP);
        self.function.patch_jump(else_jump);

        self.function.emit_byte(OP_POP);
        if tokens.last() == Some(&Token::Else) {
            self.consume(Token::Else, tokens, "Expect 'else' after 'if' block.")?;
            self.consume_stmt(tokens)?; // else statement
        }
        self.function.patch_jump(end_jump);
        InterpretResult::Ok(())
    }

    fn consume_while(&mut self, tokens:&mut Vec<Token>) -> InterpretResult<()> {
        // condition expression <==
        // OP_JUMP_IF_FALSE --    |
        // OP_POP            |    |
        // body statement    |    |
        // OP_LOOP           |  ---
        // OP_POP         <==|   
        // continues          

        let loop_start = self.function.chunk.code.len();
        self.consume(Token::While, tokens, "Expected 'while'")?;

        self.consume(Token::LeftParen, tokens, "Expect '(' after 'while'.")?;
        self.consume_expression(tokens)?;
        self.consume(Token::RightParen, tokens, "Expect ')' after condition.")?;

        let exit_jump = self.function.emit_jump(OP_JUMP_IF_FALSE);
        self.function.emit_byte(OP_POP);

        self.consume_stmt(tokens)?; // body statement
        self.function.emit_loop(loop_start);

        self.function.patch_jump(exit_jump);
        self.function.emit_byte(OP_POP);
        InterpretResult::Ok(())
    }

    fn consume_for(&mut self, tokens:&mut Vec<Token>) -> InterpretResult<()> {        

        self.consume(Token::For, tokens, "Expected 'for'")?;
        self.begin_scope()?;

        self.consume(Token::LeftParen, tokens, "Expect '(' after 'for'.")?;
        match tokens.last() {
            Some(Token::Var) => {
                self.consume_declaration(tokens)?;
            }
            Some(Token::Semicolon) => {
                self.consume(Token::Semicolon, tokens, "Expect ';' after 'for' init expression.")?;
            }
            _ => {
                self.consume_exprstmt(tokens)?;
            }
        }

        let mut loop_start = self.function.chunk.code.len();
        let mut exit_jump:Option<usize> = None;
        if tokens.last() != Some(&Token::Semicolon) {
            self.consume_expression(tokens)?;
            exit_jump = Some(self.function.emit_jump(OP_JUMP_IF_FALSE));
            self.function.emit_byte(OP_POP);
        }
        self.consume(Token::Semicolon, tokens, "Expect ';' after condition.")?;

        if tokens.last() != Some(&Token::RightParen) {
            let body_jump = self.function.emit_jump(OP_JUMP);
            let increment_start = self.function.chunk.code.len();
            self.consume_expression(tokens)?;
            self.function.emit_byte(OP_POP);
            self.consume(Token::RightParen, tokens, "Expect ')' after for clauses.")?;
            
            self.function.emit_loop(loop_start);
            loop_start = increment_start;
            self.function.patch_jump(body_jump);
        } else {
            self.consume(Token::RightParen, tokens, "Expect ')' after for clauses.")?;
        }

        self.consume_stmt(tokens)?; // body statement
        self.function.emit_loop(loop_start);

        if let Some(exit_jump) = exit_jump {
            self.function.patch_jump(exit_jump);
            self.function.emit_byte(OP_POP);
        }
        self.end_scope()?;
        InterpretResult::Ok(())
    }

    fn consume_argument_list(&mut self, tokens:&mut Vec<Token>) -> InterpretResult<u8> {
        let mut arg_count = 0;
        if tokens.last() != Some(&Token::RightParen) {
            loop {
                self.consume_expression(tokens)?;
                if arg_count == 255 {
                    return InterpretResult::CompileError("Cannot have more than 255 arguments.".to_string());
                }
                arg_count += 1;
                if tokens.last() != Some(&Token::Comma) {
                    break;
                }
                self.consume(Token::Comma, tokens, "Expect ',' after argument.")?;
            }
        }
        self.consume(Token::RightParen, tokens, "Expect ')' after arguments.")?;
        InterpretResult::Ok(arg_count as u8)  
    }

    fn consume_call(&mut self, tokens:&mut Vec<Token>) -> InterpretResult<()> {
        self.consume_primary(tokens)?;
        if let Some(Token::LeftParen) = tokens.last() {
            self.consume(Token::LeftParen, tokens, "Expect '(' after function name.")?;
            let arg_count = self.consume_argument_list(tokens)?;
            self.function.emit_byte(OP_CALL);
            self.function.emit_byte(arg_count);
        }
        InterpretResult::Ok(())
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
