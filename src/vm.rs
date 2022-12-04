use std::collections::HashMap;

use crate::value::Function;
use crate::{value::Value, result::InterpretResult};
use crate::opcode::*;

#[derive(Debug)]
pub struct VM {
    frames:         Vec<CallFrame>,
    stack:          Vec<Value>,
    // Global variables are late bound in rlox. 
    // “Late” in this context means “resolved after compile time”
    globals:    HashMap<String, Value>,
    /// for test
    prints:     Vec<String>,
}

impl<'a> VM {
    pub fn new() -> Self {
        Self {
            frames:         vec![], 
            stack:          vec![], 
            globals:        HashMap::new(),
            prints:         vec![],
        }
    }


    pub fn interpreter(&mut self, function: Function) -> InterpretResult<Vec<String>> {
        self.frames.push(CallFrame::new(function));
        self.run()
    }


    fn run(&mut self) -> InterpretResult<Vec<String>> {
        loop {
            
            self.print_stack();
            let frame = self.frames.last_mut().unwrap();
            
            let instruction = Self::read_byte(frame);
            println!("{}", opcode2string(instruction));
            match instruction {
                OP_RETURN => {
                    let result = self.stack.pop().unwrap();
                    let last_slot = self.frames.pop().unwrap().slots;
                    if self.frames.is_empty() {
                        return InterpretResult::Ok(self.prints.clone());
                    }
                    // recover the stack
                    self.stack.truncate(last_slot);
                    self.stack.push(result);
                }
                OP_CONSTANT => {
                    let constant = Self::read_constant(frame);
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
                    println!("{}", value);
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
                    let name = Self::read_constant(frame);
                    let value = self.stack.pop().unwrap();
                    self.globals.insert(name.to_string(), value);
                    
                }    
                OP_GET_GLOBAL => {
                    let name = Self::read_constant(frame);
                    let value = self.globals.get(&name.to_string());
                    match value {
                        Some(value) => self.stack.push(value.clone()),
                        None => return InterpretResult::RuntimeError(format!("Undefined variable '{}'", name)),
                    }
                }
                OP_SET_GLOBAL => {
                    let name = Self::read_constant(frame);
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
                    let slot = Self::read_byte(frame);
                    let value = self.stack.get(frame.slots + slot as usize).unwrap();
                    self.stack.push(value.clone());
                }
                OP_SET_LOCAL => {
                    let slot = Self::read_byte(frame);
                    let value = self.stack.last().unwrap();
                    self.stack[frame.slots + slot as usize] = value.clone();
                }
                OP_JUMP_IF_FALSE => {
                    let offset = Self::read_short(frame);
                    let condition = self.stack.last().unwrap();
                    if !condition.is_truthy() {
                        frame.ip += offset;
                    }
                }
                OP_JUMP => {
                    let offset = Self::read_short(frame);
                    frame.ip += offset;
                }
                OP_LOOP => {
                    let offset = Self::read_short(frame);
                    frame.ip -= offset;
                }
                OP_CALL => {
                    let argcount = Self::read_byte(frame);
                    let value = self.stack[self.stack.len() - argcount as usize - 1].clone();
                    self.call_value(value, argcount)?;
                }
                _ => {
                    return InterpretResult::RuntimeError("Unknown opcode".to_string()); 
                }
            }
        }
    }

    fn call_value(&mut self, callee: Value, argcount: u8) -> InterpretResult<()> {
        match callee {
            Value::Function(function) => {
                if argcount != function.arity {
                    return InterpretResult::RuntimeError(format!("Expected {} arguments but got {}", function.arity, argcount));
                }
                self.call(function, argcount);
                InterpretResult::Ok(())
            }
            _ => InterpretResult::RuntimeError("Can only call functions and classes.".to_string()), 
        }
    }

    fn call(&mut self, function: Function, argcount: u8) {
        let frame = CallFrame {
            ip: 0,
            function,
            slots: self.stack.len() - argcount as usize -1,
        };
        self.frames.push(frame);
    }

    fn read_byte(frame: &mut CallFrame) -> u8 {
        let instruction = frame.function.chunk.code[frame.ip];
        frame.ip += 1;
        instruction
    }

    fn read_constant(frame: &mut CallFrame) -> Value {
        let index = Self::read_byte(frame) as usize;
        frame.function.chunk.constants.values[index].clone()
    }

    fn read_short(frame: &mut CallFrame) -> usize {
        let offset = (frame.function.chunk.code[frame.ip] as usize) << 8 | frame.function.chunk.code[frame.ip + 1] as usize;
        frame.ip += 2;
        offset
    }

    fn print_stack(&self) {
        for value in &self.stack {
            print!("[ {} ]", value);
        }
        println!("")
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

#[derive(Debug)]
struct CallFrame {
    ip:         usize,
    function:   Function,
    /// frame pointer
    slots:      usize,
}

impl CallFrame {
    fn new(function: Function) -> CallFrame {
        CallFrame {
            ip: 0,
            function,
            slots: 0,
        }
    }
}

