use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
print 123 + 456; // expect: 579
print "str" + "ing"; // expect: string

"#;

#[test]
fn test_files_operator_add() {
    let expected_output = vec!["579","string"];
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}