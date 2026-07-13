use crate::token::{Token, TokenType};

pub struct Scanner {
    src: String,
    pos: usize,
}

impl Scanner {
    pub fn new(src: String) -> Self {
        Scanner {
            src,
            pos: 0,
        }
    }

    pub fn next_token(&self) -> Token {
        Token {
            kind: TokenType::Variable,
            line: 0
        }
    }
}