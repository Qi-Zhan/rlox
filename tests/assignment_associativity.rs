use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
var a = "a";
var b = "b";
var c = "c";

// Assignment is right-associative.
a = b = c;
print a; // expect: c
print b; // expect: c
print c; // expect: c

"#;

#[test]
fn test_files_assignment_associativity() {
    let expected_output = vec!["c","c","c"];
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}