use rlox::interpreter::run;
use rlox::result::InterpretResult;

const SOURCE: &str = r#"
var a = "a";
var b = "b";
a + b = "value"; // Error at '=': Invalid assignment target.

"#;

#[test]
#[ignore]
fn test_files_assignment_infix_operator() {
    
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::CompileError{..}));
}
