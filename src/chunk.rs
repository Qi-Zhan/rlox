#![allow(dead_code)]

use crate::value::{ValueArray, Value};

#[derive(Debug, Clone)]
pub struct Chunk {
    pub code:       Vec<u8>,
    pub lines:      Vec<usize>,
    pub constants:  ValueArray,
}

impl Chunk {
    pub fn new() -> Self {
        Self { 
            code: vec![], 
            constants: ValueArray::new(), 
            lines: vec![] 
        }
    }

    pub fn write_chunk(&mut self, byte: u8, line:usize) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.write_valuearray(value);
        self.constants.values.len() - 1
    }
}

