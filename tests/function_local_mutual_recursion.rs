use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
{
  fun isEven(n) {
    if (n == 0) return true;
    return isOdd(n - 1); // expect runtime error: Undefined variable 'isOdd'.
  }

  fun isOdd(n) {
    if (n == 0) return false;
    return isEven(n - 1);
  }

  isEven(4);
}
"#;

#[test]
fn test_files_function_local_mutual_recursion() {
    
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::RuntimeError{..}));
}
