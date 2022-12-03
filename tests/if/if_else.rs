use rlox::interpreter::run;
use rlox::result::InterpretResult;

const SOURCE: &str = r#"
// Evaluate the 'else' expression if the condition is false.
if (true) print "good"; else print "bad"; // expect: good
if (false) print "bad"; else print "good"; // expect: good

// Allow block body.
if (false) nil; else { print "block"; } // expect: block

"#;

#[test]
fn test_files_if_else() {
    let expected_output = vec!["good".to_string(),"good".to_string(),"block".to_string()];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}
