use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
print 123;     // expect: 123
print 987654;  // expect: 987654
print 0;       // expect: 0
print -0;      // expect: -0

print 123.456; // expect: 123.456
print -0.001;  // expect: -0.001

"#;

#[test]
fn test_files_number_literals() {
    let expected_output = vec!["123","987654","0","-0","123.456","-0.001"];
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}