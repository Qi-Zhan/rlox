use rlox::interpreter::run;
use rlox::result::InterpretResult;

const SOURCE: &str = r#"
// [line 2] Error at 'class': Expect expression.
if (true) class Foo {}

"#;

#[test]
#[ignore = "class"]
fn test_files_if_class_in_then() {
    
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::CompileError{..}));
}
