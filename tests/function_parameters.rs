use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
fun f0() { return 0; }
print f0(); // expect: 0

fun f1(a) { return a; }
print f1(1); // expect: 1

fun f2(a, b) { return a + b; }
print f2(1, 2); // expect: 3

fun f3(a, b, c) { return a + b + c; }
print f3(1, 2, 3); // expect: 6

fun f4(a, b, c, d) { return a + b + c + d; }
print f4(1, 2, 3, 4); // expect: 10

fun f5(a, b, c, d, e) { return a + b + c + d + e; }
print f5(1, 2, 3, 4, 5); // expect: 15

fun f6(a, b, c, d, e, f) { return a + b + c + d + e + f; }
print f6(1, 2, 3, 4, 5, 6); // expect: 21

fun f7(a, b, c, d, e, f, g) { return a + b + c + d + e + f + g; }
print f7(1, 2, 3, 4, 5, 6, 7); // expect: 28

fun f8(a, b, c, d, e, f, g, h) { return a + b + c + d + e + f + g + h; }
print f8(1, 2, 3, 4, 5, 6, 7, 8); // expect: 36

"#;

#[test]
fn test_files_function_parameters() {
    let expected_output = vec!["0".to_string(),"1".to_string(),"3".to_string(),"6".to_string(),"10".to_string(),"15".to_string(),"21".to_string(),"28".to_string(),"36".to_string()];
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert_eq!(result, InterpretResult::Ok(expected_output));
}
