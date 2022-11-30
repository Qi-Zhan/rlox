#![allow(dead_code)]

use std::str::Chars;

use crate::value::Value;

pub fn tokenize(input: &str) -> impl Iterator<Item = Token> + '_ {
    let mut cursor = Cursor::new(input);
    std::iter::from_fn(move || {
        let token = cursor.advance_token();
        if token == Token::Eof {
            None
        } else {
            Some(token)
        }
    })
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // Literals.
    Identifier(String),
    String(String),
    Number(Value),
    /* Keywords. */
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Error(String),
    Eof,
}

struct Cursor<'a> {
    chars: Chars<'a>,
    line: usize,
    column: usize,
}

impl<'a> Cursor<'a> {
    fn new (input: &'a str) -> Self {
        Self {
            chars: input.chars(),
            line: 1,
            column: 1,
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if !c.is_whitespace() && c != '\n'  {
                break;
            }
            self.consume();
        }
    }

    fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    fn peek(&self) -> Option<char> {
        self.chars.clone().next()
    }

    fn identifier_or_keyword(&mut self) -> Token {
        todo!("identifier_or_keyword")
    }

    fn consume(&mut self) -> Option<char> {
        let c = self.chars.next();
        if c == Some('\n') {
            self.line += 1;   
            self.column = 1; 
        }
        self.column += 1;
        c
    }
    
    fn number(&mut self, first: char) -> Token {
        let mut string = first.to_string(); 
        while !self.is_eof() && self.peek().unwrap().is_digit(10) {
            string.push(self.consume().unwrap());
        }
        if !self.is_eof() && self.peek().unwrap() == '.' {
            string.push('.');
            self.consume();
        }
        while !self.is_eof() && self.peek().unwrap().is_digit(10) {
            string.push(self.consume().unwrap());
        }
        Token::Number(string.parse().unwrap())
    }

    fn string(&mut self) -> Token {
        let mut string = String::new();
        while !self.is_eof() && self.peek().unwrap() != '"' {
            string.push(self.consume().unwrap());
        }
        if self.is_eof() {
            return Token::Error("Unterminated string.".to_string());
        }
        self.consume();
        Token::String(string)
    }

    /// Pareses a token from the input string.
    fn advance_token(&mut self) -> Token {
        let first_char = match self.consume() {
            Some(c) => c,
            None => return Token::Eof,
        };

        match first_char {
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            '{' => Token::LeftBrace,
            '}' => Token::RightBrace,
            ',' => Token::Comma,
            '.' => Token::Dot,
            '-' => Token::Minus,
            '+' => Token::Plus,
            ';' => Token::Semicolon,
            '*' => Token::Star,
            '!' => {
                if self.peek() == Some('=') {
                    self.consume();
                    Token::BangEqual
                } else {
                    Token::Bang
                }
            }
            '=' => {
                if self.peek() == Some('=') {
                    self.consume();
                    Token::EqualEqual
                } else {
                    Token::Equal
                }
            }
            '<' => {
                if self.peek() == Some('=') {
                    self.consume();
                    Token::LessEqual
                } else {
                    Token::Less
                }
            }
            '>' => {
                if self.peek() == Some('=') {
                    self.consume();
                    Token::GreaterEqual
                } else {
                    Token::Greater
                }
            }
            '/' => {
                if self.peek() == Some('/') {
                    // A comment goes until the end of the line.
                    while self.peek() != Some('\n') && !self.is_eof() {
                        self.consume();
                    }
                    return self.advance_token();
                }
                Token::Slash
            }
            ' ' | '\r' | '\t'  => {
                self.skip_whitespace();
                return self.advance_token();
            }
            '\n' => {
                self.consume();
                return self.advance_token();
            }
            '"' =>  self.string(),
            
            c if c.is_digit(10) => {
                self.number(c)
            }

            _ => Token::Error(format!("line {} column {}: Unexpected character {}", self.line, self.column, first_char))
        }




    }

