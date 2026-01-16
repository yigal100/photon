pub struct Lexer;

impl Lexer {
    pub fn new() -> Self {
        Lexer
    }

    pub fn tokenize<'a>(&self, input: &'a str) -> Vec<&'a str> {
        input.split_whitespace().collect()
    }
}
