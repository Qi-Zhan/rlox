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
    let expected_output = vec!["1".to_string(),"2".to_string(),"3".to_string()];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}
