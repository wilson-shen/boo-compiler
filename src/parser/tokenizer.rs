use regex::Regex;

#[derive(Debug, PartialEq)]
pub enum Token {
    OpenTag {
        name: String,
        attributes: Vec<(String, String)>,
        self_closing: bool,
    },
    CloseTag(String),
    Text(String),
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut pos = 0;
    let chars: Vec<char> = input.chars().collect();

    while pos < chars.len() {
        if chars[pos] == '<' {
            if pos + 1 < chars.len() && chars[pos + 1] == '/' {
                // Closing tag
                pos += 2;
                let start = pos;
                while pos < chars.len() && chars[pos] != '>' {
                    pos += 1;
                }
                let name: String = chars[start..pos].iter().collect();
                pos += 1; // skip '>'
                tokens.push(Token::CloseTag(name.trim().to_string()));
            } else {
                // Opening tag (may be self-closing and have attributes)
                pos += 1;
                let start = pos;
                while pos < chars.len() && chars[pos] != '>' {
                    pos += 1;
                }
                let contents: String = chars[start..pos].iter().collect();
                pos += 1; // skip '>'

                let (name, attributes, self_closing) = parse_tag_contents(&contents);
                tokens.push(Token::OpenTag {
                    name,
                    attributes,
                    self_closing,
                });
            }
        } else {
            // Text content
            let start = pos;
            while pos < chars.len() && chars[pos] != '<' {
                pos += 1;
            }
            let text: String = chars[start..pos].iter().collect();
            if !text.trim().is_empty() {
                tokens.push(Token::Text(text.trim().to_string()));
            }
        }
    }

    tokens
}

fn parse_tag_contents(contents: &str) -> (String, Vec<(String, String)>, bool) {
    let trimmed = contents.trim();
    let self_closing = trimmed.ends_with("/");

    let tag_and_attrs = if self_closing {
        &trimmed[..trimmed.len() - 1].trim()
    } else {
        trimmed
    };

    let mut parts = tag_and_attrs.splitn(2, char::is_whitespace);
    let tag = parts.next().unwrap().to_string();
    let attr_str = parts.next().unwrap_or("");

    let mut attributes = vec![];

    let attr_re = Regex::new(r#"([a-zA-Z_][\w-]*)\s*=\s*"([^"]*)""#).unwrap();
    for cap in attr_re.captures_iter(attr_str) {
        attributes.push((cap[1].to_string(), cap[2].to_string()));
    }

    (tag, attributes, self_closing)
}
