use crate::parser::{tokenize, Token};

#[derive(Debug, PartialEq)]
pub struct Ast {
    pub tag: String,
    pub attributes: Vec<(String, String)>,
    pub children: Vec<AstNode>,
}

#[derive(Debug, PartialEq)]
pub enum AstNode {
    Element(Ast),
    TextFragment(Vec<TextChunk>),
}

#[derive(Debug, PartialEq)]
pub enum TextChunk {
    Literal(String),
    Expr(String),
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
                let chunks = parse_text_chunks(txt);
                children.push(AstNode::TextFragment(chunks));
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

fn parse_text_chunks(text: &str) -> Vec<TextChunk> {
    let mut chunks = Vec::new();
    let mut remaining = text;

    while let Some(start) = remaining.find('{') {
        if let Some(end) = remaining[start..].find('}') {
            let end = start + end;

            // Literal before {
            if start > 0 {
                chunks.push(TextChunk::Literal(remaining[..start].to_string()));
            }

            // Expression inside {}
            let expr = &remaining[start + 1..end].trim();
            chunks.push(TextChunk::Expr(expr.to_string()));

            // Continue after }
            remaining = &remaining[end + 1..];
        } else {
            break; // No closing }
        }
    }

    if !remaining.is_empty() {
        chunks.push(TextChunk::Literal(remaining.to_string()));
    }

    chunks
}
