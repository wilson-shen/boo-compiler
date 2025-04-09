pub mod tokenizer;
pub use tokenizer::{tokenize, Token};

#[derive(Debug)]
pub struct Ast {
    pub tag: String,
    pub attributes: Vec<(String, String)>,
    pub children: Vec<AstNode>,
}

#[derive(Debug)]
pub enum AstNode {
    Element(Ast),
    Text(String),
}

pub fn parse(source: &str) -> Ast {
    let tokens = tokenize(source);
    let mut iter = tokens.into_iter().peekable();
    parse_element(&mut iter)
}

fn parse_element(tokens: &mut std::iter::Peekable<impl Iterator<Item = Token>>) -> Ast {
    let (tag, attributes, self_closing) = match tokens.next() {
        Some(Token::OpenTag {
            name,
            attributes,
            self_closing,
        }) => (name, attributes, self_closing),
        other => panic!("Expected opening tag, got: {:?}", other),
    };

    if self_closing {
        return Ast {
            tag,
            attributes,
            children: vec![],
        };
    }

    let mut children = vec![];

    while let Some(token) = tokens.peek() {
        match token {
            Token::Text(txt) => {
                children.push(AstNode::Text(txt.clone()));
                tokens.next();
            }
            Token::OpenTag { .. } => {
                let child = parse_element(tokens);
                children.push(AstNode::Element(child));
            }
            Token::CloseTag(name) if name == &tag => {
                tokens.next(); // consume the closing tag
                break;
            }
            unexpected => panic!("Unexpected token: {:?}", unexpected),
        }
    }

    Ast {
        tag,
        attributes,
        children,
    }
}
