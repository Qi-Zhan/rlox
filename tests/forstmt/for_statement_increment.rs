use rlox::interpreter::run;
use rlox::result::InterpretResult;

const SOURCE: &str = r#"
// [line 2] Error at '{': Expect expression.
for (var a = 1; a < 2; {}) {}

"#;

#[test]
fn test_files_for_statement_increment() {
    
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::CompileError{..}));
}
