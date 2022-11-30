use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
class Foo {}

var foo = Foo();

print foo.bar = "bar value"; // expect: bar value
print foo.baz = "baz value"; // expect: baz value

print foo.bar; // expect: bar value
print foo.baz; // expect: baz value

"#;

#[test]
fn test_files_field_on_instance() {
    let expected_output = vec!["bar value","baz value","bar value","baz value"];
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}