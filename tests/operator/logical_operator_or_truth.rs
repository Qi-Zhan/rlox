use rlox::interpreter::run;
use rlox::result::InterpretResult;

const SOURCE: &str = r#"
// False and nil are false.
print false or "ok"; // expect: ok
print nil or "ok"; // expect: ok

// Everything else is true.
print true or "ok"; // expect: true
print 0 or "ok"; // expect: 0
print "s" or "ok"; // expect: s

"#;

#[test]
fn test_files_logical_operator_or_truth() {
    let expected_output = vec!["ok".to_string(),"ok".to_string(),"true".to_string(),"0".to_string(),"s".to_string()];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}
