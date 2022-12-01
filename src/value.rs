#![allow(dead_code)]

use std::{ops::{Add, Sub, Mul, Div, Neg}, fmt::{Display, Formatter}};



#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    Obj(Obj),
    Bool(bool),
    Nil,
}
    
#[derive(Debug, Clone, PartialEq)]
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
            // (Value::String(a), Value::String(b)) => Value::String(a + &b),
            // TODO String add
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

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(number) => write!(f, "{}", number),
            Value::Obj(obj) => write!(f, "{:?}", obj),
            Value::Bool(boolean) => write!(f, "{}", boolean),
            Value::Nil => write!(f, "nil"),
        }
    }
}

