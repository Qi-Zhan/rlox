#[derive(Debug, Clone, PartialEq)]
pub enum InterpretResult<'a, T> {
    Ok(T),
    LexerError(&'a str),
    CompilerError(&'a str),
    RuntimeError(&'a str),

}