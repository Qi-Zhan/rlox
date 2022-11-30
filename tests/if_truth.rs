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
    let expected_output = vec!["false","nil","true","0","empty"];
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}