use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
// A dangling else binds to the right-most if.
if (true) if (false) print "bad"; else print "good"; // expect: good
if (false) if (true) print "bad"; else print "bad";

"#;

#[test]
fn test_files_if_dangling_else() {
    let expected_output = vec!["good".to_string()];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}
