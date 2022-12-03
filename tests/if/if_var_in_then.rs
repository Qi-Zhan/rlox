use rlox::interpreter::run;
use rlox::result::InterpretResult;

const SOURCE: &str = r#"
// [line 2] Error at 'var': Expect expression.
if (true) var foo;

"#;

#[test]
fn test_files_if_var_in_then() {
    
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::CompileError{..}));
}
