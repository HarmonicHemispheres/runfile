#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    Ident,
    EqualSign,
    Command,
    Script,
    Variable,
    OpenScope,
    CloseScope,
    OpenMLineScope,
    CloseMLineScope,
    OpenAttr,
    CloseAttr,
    StringValue,
    NewLine,
    Colon,
    Comma,
    Value,
    VarRef,
    EOF
}


#[derive(Debug, Clone)]
pub struct Token {
    pub t: TokenType,
    pub v: Option<String>
}