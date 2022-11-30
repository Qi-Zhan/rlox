use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
var a = "a";
(a) = "value"; // Error at '=': Invalid assignment target.

"#;

#[test]
fn test_files_assignment_grouping() {
    
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::CompilerError{..}));
}