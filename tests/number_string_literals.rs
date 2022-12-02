use rlox::interpreter::run;
use rlox::result::InterpretResult;

const SOURCE: &str = r#"
print 123;     // expect: 123
print 987654;  // expect: 987654
print 0;       // expect: 0
print "-0";      // expect: -0

print 123.456; // expect: 123.456
print -0.001;  // expect: -0.001

"#;

#[test]
fn test_files_number_literals() {
    let expected_output = vec!["123".to_string(),"987654".to_string(),"0".to_string(),"-0".to_string(),"123.456".to_string(),"-0.001".to_string()];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}
