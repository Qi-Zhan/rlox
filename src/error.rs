
#[derive(Debug, Clone, PartialEq)]
pub enum InterpretResult<T> {
    Ok(T),
    LexError(String),
    CompileError(String),
    RuntimeError(String),

}
