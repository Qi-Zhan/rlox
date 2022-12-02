use rlox::interpreter::run;
use rlox::result::InterpretResult;

const SOURCE: &str = r#"
print 123 + 456; // expect: 579
print "str" + "ing"; // expect: string

"#;

#[test]
fn test_files_operator_add() {
    let expected_output = vec!["579".to_string(),"string".to_string()];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}
