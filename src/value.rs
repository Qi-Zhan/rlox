#![allow(dead_code)]

use std::{ops::*, fmt::{Display, Formatter}, cmp::Ordering};

use crate::result::InterpretResult;


#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    Obj(Obj),
    Bool(bool),
    Nil,
}
    
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Obj {
    Str(String),
    Var(String),
    Function,
    Native,
    Array,
    Hash,
    Class,
    Instance,

}

#[derive(Debug, Clone)]
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
    
}

impl Add for Value {
    type Output = InterpretResult<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => InterpretResult::Ok(Value::Number(a + b)),
            (Value::Obj(Obj::Str(a)), Value::Obj(Obj::Str(b))) => InterpretResult::Ok(Value::Obj(Obj::Str(a + &b))),
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
            (Value::Obj(Obj::Str(a)), Value::Obj(Obj::Str(b))) => a.partial_cmp(b),

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
            Value::Obj(Obj::Str(s)) => write!(f, "{}", s),
            Value::Obj(Obj::Var(s)) => write!(f, "{}", s),
            Value::Obj(obj) => write!(f, "{:?}", obj),
            Value::Bool(boolean) => write!(f, "{}", boolean),
            Value::Nil => write!(f, "nil"), 
        }
    }
}

