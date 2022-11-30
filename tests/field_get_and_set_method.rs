use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
// Bound methods have identity equality.
class Foo {
  method(a) {
    print "method";
    print a;
  }
  other(a) {
    print "other";
    print a;
  }
}

var foo = Foo();
var method = foo.method;

// Setting a property shadows the instance method.
foo.method = foo.other;
foo.method(1);
// expect: other
// expect: 1

// The old method handle still points to the original method.
method(2);
// expect: method
// expect: 2

"#;

#[test]
fn test_files_field_get_and_set_method() {
    let expected_output = vec!["other","1","method","2"];
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}