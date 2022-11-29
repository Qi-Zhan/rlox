use crate::error::InterpretResult;
use crate::scanner::tokenize;

pub fn run(string:&str) -> InterpretResult<()> {
    println!("{}", string);
    let mut tokens = tokenize(string);
    for token in tokens {
        println!("{:?}", token)
    }
    InterpretResult::Ok(())
}

