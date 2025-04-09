use compiler::{codegen::GenerateTarget, compile, CompileOptions};

#[test]
fn test_codegen_output() {
    let source = std::fs::read_to_string("tests/examples/test_codegen.boo").unwrap();
    let options = CompileOptions {
        filename: Some("test_codegen.boo".into()),
        generate: GenerateTarget::Both,
        dev: true,
    };
    let result = compile(&source, options);
    println!("{}", result.code);
}
