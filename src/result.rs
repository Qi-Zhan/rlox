use std::ops::{Try, FromResidual, ControlFlow};

#[derive(Debug, Clone, PartialEq)]
pub enum InterpretResult<T, E = String> {
    Ok(T),
    LexError(E),
    CompileError(E),
    RuntimeError(E),

}

impl<T, E> Try for InterpretResult<T, E> {
    type Output = T;
    type Residual = InterpretResult<!, E>;

    #[inline]
    fn from_output(c: T) -> Self {
        InterpretResult::Ok(c)
    }

    #[inline]
    fn branch(self) -> ControlFlow<Self::Residual, T> {
        match self {
            InterpretResult::Ok(c) => ControlFlow::Continue(c),
            InterpretResult::LexError(e) => ControlFlow::Break(InterpretResult::LexError(e)),
            InterpretResult::CompileError(e) => ControlFlow::Break(InterpretResult::CompileError(e)),
            InterpretResult::RuntimeError(e) => ControlFlow::Break(InterpretResult::RuntimeError(e)),
        }
    }
}

impl<T, E, F: From<E>> FromResidual<InterpretResult<!, E>> for InterpretResult<T, F> {
    fn from_residual(x: InterpretResult<!, E>) -> Self {
        match x {
            InterpretResult::LexError(e) => InterpretResult::LexError(e.into()),
            InterpretResult::CompileError(e) => InterpretResult::CompileError(e.into()),
            InterpretResult::RuntimeError(e) => InterpretResult::RuntimeError(e.into()),
            InterpretResult::Ok(_) => unreachable!(),
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_trait_test() {
        fn test() -> InterpretResult<i32> {
            let a = 1;
            let b = 2;
            let c = a + b;
            if c == 3 {
                InterpretResult::Ok(c)
            } else {
                InterpretResult::RuntimeError("error".to_string())
            }
        }

        assert_eq!(test(), InterpretResult::Ok(3));
    }

    #[test]
    fn try_trait_test2() {
        assert_eq!(InterpretResult::Ok::<_, String>(3).branch(), ControlFlow::Continue(3));
        assert_eq!(
            InterpretResult::CompileError::<String, _>(3).branch(), 
            ControlFlow::Break(InterpretResult::CompileError(3))
        );
        
    }

}

