use crate::chunk::Chunk;
use crate::compiler::Compiler;
use crate::vm::VM;
use crate::result::InterpretResult;
use crate::scanner::{tokenize, Token};

// #[cfg(not(test))]
pub fn run(string:&str) -> InterpretResult<Vec<String>> {
    // println!("{}", string);
    let mut tokens = tokenize(string);
    for token in tokens.by_ref() {
        println!("{:?}", token);
        if let Token::Error(s) = token {
            return InterpretResult::LexError(s);
        }
    }

    let mut compiler = Compiler::new();
    let chunk = compiler.compile(tokens)?;
    println!("{:#?}", chunk);
    let mut vm = VM::new(Chunk::new());
    let result = vm.interpreter(chunk)?;
    println!("{:#?}", result);
    InterpretResult::Ok(result)
    
}

