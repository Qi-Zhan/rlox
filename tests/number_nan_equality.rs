use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
var nan = 0/0;

print nan == 0; // expect: false
print nan != 1; // expect: true

// NaN is not equal to self.
print nan == nan; // expect: false
print nan != nan; // expect: true

"#;

#[test]
fn test_files_number_nan_equality() {
    let expected_output = vec!["false".to_string(),"true".to_string(),"false".to_string(),"true".to_string()];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}
