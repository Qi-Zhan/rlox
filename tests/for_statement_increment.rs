use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
// [line 2] Error at '{': Expect expression.
for (var a = 1; a < 2; {}) {}

"#;

#[test]
fn test_files_for_statement_increment() {
    
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::LexerError{..}));
}