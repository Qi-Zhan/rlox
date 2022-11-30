use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
// [line 2] Error at 'fun': Expect expression.
for (;;) fun foo() {}

"#;

#[test]
fn test_files_for_fun_in_body() {
    
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::LexerError{..}));
}