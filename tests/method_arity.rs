use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
class Foo {
  method0() { return "no args"; }
  method1(a) { return a; }
  method2(a, b) { return a + b; }
  method3(a, b, c) { return a + b + c; }
  method4(a, b, c, d) { return a + b + c + d; }
  method5(a, b, c, d, e) { return a + b + c + d + e; }
  method6(a, b, c, d, e, f) { return a + b + c + d + e + f; }
  method7(a, b, c, d, e, f, g) { return a + b + c + d + e + f + g; }
  method8(a, b, c, d, e, f, g, h) { return a + b + c + d + e + f + g + h; }
}

var foo = Foo();
print foo.method0(); // expect: no args
print foo.method1(1); // expect: 1
print foo.method2(1, 2); // expect: 3
print foo.method3(1, 2, 3); // expect: 6
print foo.method4(1, 2, 3, 4); // expect: 10
print foo.method5(1, 2, 3, 4, 5); // expect: 15
print foo.method6(1, 2, 3, 4, 5, 6); // expect: 21
print foo.method7(1, 2, 3, 4, 5, 6, 7); // expect: 28
print foo.method8(1, 2, 3, 4, 5, 6, 7, 8); // expect: 36

"#;

#[test]
fn test_files_method_arity() {
    let expected_output = vec!["no args","1","3","6","10","15","21","28","36"];
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}