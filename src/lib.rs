pub mod parser;
pub mod codegen;

pub use parser::parse;
pub use codegen::generate;

pub struct CompileOptions {
    pub filename: Option<String>,
    pub generate: GenerateTarget, // Swift / Kotlin / Both
    pub dev: bool,
}

pub enum GenerateTarget {
    Swift,
    Kotlin,
    Both,
}

pub struct CompileResult {
    pub code: String,
    pub warnings: Vec<String>,
}

pub fn compile(source: &str, options: CompileOptions) -> CompileResult {
    let ast = parser::parse(source);

    // Later: transform(ast)

    let code = codegen::generate(&ast, &options);

    CompileResult {
        code,
        warnings: vec![], // later: collect warnings from parser/transform/codegen
    }
}

