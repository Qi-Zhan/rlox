use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
class Foo {
  inFoo() {
    print "in foo";
  }
}

class Bar < Foo {
  inBar() {
    print "in bar";
  }
}

class Baz < Bar {
  inBaz() {
    print "in baz";
  }
}

var baz = Baz();
baz.inFoo(); // expect: in foo
baz.inBar(); // expect: in bar
baz.inBaz(); // expect: in baz

"#;

#[test]
fn test_files_class_inherited_method() {
    let expected_output = vec!["in foo","in bar","in baz"];
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}