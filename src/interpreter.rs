use crate::chunk::Chunk;
use crate::compiler::Compiler;
use crate::vm::VM;
use crate::error::InterpretResult;
use crate::scanner::tokenize;

// #[cfg(not(test))]
pub fn run(string:&str) -> InterpretResult<Vec<String>> {
    // println!("{}", string);
    let mut tokens = tokenize(string);
    for token in tokens.by_ref() {
        println!("{:?}", token)
    }

    let mut compiler = Compiler::new();
    let chunk = compiler.compile(tokens);

    match chunk {
        InterpretResult::Ok(chunk) => {
            println!("{:?}", chunk);
            let mut vm = VM::new(Chunk::new());
            let result = vm.interpreter(chunk);
            match result {
                InterpretResult::Ok(prints) => {
                    println!("{:?}", prints);
                    InterpretResult::Ok(prints)
                }
                InterpretResult::LexError(error) => InterpretResult::LexError(error),
                InterpretResult::CompileError(error) => InterpretResult::CompileError(error),
                InterpretResult::RuntimeError(error) => InterpretResult::RuntimeError(error),
            }
        }
        InterpretResult::LexError(_err) => InterpretResult::LexError(_err),
        InterpretResult::CompileError(_err) => InterpretResult::CompileError(_err),
        InterpretResult::RuntimeError(_err) => InterpretResult::RuntimeError(_err),
    }

}

// #[cfg(test)]
// pub fn run(string:&str) -> InterpretResult<Vec<&str>> {
//     println!("{}", string);
//     let tokens = tokenize(string);
//     for token in tokens {
//         println!("{:?}", token)
//     }
//     InterpretResult::Ok(vec![])  // todo
// }
