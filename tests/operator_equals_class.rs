use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
// Bound methods have identity equality.
class Foo {}
class Bar {}

print Foo == Foo; // expect: true
print Foo == Bar; // expect: false
print Bar == Foo; // expect: false
print Bar == Bar; // expect: true

print Foo == "Foo"; // expect: false
print Foo == nil;   // expect: false
print Foo == 123;   // expect: false
print Foo == true;  // expect: false

"#;

#[test]
fn test_files_operator_equals_class() {
    let expected_output = vec!["true","false","false","true","false","false","false","false"];
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}