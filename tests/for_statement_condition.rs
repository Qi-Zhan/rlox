use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
// [line 3] Error at '{': Expect expression.
// [line 3] Error at ')': Expect ';' after expression.
for (var a = 1; {}; a = a + 1) {}

"#;

#[test]
fn test_files_for_statement_condition() {
    let expected_output = vec![];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::LexError{..}));
}
