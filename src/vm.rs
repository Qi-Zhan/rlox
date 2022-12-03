#![allow(dead_code)]

use std::collections::HashMap;

use crate::{chunk::Chunk, value::Value, result::InterpretResult};
use crate::opcode::*;

#[derive(Debug)]
pub struct VM {
    chunk:      Chunk,
    ip:         usize,
    stack:      Vec<Value>,
    // Global variables are late bound in rlox. 
    // “Late” in this context means “resolved after compile time”
    globals:    HashMap<String, Value>,
    /// for test
    prints   : Vec<String>,
}

impl<'a> VM {
    pub fn new(chunk: Chunk) -> Self {
        Self {
            chunk, 
            ip: 0, 
            stack: vec![], 
            globals: HashMap::new(),
            prints: vec![],
        }
    }


    pub fn interpreter(&mut self, chunk: Chunk) -> InterpretResult<Vec<String>> {
        self.chunk = chunk;        
        self.run()
    }

    fn run(&mut self) -> InterpretResult<Vec<String>> {
        loop {

            // println!("ip: {}, stack: {:?}", self.ip, self.stack);

            if self.ip >= self.chunk.code.len() {
                assert!(self.stack.is_empty());
                return InterpretResult::Ok(self.prints.clone());
            }
            
            let instruction = self.read_byte();
            
            match instruction {
                OP_RETURN => {
                    match self.stack.pop() {
                        Some(value) => {
                            self.prints.push(value.to_string());
                            return InterpretResult::Ok(self.prints.clone());
                        }
                        None => return InterpretResult::RuntimeError("Stack is empty".to_string()),
                    }
                }
                OP_CONSTANT => {
                    let constant = self.read_constant();
                    self.stack.push(constant)
                }
                OP_NEGATE => {
                    let value = -self.stack.pop().unwrap();
                    self.stack.push(value?)
                }
                OP_ADD => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push((a + b)?)
                }
                OP_DIVIDE=> {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push((a / b)?)
                }
                OP_SUBTRACT => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push((a - b)?)
                }
                OP_MULTIPLY => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push((a * b)?)
                }
                OP_PRINT => {
                    let value = self.stack.pop().unwrap();
                    // println!("{}", value); TODO: remove this
                    self.prints.push(value.to_string());
                }
                OP_AND => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push((a & b)?)
                }
                OP_OR => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push((a | b)?)
                }
                OP_NOT => {
                    let value = self.stack.pop().unwrap();
                    self.stack.push((!value)?)
                }
                OP_EQUAL => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(Value::Bool(a == b))
                }
                OP_GT => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    if a.is_number() && b.is_number() {
                        self.stack.push(Value::Bool(a > b))
                    } else {
                        return InterpretResult::RuntimeError("Operands must be numbers".to_string());
                    }
                }
                OP_LT => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    if a.is_number() && b.is_number() {
                        self.stack.push(Value::Bool(a < b))
                    } else {
                        return InterpretResult::RuntimeError("Operands must be numbers".to_string());
                    }
                }
                OP_GE => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    if a.is_number() && b.is_number() {
                        self.stack.push(Value::Bool(a >= b))
                    } else {
                        return InterpretResult::RuntimeError("Operands must be numbers".to_string());
                    }
                }
                OP_LE => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    if a.is_number() && b.is_number() {
                        self.stack.push(Value::Bool(a <= b))
                    } else {
                        return InterpretResult::RuntimeError("Operands must be numbers".to_string());
                    }
                }
                OP_NE => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(Value::Bool(a != b))
                }
                OP_POP => {
                    self.stack.pop();
                }
                OP_DEFINE_GLOBAL => {
                    let name = self.read_constant();
                    let value = self.stack.pop().unwrap();
                    self.globals.insert(name.to_string(), value);
                    
                }    
                OP_GET_GLOBAL => {
                    let name = self.read_constant();
                    let value = self.globals.get(&name.to_string());
                    match value {
                        Some(value) => self.stack.push(value.clone()),
                        None => return InterpretResult::RuntimeError(format!("Undefined variable '{}'", name)),
                    }
                }
                OP_SET_GLOBAL => {
                    let name = self.read_constant();
                    // setting a variable does not consume it
                    let value = self.stack.last().unwrap();
                    if self.globals.contains_key(name.to_string().as_str()) {
                        self.globals.insert(name.to_string(), value.clone());
                    } else {
                        return InterpretResult::RuntimeError(format!("Undefined variable '{}'", name));
                    }
                }
                OP_NIL => {
                    self.stack.push(Value::Nil)
                }
                OP_GET_LOCAL => {
                    let slot = self.read_byte();
                    let value = self.stack.get(slot as usize).unwrap();
                    self.stack.push(value.clone());
                }
                OP_SET_LOCAL => {
                    let slot = self.read_byte();
                    let value = self.stack.last().unwrap();
                    self.stack[slot as usize] = value.clone();
                }
                OP_JUMP_IF_FALSE => {
                    let offset = self.read_short();
                    let condition = self.stack.last().unwrap();
                    if !condition.is_truthy() {
                        self.ip += offset;
                    }
                }
                OP_JUMP => {
                    let offset = self.read_short();
                    self.ip += offset;
                }
                OP_LOOP => {
                    let offset = self.read_short();
                    self.ip -= offset;
                }
                _ => {
                    return InterpretResult::RuntimeError("Unknown opcode".to_string()); 
                }
            }
        }
    }

    fn read_byte(&mut self) -> u8 {
        let instruction = self.chunk.code[self.ip];
        self.ip += 1;
        instruction
    }

    fn read_constant(&mut self) -> Value {
        let index = self.read_byte() as usize;
        self.chunk.constants.values[index].clone()
    }

    fn read_short(&mut self) -> usize {
        let offset = (self.chunk.code[self.ip] as usize) << 8 | self.chunk.code[self.ip + 1] as usize;
        self.ip += 2;
        offset
    }


}

impl<'a> std::fmt::Display for VM {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for value in &self.stack {
            write!(f, "[ ")?;
            write!(f, "{value}")?;
            write!(f, " ]")?;
        }
        writeln!(f, "")
    }
}
