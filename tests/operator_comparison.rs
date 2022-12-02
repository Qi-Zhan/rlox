use rlox::interpreter::run;
use rlox::result::InterpretResult;

const SOURCE: &str = r#"
print 1 < 2;    // expect: true
print 2 < 2;    // expect: false
print 2 < 1;    // expect: false

print 1 <= 2;    // expect: true
print 2 <= 2;    // expect: true
print 2 <= 1;    // expect: false

print 1 > 2;    // expect: false
print 2 > 2;    // expect: false
print 2 > 1;    // expect: true

print 1 >= 2;    // expect: false
print 2 >= 2;    // expect: true
print 2 >= 1;    // expect: true

// Zero and negative zero compare the same.
print 0 < -0; // expect: false
print -0 < 0; // expect: false
print 0 > -0; // expect: false
print -0 > 0; // expect: false
print 0 <= -0; // expect: true
print -0 <= 0; // expect: true
print 0 >= -0; // expect: true
print -0 >= 0; // expect: true

"#;

#[test]
fn test_files_operator_comparison() {
    let expected_output = vec!["true".to_string(),"false".to_string(),"false".to_string(),"true".to_string(),"true".to_string(),"false".to_string(),"false".to_string(),"false".to_string(),"true".to_string(),"false".to_string(),"true".to_string(),"true".to_string(),"false".to_string(),"false".to_string(),"false".to_string(),"false".to_string(),"true".to_string(),"true".to_string(),"true".to_string(),"true".to_string()];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}
