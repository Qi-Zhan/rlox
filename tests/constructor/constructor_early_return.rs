use rlox::interpreter::run;
use rlox::result::InterpretResult;

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
    let expected_output = vec!["init".to_string(),"Foo instance".to_string()];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}
