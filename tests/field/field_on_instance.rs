use rlox::interpreter::run;
use rlox::result::InterpretResult;

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
    let expected_output = vec!["bar value".to_string(),"baz value".to_string(),"bar value".to_string(),"baz value".to_string()];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}
