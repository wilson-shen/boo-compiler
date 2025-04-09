use compiler::parser::{parse, AstNode};
use std::fs;

#[test]
fn test_parser_from_example_file() {
    let source = fs::read_to_string("tests/examples/Test.boo")
        .expect("Failed to read Test.boo");

    let ast = parse(&source);

    println!("{:#?}", ast);

    assert_eq!(ast.tag, "App");
    assert_eq!(ast.attributes.len(), 0);
    assert_eq!(ast.children.len(), 2);

    match &ast.children[0] {
        AstNode::Element(button) => {
            assert_eq!(button.tag, "Button");
            assert_eq!(button.attributes, vec![("color".into(), "red".into())]);
            assert_eq!(button.children.len(), 1);

            match &button.children[0] {
                AstNode::Text(text) => assert_eq!(text, "Click"),
                _ => panic!("Expected text inside Button"),
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
}

