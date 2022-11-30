use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
print nil != nil; // expect: false

print true != true; // expect: false
print true != false; // expect: true

print 1 != 1; // expect: false
print 1 != 2; // expect: true

print "str" != "str"; // expect: false
print "str" != "ing"; // expect: true

print nil != false; // expect: true
print false != 0; // expect: true
print 0 != "0"; // expect: true

"#;

#[test]
fn test_files_operator_not_equals() {
    let expected_output = vec!["false","false","true","false","true","false","true","true","true","true"];
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}