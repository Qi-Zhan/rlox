use std::{ops::*, fmt::{Display, Formatter}, cmp::Ordering};

use crate::{result::InterpretResult, chunk::Chunk};
use crate::opcode::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    Bool(bool),
    Nil,
    String(String),
    Function(Function),
}

#[derive(Debug, Clone, PartialEq)]
pub enum FunctionType {
    Function,
    Script,
}

#[derive(Debug, Clone, PartialEq)]
pub struct  Function {
    name:   String,
    arity:  usize,
    pub chunk:  Chunk,
}

impl Function {
    pub fn new(name: String, arity: usize) -> Self {
        Self { name, arity, chunk: Chunk::new() }
    }

    pub fn emit_byte(&mut self, byte: u8) {
        self.chunk.write_chunk(byte);
    }

    pub fn emit_bytes(&mut self, byte1: u8, byte2: u8) {
        self.emit_byte(byte1);
        self.emit_byte(byte2);
    }

    pub fn emit_return(&mut self) {
        self.emit_byte(OP_RETURN);
    }

    pub fn emit_jump(&mut self, instruction: u8) -> usize {
        self.emit_byte(instruction);
        self.emit_byte(0xff);
        self.emit_byte(0xff);
        self.chunk.code.len() - 2
    }

    pub fn emit_loop(&mut self, loop_start: usize) {
        self.emit_byte(OP_LOOP);
        let offset = self.chunk.code.len() - loop_start + 2;
        self.emit_byte((offset >> 8) as u8);
        self.emit_byte((offset & 0xff) as u8);
    }

    pub fn patch_jump(&mut self, offset: usize) {
        let jump = self.chunk.code.len() - offset - 2;
        self.chunk.code[offset] = ((jump >> 8) & 0xff) as u8;
        self.chunk.code[offset + 1] = (jump & 0xff) as u8;
    }

    pub fn emit_constant(&mut self, value: Value) {
        let constant = self.chunk.add_constant(value);
        self.emit_bytes(OP_CONSTANT, constant as u8);
    }

    pub fn make_string(&mut self, value: String) -> u8 {
        let constant = self.chunk.add_constant(Value::String(value));
        constant as u8
    }

    pub fn make_constant(&mut self, value: Value) -> u8 {
        let constant = self.chunk.add_constant(value);
        constant as u8
    }

}

#[derive(Debug, Clone, PartialEq)]
pub struct ValueArray {
    pub values: Vec<Value>,
}

impl ValueArray {
    pub fn new() -> Self {
        Self { values: vec![] }
    }

    pub fn write_valuearray(&mut self, value: Value) {
        self.values.push(value);
    }
    
}

impl Value {
    pub fn is_number(&self) -> bool {
        match self {
            Value::Number(_) => true,
            _ => false,
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Nil => false,
            Value::Bool(b) => *b,
            _ => true,
        }
    }
    
}

impl Add for Value {
    type Output = InterpretResult<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => InterpretResult::Ok(Value::Number(a + b)),
            (Value::String(a), Value::String(b)) => InterpretResult::Ok(Value::String(a + &b)),
            _ => InterpretResult::RuntimeError("Operands must be two numbers or two strings.".to_string()),
        }
    }
    
}

impl Sub for Value {
    type Output = InterpretResult<Self>;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => InterpretResult::Ok(Value::Number(a - b)),
            _ => InterpretResult::RuntimeError("Operands must be two numbers.".to_string()),
        }
    }
    
}

impl Mul for Value {
    type Output = InterpretResult<Self>;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => InterpretResult::Ok(Value::Number(a * b)),
            _ => InterpretResult::RuntimeError("Operands must be two numbers.".to_string()),
        }
    }
    
}

impl Div for Value {
    type Output = InterpretResult<Self>;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => InterpretResult::Ok(Value::Number(a / b)),
            _ => InterpretResult::RuntimeError("Operands must be two numbers.".to_string()),
        }
    }
    
}

impl Neg for Value {
    type Output = InterpretResult<Self>;

    fn neg(self) -> Self::Output {
        match self {
            Value::Number(a) => InterpretResult::Ok(Value::Number(-a)),
            _ => InterpretResult::RuntimeError("Operand must be a number.".to_string()),
        }
    }
    
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a.partial_cmp(b),
            _ => None,
        }
    }
}

impl Eq for Value {

}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a.partial_cmp(b).unwrap(),
            _ => panic!("Invalid operands"),
        }
    }
}

impl BitAnd for Value {
    type Output = InterpretResult<Self>;

    fn bitand(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Bool(true), b) => InterpretResult::Ok(b),
            (Value::Bool(false), _) => InterpretResult::Ok(Value::Bool(false)),
            (Value::Nil, _) => InterpretResult::Ok(Value::Nil),
            (_, b) => InterpretResult::Ok(b),
        }
    }
    
}

impl BitOr for Value {
    type Output = InterpretResult<Self>;

    fn bitor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Bool(true), _) => InterpretResult::Ok(Value::Bool(true)),
            (Value::Bool(false), b) => InterpretResult::Ok(b),
            (Value::Nil, b) => InterpretResult::Ok(b),
            (a, _) => InterpretResult::Ok(a),
        }
    }
    
}

impl Not for Value {
    type Output = InterpretResult<Self>;

    fn not(self) -> Self::Output {
        match self {
            Value::Bool(a) => InterpretResult::Ok(Value::Bool(!a)),
            _ => InterpretResult::RuntimeError("Operand must be a boolean.".to_string()),
        }
    }
    
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(number) => {
                if *number as i64 as f64 == *number {
                    write!(f, "{}", *number as i64)
                } else {
                    write!(f, "{}", number)
                }
            }
            Value::String(string) => write!(f, "{}", string),
            Value::Bool(boolean) => write!(f, "{}", boolean),
            Value::Nil => write!(f, "nil"), 
            Value::Function(function) => {
                if function.name == "" {
                    write!(f, "<script>")
                } else {
                    write!(f, "<fn {}>", function.name)
                }
            }
        }
    }
}

