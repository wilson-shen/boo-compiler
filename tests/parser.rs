use compiler::parser::{parse, preprocess_file, AstNode, TextChunk};
use std::fs;

#[test]
fn test_parser_file() {
    let source = fs::read_to_string("tests/examples/test_parser.boo")
        .expect("Failed to read test_parser.boo");

    let ast = parse(&source);
    // println!("{:#?}", ast);

    assert_eq!(ast.tag, "App");
    assert_eq!(ast.attributes.len(), 0);
    assert_eq!(ast.children.len(), 3);

    match &ast.children[0] {
        AstNode::Element(button) => {
            assert_eq!(button.tag, "Button");
            assert_eq!(button.attributes, vec![("color".into(), "red".into())]);
            assert_eq!(button.children.len(), 1);

            match &button.children[0] {
                AstNode::TextFragment(chunks) => {
                    assert_eq!(chunks.len(), 1);
                    assert_eq!(chunks[0], TextChunk::Literal("Click".into()));
                }
                _ => panic!("Expected TextFragment inside Button"),
            }
        }
        _ => panic!("Expected Button element"),
    }

    match &ast.children[1] {
        AstNode::Element(spacer) => {
            assert_eq!(spacer.tag, "Spacer");
            assert_eq!(spacer.attributes.len(), 0);
            assert!(spacer.children.is_empty());
        }
        _ => panic!("Expected Spacer element"),
    }

    match &ast.children[2] {
        AstNode::Element(text) => {
            assert_eq!(text.tag, "Text");

            match &text.children[0] {
                AstNode::TextFragment(chunks) => {
                    assert_eq!(chunks.len(), 3);
                    assert_eq!(chunks[0], TextChunk::Literal("Hello ".into()));
                    assert_eq!(chunks[1], TextChunk::Expr("count".into()));
                    assert_eq!(chunks[2], TextChunk::Literal("!".into()));
                }
                _ => panic!("Expected TextFragment"),
            }
        }
        _ => panic!("Expected Text element"),
    }
}

#[test]
fn test_preprocess_file() {
    let source = fs::read_to_string("tests/examples/test_preprocess.boo")
        .expect("Failed to read test_preprocess.boo");

    let result = preprocess_file(&source);

    assert_eq!(result.script.trim(), "let count = 0;");
    assert!(result.markup.contains("<App>"));
    assert!(result.markup.contains("onclick"));
    assert!(result.style.contains("color: red"));
}
