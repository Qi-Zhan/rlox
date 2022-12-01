use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
class Foo {
  init(arg) {
    print "Foo.init(" + arg + ")";
    this.field = "init";
  }
}

var foo = Foo("one"); // expect: Foo.init(one)
foo.field = "field";

var foo2 = foo.init("two"); // expect: Foo.init(two)
print foo2; // expect: Foo instance

// Make sure init() doesn't create a fresh instance.
print foo.field; // expect: init

"#;

#[test]
fn test_files_constructor_call_init_explicitly() {
    let expected_output = vec!["Foo.init(one)".to_string(),"Foo.init(two)".to_string(),"Foo instance".to_string(),"init".to_string()];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}
