use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
class Foo {
  foo(a, b) {
    this.field1 = a;
    this.field2 = b;
  }

  fooPrint() {
    print this.field1;
    print this.field2;
  }
}

class Bar < Foo {
  bar(a, b) {
    this.field1 = a;
    this.field2 = b;
  }

  barPrint() {
    print this.field1;
    print this.field2;
  }
}

var bar = Bar();
bar.foo("foo 1", "foo 2");
bar.fooPrint();
// expect: foo 1
// expect: foo 2

bar.bar("bar 1", "bar 2");
bar.barPrint();
// expect: bar 1
// expect: bar 2

bar.fooPrint();
// expect: bar 1
// expect: bar 2

"#;

#[test]
fn test_files_inheritance_set_fields_from_base_class() {
    let expected_output = vec!["foo 1","foo 2","bar 1","bar 2","bar 1","bar 2"];
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}