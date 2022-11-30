use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
class Foo {
  init() {
    print "init";
    return;
    print "nope";
  }
}

var foo = Foo(); // expect: init
print foo; // expect: Foo instance

"#;

#[test]
fn test_files_constructor_early_return() {
    let expected_output = vec!["init","Foo instance"];
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}