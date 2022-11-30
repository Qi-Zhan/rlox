use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
// [line 2] Error at 'class': Expect expression.
if (true) "ok"; else class Foo {}

"#;

#[test]
fn test_files_if_class_in_else() {
    
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::LexerError{..}));
}