use rlox::interpreter::run;
use rlox::result::InterpretResult;

const SOURCE: &str = r#"
// [line 3] Error: Unexpected character.
// [java line 3] Error at 'b': Expect ')' after arguments.
foo(a | b);

"#;

#[test]
fn test_files_unexpected_character() {
    
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    println!("{:?}", result);
    assert!(matches!(result, InterpretResult::CompileError{..}));
}
