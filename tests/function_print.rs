use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
fun foo() {}
print foo; // expect: <fn foo>

print clock; // expect: <native fn>

"#;

#[test]
fn test_files_function_print() {
    let expected_output = vec!["<fn foo>","<native fn>"];
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}