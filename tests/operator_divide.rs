use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
print 8 / 2;         // expect: 4
print 12.34 / 12.34;  // expect: 1

"#;

#[test]
fn test_files_operator_divide() {
    let expected_output = vec!["4".to_string(),"1".to_string()];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}
