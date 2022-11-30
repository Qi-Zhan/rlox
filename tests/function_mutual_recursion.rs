use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
fun isEven(n) {
  if (n == 0) return true;
  return isOdd(n - 1);
}

fun isOdd(n) {
  if (n == 0) return false;
  return isEven(n - 1);
}

print isEven(4); // expect: true
print isOdd(3); // expect: true

"#;

#[test]
fn test_files_function_mutual_recursion() {
    let expected_output = vec!["true","true"];
    let result: InterpretResult<Vec<&str>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}