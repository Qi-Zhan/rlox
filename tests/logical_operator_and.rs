use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
// Note: These tests implicitly depend on ints being truthy.

// Return the first non-true argument.
print false and 1; // expect: false
print true and 1; // expect: 1
print 1 and 2 and false; // expect: false

// Return the last argument if all are true.
print 1 and true; // expect: true
print 1 and 2 and 3; // expect: 3

// Short-circuit at the first false argument.
var a = "before";
var b = "before";
(a = true) and
    (b = false) and
    (a = "bad");
print a; // expect: true
print b; // expect: false

"#;

#[test]
fn test_files_logical_operator_and() {
    let expected_output = vec!["false".to_string(),"1".to_string(),"false".to_string(),"true".to_string(),"3".to_string(),"true".to_string(),"false".to_string()];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}
