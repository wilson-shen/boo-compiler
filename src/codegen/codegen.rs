use crate::codegen::kotlin::generate as generate_kotlin;
use crate::codegen::swift::generate as generate_swift;
use crate::parser::Ast;
use crate::CompileOptions;

pub enum GenerateTarget {
    Swift,
    Kotlin,
    Both,
}

pub fn generate(ast: &Ast, options: &CompileOptions) -> String {
    match options.generate {
        GenerateTarget::Swift => generate_swift(ast),
        GenerateTarget::Kotlin => generate_kotlin(ast),
        GenerateTarget::Both => {
            let swift = generate_swift(ast);
            let kotlin = generate_kotlin(ast);
            format!("{}\n{}", swift, kotlin)
        }
    }
}
