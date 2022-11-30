use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
class Foo {}

var foo = Foo();
print foo; // expect: Foo instance

"#;

#[test]
fn test_files_constructor_default() {
    let expected_output = vec!["Foo instance"];
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}