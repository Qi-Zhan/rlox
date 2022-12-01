use rlox::interpreter::run;
use rlox::error::InterpretResult;

const SOURCE: &str = r#"
// [line 2] Error at 'this': Expect variable name.
var this = "value";

"#;

#[test]
fn test_files_variable_use_this_as_var() {
    
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::LexError{..}));
}