    fn is_alpha(c: char) -> bool {
        c.is_alphabetic() || c == '_'
    }
    
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number() {
        assert_eq!(Token::Number(123.00000123), tokenize(" 123.00000123 ").next().unwrap());
        assert_eq!(Token::Number(123.), tokenize(" 123. ").next().unwrap());
        assert_eq!(Token::Number(123.0), tokenize(" 123 ").next().unwrap());
    }

    #[test]
    fn test_strings() {
        assert_eq!(Token::String("abcd888_".to_string()), tokenize("\"abcd888_\"").next().unwrap());
        assert_eq!(Token::String("".to_string()), tokenize("\"\"").next().unwrap());
    }

    #[test]
    fn test_identifier() {
        let input = "andy formless fo _ _123 _abc ab123";
        let mut tokens = tokenize(input);
        assert_eq!(Token::Identifier("andy".to_string()), tokens.next().unwrap());
        assert_eq!(Token::Identifier("formless".to_string()), tokens.next().unwrap());
        assert_eq!(Token::Identifier("fo".to_string()), tokens.next().unwrap());
        assert_eq!(Token::Identifier("_".to_string()), tokens.next().unwrap());
        assert_eq!(Token::Identifier("_123".to_string()), tokens.next().unwrap());
        assert_eq!(Token::Identifier("_abc".to_string()), tokens.next().unwrap());
        assert_eq!(Token::Identifier("ab123".to_string()), tokens.next().unwrap());

    }

    #[test]
    fn test_keywords() {
        assert_eq!(Token::And, tokenize("and").next().unwrap());
        assert_eq!(Token::Class, tokenize("class").next().unwrap());
        assert_eq!(Token::Else, tokenize("else").next().unwrap());
        assert_eq!(Token::False, tokenize("false").next().unwrap());
        assert_eq!(Token::For, tokenize("for").next().unwrap());
        assert_eq!(Token::Fun, tokenize("fun").next().unwrap());
        assert_eq!(Token::If, tokenize("if").next().unwrap());
        assert_eq!(Token::Nil, tokenize("nil").next().unwrap());
        assert_eq!(Token::Or, tokenize("or").next().unwrap());
        assert_eq!(Token::Print, tokenize("print").next().unwrap());
        assert_eq!(Token::Return, tokenize("return").next().unwrap());
        assert_eq!(Token::Super, tokenize("super").next().unwrap());
        assert_eq!(Token::This, tokenize("this").next().unwrap());
        assert_eq!(Token::True, tokenize("true").next().unwrap());
        assert_eq!(Token::Var, tokenize("var").next().unwrap());
        assert_eq!(Token::While, tokenize("while").next().unwrap());
    }

    #[test]
    fn test_punctuation() {
        assert_eq!(Token::LeftParen, tokenize("(").next().unwrap());
        assert_eq!(Token::RightParen, tokenize(")").next().unwrap());
        assert_eq!(Token::LeftBrace, tokenize("{").next().unwrap());
        assert_eq!(Token::RightBrace, tokenize("}").next().unwrap());
        assert_eq!(Token::Comma, tokenize(",").next().unwrap());
        assert_eq!(Token::Dot, tokenize(".").next().unwrap());
        assert_eq!(Token::Minus, tokenize("-").next().unwrap());
        assert_eq!(Token::Plus, tokenize("+").next().unwrap());
        assert_eq!(Token::Semicolon, tokenize(";").next().unwrap());
        assert_eq!(Token::Slash, tokenize("/").next().unwrap());
        assert_eq!(Token::Star, tokenize("*").next().unwrap());
    }

    #[test]
    fn test_operators() {
        assert_eq!(Token::Bang, tokenize("!").next().unwrap());
        assert_eq!(Token::BangEqual, tokenize("!=").next().unwrap());
        assert_eq!(Token::Equal, tokenize("=").next().unwrap());
        assert_eq!(Token::EqualEqual, tokenize("==").next().unwrap());
        assert_eq!(Token::Greater, tokenize(">").next().unwrap());
        assert_eq!(Token::GreaterEqual, tokenize(">=").next().unwrap());
        assert_eq!(Token::Less, tokenize("<").next().unwrap());
        assert_eq!(Token::LessEqual, tokenize("<=").next().unwrap());
    }

    #[test]
    fn test_whitespace() {
        
    }


}