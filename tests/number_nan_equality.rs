use rlox::interpreter::run;
use rlox::result::InterpretResult;

const SOURCE: &str = r#"


print 0/0 == 0; // expect: false
print 0/0 != 1; // expect: true

// NaN is not equal to self.
print 0/0 == 0/0; // expect: false
print 0/0 != 0/0; // expect: true

"#;

#[test]
fn test_files_number_nan_equality() {
    let expected_output = vec!["false".to_string(),"true".to_string(),"false".to_string(),"true".to_string()];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}
