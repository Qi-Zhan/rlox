use rlox::interpreter::run;
use rlox::result::InterpretResult;

const SOURCE: &str = r#"
fun foo() {}
print foo; // expect: <fn foo>

print clock; // expect: <native fn>

"#;

#[test]
#[ignore = "native"]
fn test_files_function_print() {
    let expected_output = vec!["<fn foo>".to_string(),"<native fn>".to_string()];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}
