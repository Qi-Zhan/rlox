use rlox::interpreter::run;
use rlox::result::InterpretResult;

const SOURCE: &str = r#"
{
  class Foo < Foo {} // Error at 'Foo': A class can't inherit from itself.
}
// [c line 5] Error at end: Expect '}' after block.

"#;

#[test]
fn test_files_class_local_inherit_self() {
    
    let result: InterpretResult<Vec<String>>= run(SOURCE);
    
    assert!(matches!(result, InterpretResult::CompileError{..}));
}
