pub mod codegen;
pub mod parser;

use codegen::{generate, GenerateTarget};
use parser::{parse, preprocess_file, PreprocessedSource};

pub struct CompileOptions {
    pub filename: Option<String>,
    pub generate: GenerateTarget,
    pub dev: bool,
}

pub struct CompileResult {
    pub code: String,
    pub warnings: Vec<String>,
}

pub fn compile(source: &str, options: CompileOptions) -> CompileResult {
    let PreprocessedSource {
        script: _,
        style: _,
        markup,
    } = preprocess_file(source);

    let ast = parse(&markup);

    let code = generate(&ast, &options); // later: also pass script/style

    CompileResult {
        code,
        warnings: vec![], // TODO: collect real warnings
    }
}
