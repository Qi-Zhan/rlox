use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
class Bar {}
print !Bar;      // expect: false
print !Bar();    // expect: false

"#;

#[test]
fn test_files_operator_not_class() {
    let expected_output = vec!["false","false"];
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}