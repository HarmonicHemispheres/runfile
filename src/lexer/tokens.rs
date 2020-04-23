#[derive(Debug, Clone)]
pub enum TokenType {
    Ident,
    Command,
    Script,
    Variable,
    OpenScope,
    CloseScope,
    OpenAttr,
    CloseAttr,
    StringValue,
    Colon,
    Value,
    EOF
}


#[derive(Debug, Clone)]
pub struct Token {
    pub t: TokenType,
    pub v: Option<String>
}