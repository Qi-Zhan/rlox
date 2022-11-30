use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
class Foo {}

print Foo; // expect: Foo

"#;

#[test]
fn test_files_class_empty() {
    let expected_output = vec!["Foo"];
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}