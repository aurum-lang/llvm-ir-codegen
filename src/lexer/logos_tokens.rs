use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,

    #[token("let")]
    Let,

    #[token("do")]
    Do,

    #[token("end")]
    End,

    #[token("rep")]
    #[token("represent")]
    Represents,

    #[token("==")]
    #[token("=")]
    #[token("equals")]
    #[token("is")]
    EqualTo,

    #[regex(r#"/".*"/g"#)]
    StringLiteral,

    #[regex(r"[0-9]+")]
    IntLiteral,

    #[token("int")]
    IntType,
    #[token("string")]
    StringType,
    #[token("bool")]
    BoolType,

    #[token("[int]")]
    IntArrType,
    #[token("[string]")]
    StringArrType,
    #[token("[bool]")]
    BoolArrType,

    #[token(".")]
    Period,

    #[regex("[a-zA-Z]*")]
    OtherKeyword,

    #[error]
    Error,

    #[regex(r"[ \t\n\f]+", logos::skip)]
    Skipped,
}
