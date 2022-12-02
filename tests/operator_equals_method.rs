use rlox::interpreter::run;
use rlox::result::InterpretResult;

const SOURCE: &str = r#"
// Bound methods have identity equality.
class Foo {
  method() {}
}

var foo = Foo();
var fooMethod = foo.method;

// Same bound method.
print fooMethod == fooMethod; // expect: true

// Different closurizations.
print foo.method == foo.method; // expect: false

"#;

#[test]
#[ignore = "class"]
fn test_files_operator_equals_method() {
    let expected_output = vec!["true".to_string(),"false".to_string()];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}
