#![allow(dead_code)]

use crate::chunk::Chunk;
use crate::scanner::Token;
use crate::error::InterpretResult;
use crate::parser::{ParseRule, Parser};
use crate::value::*;
use crate::opcode::*;

#[derive(Debug)]
pub struct Compiler {

    parser: Parser,
    byteemiter: ByteEmiter,

}

impl Compiler {
    pub fn new() -> Self {
        Self { 
            parser: Parser::new(), 
            byteemiter: ByteEmiter::new(),
        }
    }

    pub fn compile(&mut self, tokens: impl Iterator<Item = Token>) -> InterpretResult<Chunk> {
        self.parser.parse(tokens, &mut self.byteemiter);
        self.parser.advance();
        InterpretResult::Ok(self.byteemiter.return_chunk())

    }
}
    

#[derive(Debug)]
pub struct ByteEmiter {
    chunk: Chunk,
}

impl ByteEmiter {
    fn new() -> Self {
        Self { 
            chunk: Chunk::new(),
        }
    }

    pub fn return_chunk(&self) -> Chunk {
        self.chunk.clone()
    }

    fn emit_byte(&mut self, byte: u8) {
        self.chunk.write_chunk(byte);
    }

    fn emit_bytes(&mut self, byte1: u8, byte2: u8) {
        self.emit_byte(byte1);
        self.emit_byte(byte2);
    }

    fn emit_return(&mut self) {
        self.emit_byte(OP_RETURN);
    }

    fn emit_constant(&mut self, value: Value) {
        let constant = self.chunk.add_constant(value);
        if constant > u8::MAX as usize {
            println!("Too many constants in one chunk.");
            return;
        }
        self.emit_bytes(OP_CONSTANT, constant as u8);
    }

    fn emit_constant_number(&mut self, value: f64) {
        self.emit_constant(Value::Number(value));
    }

    fn emit_constant_string(&mut self, value: String) {
        let obj = Obj::Str(value);
        self.emit_constant(Value::Obj(obj));
    }

    fn emit_constant_bool(&mut self, value: bool) {
        self.emit_constant(Value::Bool(value));
    }

    fn emit_constant_nil(&mut self) {
        self.emit_constant(Value::Nil);
    }


    // fn emit_constant_array(&mut self, value: Vec<Value>) {
    //     self.emit_constant(Value::Array(value));
    // }

    // fn emit_constant_hash(&mut self, value: Vec<(Value, Value)>) {
    //     self.emit_constant(Value::Hash(value));
    // }

    // fn emit_constant_function(&mut self, value: Vec<u8>) {
    //     self.emit_constant(Value::Function(value));
    // }

    // fn emit_constant_native(&mut self, value: fn(Vec<Value>) -> Value) {
    //     self.emit_constant(Value::Native(value));
    // }




}


