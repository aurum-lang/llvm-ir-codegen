#![allow(unused, dead_code)]

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Primitive {
    Integer,
    Float,
    Boolean,
    String,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Literal {
    Int(i32),
    Float(f32),
    Bool(bool),
    String(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Token {
    Keyword(Keyword),
    Symbol(Symbol),
    Literal(Literal),
    Array(Primitive),
}
