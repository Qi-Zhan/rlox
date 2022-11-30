use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
// Evaluate the 'then' expression if the condition is true.
if (true) print "good"; // expect: good
if (false) print "bad";

// Allow block body.
if (true) { print "block"; } // expect: block

// Assignment in if condition.
var a = false;
if (a = true) print a; // expect: true

"#;

#[test]
fn test_files_if_if() {
    let expected_output = vec!["good","block","true"];
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}