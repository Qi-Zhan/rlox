#![allow(dead_code)]

use std::{ops::*, fmt::{Display, Formatter}, cmp::Ordering};



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
    fn new_number(value: f64) -> Self {
        Self::Number(value)
    }

    fn new_obj(value: Obj) -> Self {
        Self::Obj(value)
    }

    fn new_bool(value: bool) -> Self {
        Self::Bool(value)
    }

    fn new_nil() -> Self {
        Self::Nil
    }
    
}

impl Add for Value {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a + b),
            (Value::Obj(Obj::Str(a)), Value::Obj(Obj::Str(b))) => Value::Obj(Obj::Str(a + &b)),
            _ => panic!("Invalid operands"),
        }
    }
    
}

impl Sub for Value {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a - b),
            _ => panic!("Invalid operands"),
        }
    }
    
}

impl Mul for Value {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a * b),
            _ => panic!("Invalid operands"),
        }
    }
    
}

impl Div for Value {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a / b),
            _ => panic!("Invalid operands"),
        }
    }
    
}

impl Neg for Value {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Value::Number(a) => Value::Number(-a),
            _ => panic!("Invalid operands"),
        }
    }
    
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a.partial_cmp(b),
            _ => panic!("Invalid operands"),
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
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Bool(a), Value::Bool(b)) => Value::Bool(a & b),
            _ => panic!("Invalid operands"),
        }
    }
    
}

impl BitOr for Value {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Bool(a), Value::Bool(b)) => Value::Bool(a | b),
            _ => panic!("Invalid operands"),
        }
    }
    
}

impl Not for Value {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Value::Bool(a) => Value::Bool(!a),
            _ => panic!("Invalid operands"),
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
            Value::Obj(obj) => write!(f, "{:?}", obj),
            Value::Bool(boolean) => write!(f, "{}", boolean),
            Value::Nil => write!(f, "nil"), 
        }
    }
}

