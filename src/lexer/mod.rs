pub mod tokens;
pub mod source;

pub use tokens::Token;
pub use source::SourceFile;
use logos::Logos;

pub struct Lexer<'a> {
    inner: logos::Lexer<'a, Token>,
}

impl<'a> Lexer<'a> {
    /// Creates a new Lexer instance from a SourceFile reference
    pub fn new(source_file: &'a SourceFile) -> Self {
        Self {
            inner: Token::lexer(&source_file.content),
        }
    }
}

/// Implements Rust's Iterator trait to stream tokens one by one into your parser
impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token, ()>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|res| res.map_err(|_| ()))
    }
}