use std::fmt::{self, Formatter, Display};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    Ident,
    Keyword,
    EqualSign,
    Comment,
    RunSym,
    OpenScope,
    CloseScope,
    OpenAttr,
    CloseAttr,
    OpenParen,
    CloseParen,
    StringValue,
    IntValue,
    NewLine,
    Period,
    Colon,
    SemiColon,
    Comma,
    Space,
    Value,
    DollarSign,
    EOF
}

// NOTE: 
// #[derive(Debug, Clone, PartialEq, Eq)]
// pub enum KeywordType {}



#[derive(Debug, Clone)]
pub struct Token {
    pub t: TokenType,
    pub v: Option<String>,
    pub col: usize,
    pub ln: usize
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
