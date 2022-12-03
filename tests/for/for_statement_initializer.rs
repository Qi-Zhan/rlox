use rlox::interpreter::run;
use rlox::result::InterpretResult;

const SOURCE: &str = r#"
// [line 3] Error at '{': Expect expression.
// [line 3] Error at ')': Expect ';' after expression.
for ({}; a < 2; a = a + 1) {}

"#;

#[test]
fn test_files_for_statement_initializer() {
    
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::CompileError{..}));
}
