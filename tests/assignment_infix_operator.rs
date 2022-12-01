use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
var a = "a";
var b = "b";
a + b = "value"; // Error at '=': Invalid assignment target.

"#;

#[test]
fn test_files_assignment_infix_operator() {
    let expected_output = vec![];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::CompileError{..}));
}
