use rlox::interpreter::run;
use rlox::result::InterpretResult;

const SOURCE: &str = r#"
var a = "a";
!a = "value"; // Error at '=': Invalid assignment target.

"#;

#[test]
fn test_files_assignment_prefix_operator() {
    
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::CompileError{..}));
}
