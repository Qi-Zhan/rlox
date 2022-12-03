use rlox::interpreter::run;
use rlox::result::InterpretResult;

const SOURCE: &str = r#"
fun returnArg(arg) {
  return arg;
}

fun returnFunCallWithArg(func, arg) {
  return returnArg(func)(arg);
}

fun printArg(arg) {
  print arg;
}

returnFunCallWithArg(printArg, "hello world"); // expect: hello world

"#;

#[test]
fn test_files_function_nested_call_with_arguments() {
    let expected_output = vec!["hello world".to_string()];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}
