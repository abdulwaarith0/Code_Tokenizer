use std::fmt::{Display, Formatter};
#[derive(PartialEq, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub literal: String,
}

#[derive(PartialEq, Debug)]
pub enum TokenKind {
    Illegal,
    Eof,
    Ident,
    Int,
    Assign,
    Plus,
    Comma,
    Semicolon,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Function,
    Let,
    If,
    Else,
    Return,
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::Illegal => write!(f, "Illegal"),
            TokenKind::Eof => write!(f, "EOF"),
            TokenKind::Ident => write!(f, "Ident"),
            TokenKind::Int => write!(f, "Int"),
            TokenKind::Assign => write!(f, "Assign"),
            TokenKind::Plus => write!(f, "Plus"),
            TokenKind::Comma => write!(f, "Comma"),
            TokenKind::Semicolon => write!(f, "Semicolon"),
            TokenKind::LeftParen => write!(f, "LeftParen"),
            TokenKind::RightParen => write!(f, "RightParen"),
            TokenKind::LeftBrace => write!(f, "LeftBrace"),
            TokenKind::RightBrace => write!(f, "RightBrace"),
            TokenKind::Function => write!(f, "Function"),
            TokenKind::Let => write!(f, "Let"),
            TokenKind::If => write!(f, "If"),
            TokenKind::Else => write!(f, "Else"),
            TokenKind::Return => write!(f, "Return"),
        }
    }
}
