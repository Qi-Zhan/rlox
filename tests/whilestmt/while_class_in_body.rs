use rlox::interpreter::run;
use rlox::result::InterpretResult;

const SOURCE: &str = r#"
// [line 2] Error at 'class': Expect expression.
while (true) class Foo {}

"#;

#[test]
fn test_files_while_class_in_body() {
    
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::CompileError{..}));
}
