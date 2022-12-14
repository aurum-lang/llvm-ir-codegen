#![allow(unused, dead_code)]

/// Representations of static primitive types
/// Do NOT contain values, instead represent expected values
/// Useful for signifying collection types in ASTs
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Primitive {
    Integer,
    Float,
    Boolean,
    String,
}

/// Literal types which contain values
/// (Hybrid AST type)
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Literal {
    Int(i32),
    Float(f32),
    Bool(bool),
    String(String),
}

/// Keyword token to be interpreted
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

/// Token symbols to be preceded by keywords
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

/// Monolithic token type
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Token {
    Keyword(Keyword),
    Symbol(Symbol),
    Literal(Literal),
    Array(Primitive),
    Unknown(String),
}
