#[derive(Debug)]
pub enum Token {
    // primitive values
    Id(String),
    Int(u64),
    Float(f64),
    Bool(bool),

    // keywords
    Echo,
    Else,
    Fn,
    If,
    Let,
    Match,
    Read,
    Return,
    Then,
    Typedef,

    // type keywords
    IntType,
    FloatType,
    BoolType,
    TypeId(String),

    // punctuation
    LParen,
    RParen,
    Comma,
    Arrow,
    Colon,
    Semicolon,
    LBrace,
    RBrace,

    // operators
    Not,
    NotEq,
    And,
    Times,
    Plus,
    Minus,
    Divide,
    LessThan,
    LessThanOrEq,
    Eq,
    GreaterThan,
    GreaterThanOrEq,
    Pipe,
    Or,

    // miscellaneous
    Invalid(char),
    EoF,
}
