use rlox::interpreter::run;
use rlox::result::InterpretResult;

const SOURCE: &str = r#"
// [line 2] Error at 'fun': Expect expression.
while (true) fun foo() {}

"#;

#[test]
#[ignore="function"]
fn test_files_while_fun_in_body() {
    
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::CompileError{..}));
}
