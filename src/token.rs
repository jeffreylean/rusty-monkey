#![allow(unused)]
use std::collections::HashMap;

use lazy_static::lazy_static;

lazy_static! {
    static ref KEYWORD_MAP: HashMap<&'static str, TokenType> = {
        let mut map = HashMap::new();
        map.insert("fn", TokenType::Function);
        map.insert("let", TokenType::Let);
        map.insert("true", TokenType::True);
        map.insert("false", TokenType::False);
        map.insert("if", TokenType::If);
        map.insert("else", TokenType::Else);
        map.insert("return", TokenType::Return);

        map
    };
}

pub fn lookup_ident(ident: &str) -> TokenType {
    if KEYWORD_MAP.contains_key(ident) {
        return KEYWORD_MAP[ident].clone();
    }
    TokenType::Ident
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Illegal,
    Eof,
    Ident,
    Int,
    Assign,
    // Operators
    Plus,
    Minus,
    Bang,
    Asterisk,
    Comma,
    Semicolon,

    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Lt,
    Gt,
    Slash,
    // Keywords
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,

    Eq,
    NotEq,

    Default,
}

impl TokenType {
    fn as_str(&self) -> &str {
        match self {
            TokenType::Illegal => "ILLEGAL",
            TokenType::Eof => "EOF",
            TokenType::Ident => "IDENT",
            TokenType::Int => "INT",
            TokenType::Assign => "=",
            TokenType::Plus => "+",
            TokenType::Minus => "-",
            TokenType::Comma => ",",
            TokenType::Semicolon => ";",
            TokenType::Lparen => "(",
            TokenType::Rparen => ")",
            TokenType::Lbrace => "{",
            TokenType::Rbrace => "}",
            TokenType::Lt => "<",
            TokenType::Gt => ">",
            TokenType::Bang => "!",
            TokenType::Slash => "/",
            TokenType::Asterisk => "*",
            TokenType::Function => "FUNCTION",
            TokenType::Let => "LET",
            TokenType::True => "TRUE",
            TokenType::False => "FALSE",
            TokenType::If => "IF",
            TokenType::Else => "ELSE",
            TokenType::Return => "RETURN",
            TokenType::Eq => "==",
            TokenType::NotEq => "!=",
            TokenType::Default => "default",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub t_type: TokenType,
    pub literal: String,
}
