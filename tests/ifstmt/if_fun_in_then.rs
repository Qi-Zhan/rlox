use rlox::interpreter::run;
use rlox::result::InterpretResult;

const SOURCE: &str = r#"
// [line 2] Error at 'fun': Expect expression.
if (true) fun foo() {}

"#;

#[test]
#[ignore="function"]
fn test_files_if_fun_in_then() {
    
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::CompileError{..}));
}
