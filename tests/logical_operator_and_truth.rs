use rlox::interpreter::run;
use rlox::result::InterpretResult;

const SOURCE: &str = r#"
// False and nil are false.
print false and "bad"; // expect: false
print nil and "bad"; // expect: nil

// Everything else is true.
print true and "ok"; // expect: ok
print 0 and "ok"; // expect: ok
print "" and "ok"; // expect: ok

"#;

#[test]
fn test_files_logical_operator_and_truth() {
    let expected_output = vec!["false".to_string(),"nil".to_string(),"ok".to_string(),"ok".to_string(),"ok".to_string()];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}
