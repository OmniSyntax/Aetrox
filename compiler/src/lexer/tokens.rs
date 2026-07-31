use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\n\r\f]+")]// Automatically skips spaces, tabs, and newlines
pub enum Token {
    // ==========================================
    // 1. KEYWORDS (Reserved Language Words)
    // ==========================================
    #[token("fn")]
    Fn,

    #[token("let")]
    Let,

    #[token("mut")]
    Mut,

    #[token("if")]
    If,

    #[token("else")]
    Else,

    #[token("while")]
    While,

    #[token("return")]
    Return,

    // ==========================================
    // 2. IDENTIFIERS & LITERALS (Dynamic Values)
    // ==========================================
    // Matches variable and function names (e.g., `count`, `main`)
    #[regex("[a-zA-Z_$][a-zA-Z0-9_$]*", |lex| lex.slice().to_string())]
    Identifier(String),

    // Matches whole integer numbers (e.g., `0`, `42`, `100`)
    #[regex("[0-9]+", |lex| lex.slice().parse::<i64>().unwrap_or(0))]
    IntLiteral(i64),

    // Matches text strings inside double quotes (e.g., "Hello World")
    #[regex(r#""([^"\\]|\\.)*""#, |lex| {
        let slice = lex.slice();
        slice[1..slice.len() - 1].to_string()
    })]
    StringLiteral(String),

    // ==========================================
    // 3. OPERATORS (Math & Logic)
    // ==========================================
    #[token("=")]
    Assign,

    #[token("==")]
    Equals,

    #[token("!=")]
    NotEquals,

    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Star,

    #[token("/")]
    Slash,

    #[token("<")]
    LessThan,

    #[token(">")]
    GreaterThan,

    // ==========================================
    // 4. PUNCTUATION & DELIMITERS (Structure)
    // ==========================================
    #[token("(")]
    LeftParen,

    #[token(")")]
    RightParen,

    #[token("{")]
    LeftBrace,

    #[token("}")]
    RightBrace,

    #[token(",")]
    Comma,

    #[token(";")]
    Semicolon,
}