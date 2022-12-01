use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
// [line 2] Error at 'fun': Expect expression.
if (true) "ok"; else fun foo() {}

"#;

#[test]
fn test_files_if_fun_in_else() {
    
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::LexError{..}));
}
