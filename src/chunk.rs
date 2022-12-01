#![allow(dead_code)]

use crate::value::{ValueArray, Value};
use crate::opcode::*;

#[derive(Debug, Clone)]
pub struct Chunk {
    pub code:       Vec<u8>,
    pub constants:  ValueArray,
}

impl Chunk {
    pub fn new() -> Self {
        Self { 
            code: vec![], 
            constants: ValueArray::new(), 
        }
    }

    pub fn write_chunk(&mut self, byte: u8 ) {
        self.code.push(byte);
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.write_valuearray(value);
        self.constants.values.len() - 1
    }

    pub fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{:04} ", offset);
        // if offset > 0 && self.lines[offset] == self.lines[offset - 1] {
        //     print!("   | ");
        // } else {
        //     print!("{:04} ", self.lines[offset]);
        // }

        let instruction = self.code[offset];
        match instruction {
            OP_CONSTANT => {
                let constant = self.code[offset + 1];
                println!("OP_CONSTANT {:04} '{:?}'", constant, self.constants.values[constant as usize]);
                offset + 2
            }
            OP_NEGATE => {
                println!("OP_NEGATE");
                offset + 1
            }
            OP_ADD => {
                println!("OP_ADD");
                offset + 1
            }
            OP_SUBSTRACT => {
                println!("OP_SUBSTRACT");
                offset + 1
            }
            OP_DIVIDE => {
                println!("OP_DIVIDE");
                offset + 1
            }
            OP_RETURN => {
                println!("OP_RETURN");
                offset + 1
            }
            _ => {
                println!("Unknown opcode {}", instruction);
                offset + 1
            }
        }
    }
}

