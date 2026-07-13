#[allow(dead_code)]
mod token;
mod scanner;
use scanner::Scanner;

#[cfg(test)]
mod tests {
    use crate::token::TokenType;
    use super::*;

    #[test]
    fn test_test(){
        let input: String = String::from("var a: Int = 50;");
        let lexer = Scanner::new(input);
        assert_eq!(lexer.next_token().kind, TokenType::Variable);
        assert_eq!(2, 2);
    }
}