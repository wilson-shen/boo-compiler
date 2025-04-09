use crate::{CompileOptions, GenerateTarget};
use crate::parser::{Ast, AstNode};

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

fn generate_kotlin(ast: &Ast) -> String {
    let mut result = format!("Compose({}) {{\n", ast.tag);
    for node in &ast.children {
        match node {
            AstNode::Text(text) => result += &format!("  Text(\"{}\")\n", text),
            AstNode::Element(child) => result += &generate_kotlin(child),
        }
    }
    result += "}\n";
    result
}

fn generate_swift(ast: &Ast) -> String {
    let mut result = format!("{} {{\n", ast.tag);
    for node in &ast.children {
        match node {
            AstNode::Text(text) => result += &format!("  Text(\"{}\")\n", text),
            AstNode::Element(child) => result += &generate_swift(child),
        }
    }
    result += "}\n";
    result
}

