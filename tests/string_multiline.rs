use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
var a = "1
2
3";
print a;
// expect: 1
// expect: 2
// expect: 3

"#;

#[test]
fn test_files_string_multiline() {
    let expected_output = vec!["1","2","3"];
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}