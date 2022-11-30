use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
print nil == nil; // expect: true

print true == true; // expect: true
print true == false; // expect: false

print 1 == 1; // expect: true
print 1 == 2; // expect: false

print "str" == "str"; // expect: true
print "str" == "ing"; // expect: false

print nil == false; // expect: false
print false == 0; // expect: false
print 0 == "0"; // expect: false

"#;

#[test]
fn test_files_operator_equals() {
    let expected_output = vec!["true","true","false","true","false","true","false","false","false","false"];
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}