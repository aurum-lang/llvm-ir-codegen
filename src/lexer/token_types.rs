pub enum Primitive {
    Integer,
    Float,
    Boolean,
    String,
}

pub enum Keyword {
    For,
    While,
    If,
    In,
    And,
    Or,
    Run,
    Int,
    Float,
    Bool,
    String,
}

pub enum Symbol {
    LBracket,
    RBracket,
    LParen,
    RParen,
    Equal,
    DoubleEqual,
    Add,
    Minus,
    Multiply,
    Divide,
    PlusEqual,
    MinusEqual,
    MultiplyEqual,
    DivideEqual,
}

pub enum Token {
    Keyword(Keyword),
    Symbol(Symbol),
    Literal(String),
    Array(Primitive),
}
