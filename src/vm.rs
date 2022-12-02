#![allow(dead_code)]

use crate::{chunk::Chunk, value::Value, result::InterpretResult};
use crate::opcode::*;

#[derive(Debug)]
pub struct VM {
    pub chunk:      Chunk,
    pub ip:         usize,
    pub stack:      Vec<Value>,
    /// for test
    pub prints   : Vec<String>,
}

impl<'a> VM {
    pub fn new(chunk: Chunk) -> Self {
        Self { chunk, ip: 0, stack: vec![], prints: vec![] }
    }


    pub fn interpreter(&mut self, chunk: Chunk) -> InterpretResult<Vec<String>> {
        self.chunk = chunk;        
        self.run()
    }

    fn run(&mut self) -> InterpretResult<Vec<String>> {
        loop {
            if cfg!(debug_assertions) {
                print!("{self}");
            }

            if self.ip >= self.chunk.code.len() {
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
                _ => {
                    return InterpretResult::CompileError("Unknown opcode".to_string()); 
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
