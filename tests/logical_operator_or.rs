use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
// Note: These tests implicitly depend on ints being truthy.

// Return the first true argument.
print 1 or true; // expect: 1
print false or 1; // expect: 1
print false or false or true; // expect: true

// Return the last argument if all are false.
print false or false; // expect: false
print false or false or false; // expect: false

// Short-circuit at the first true argument.
var a = "before";
var b = "before";
(a = false) or
    (b = true) or
    (a = "bad");
print a; // expect: false
print b; // expect: true

"#;

#[test]
fn test_files_logical_operator_or() {
    let expected_output = vec!["1","1","true","false","false","false","true"];
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}