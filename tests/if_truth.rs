use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
// False and nil are false.
if (false) print "bad"; else print "false"; // expect: false
if (nil) print "bad"; else print "nil"; // expect: nil

// Everything else is true.
if (true) print true; // expect: true
if (0) print 0; // expect: 0
if ("") print "empty"; // expect: empty

"#;

#[test]
fn test_files_if_truth() {
    let expected_output = vec!["false".to_string(),"nil".to_string(),"true".to_string(),"0".to_string(),"empty".to_string()];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}
