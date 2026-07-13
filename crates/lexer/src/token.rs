#[derive(Debug,Clone,PartialEq)]

pub enum TokenType {
    //idents
    Identifier,

    //literal
    IntegerLiteral(i32),    // 4bytes Integer
    FloatLiteral(f64),      // 8bytes Float
    BooleanLiteral(bool),   // boolean
    CharLiteral(char),      // character

    //data type
    Integer,    // Int
    Boolean,    // Bool
    Character,  // Char
    Float,      // Float

    //keywords
    Variable,   // var
    Constant,   // const
    Function,   // function
    Return,     // return

    //assignment
    Assignment, // =

    //syntax
    Colon,      // :
    Semicolon,  // ;
    LParen,     // (
    RParen,     // )
    LBrace,     // {
    RBrace,     // }
    LBracket,   // [
    RBracket,   // ]
    Comma,      // ,

    //arithmetic operators
    Plus,       // +
    Minus,      // -
    Multiply,   // *
    Divide,     // /
    Modulo,     // %
    Exponent,   // ^

    //comparison operator
    Equality,           // ==
    GreaterThan,        // >
    GreaterThanEqual,   // >=
    LessThan,           // <
    LessThanEqual,      // <=
    NotEqual,           // !=

    //logical operator
    Not,    // not
    And,    // and
    Or,     // or
    Exor,   // exor
}

pub struct Token {
    pub kind: TokenType,
    pub line: usize,
}