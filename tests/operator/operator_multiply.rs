use rlox::interpreter::run;
use rlox::result::InterpretResult;

const SOURCE: &str = r#"
print 5 * 3; // expect: 15
print 12.34 * 0.3; // expect: 3.702

"#;

#[test]
fn test_files_operator_multiply() {
    let expected_output = vec!["15".to_string(),"3.702".to_string()];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}
