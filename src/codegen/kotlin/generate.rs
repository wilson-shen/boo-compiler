use crate::parser::{Ast, AstNode, TextChunk};

pub fn generate(ast: &Ast) -> String {
    match ast.tag.as_str() {
        "Button" => {
            let action = get_attribute("onclick", &ast.attributes)
                .map(|v| format!("onClick = {{ {} }}", v))
                .unwrap_or_default();
            let mut result = format!("Button({}) {{\n", action);
            for node in &ast.children {
                result += &render_node(node);
            }
            result += "}\n";
            result
        }
        "Text" => {
            let content = ast.children.iter().map(render_node).collect::<String>();
            format!("Text({})\n", content.trim())
        }
        "Spacer" => "Spacer()\n".to_string(),
        _ => {
            let mut result = format!("Compose({}) {{\n", ast.tag);
            for node in &ast.children {
                result += &render_node(node);
            }
            result += "}\n";
            result
        }
    }
}

fn render_node(node: &AstNode) -> String {
    match node {
        AstNode::Element(child) => generate(child),
        AstNode::TextFragment(chunks) => {
            let joined = chunks
                .iter()
                .map(format_chunk)
                .collect::<Vec<_>>()
                .join(" + ");
            format!("  Text({})\n", joined)
        }
    }
}

fn format_chunk(chunk: &TextChunk) -> String {
    match chunk {
        TextChunk::Literal(text) => format!("\"{}\"", text.replace('"', "\\\"")),
        TextChunk::Expr(expr) => expr.to_string(),
    }
}

fn get_attribute<'a>(name: &str, attrs: &'a [(String, String)]) -> Option<&'a String> {
    attrs.iter().find(|(k, _)| k == name).map(|(_, v)| v)
}
