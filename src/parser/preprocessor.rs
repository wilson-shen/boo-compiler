use regex::Regex;

#[derive(Debug)]
pub struct PreprocessedSource {
    pub script: String,
    pub style: String,
    pub markup: String,
}

pub fn preprocess_file(source: &str) -> PreprocessedSource {
    let script_re = Regex::new(r"(?s)<script>(.*?)</script>").unwrap();
    let style_re = Regex::new(r"(?s)<style>(.*?)</style>").unwrap();

    let script = script_re
        .captures(source)
        .and_then(|cap| cap.get(1).map(|m| m.as_str().trim().to_string()))
        .unwrap_or_default();

    let style = style_re
        .captures(source)
        .and_then(|cap| cap.get(1).map(|m| m.as_str().trim().to_string()))
        .unwrap_or_default();

    let markup = script_re
        .replace(source, "")
        .to_string()
        .pipe(|s| style_re.replace(&s, "").to_string())
        .trim()
        .to_string();

    PreprocessedSource {
        script,
        style,
        markup,
    }
}

trait Pipe: Sized {
    fn pipe<F: FnOnce(Self) -> R, R>(self, f: F) -> R { f(self) }
}
impl<T> Pipe for T {}

