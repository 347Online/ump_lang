#[derive(Debug, Clone, Copy)]
pub enum TokenType {
    Semicolon,
    Equal,

    Let,
    Print,

    Number,
    String,
    Identifier,

    Error,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
