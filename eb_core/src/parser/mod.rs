pub mod expr;
pub mod function;

use crate::lexer::{
    location::Location,
    token::{DelimKind, PunctKind, Token, TokenKind, TokenStream},
};
use anyhow::Result;
use std::{error::Error as StdErr, fmt, iter::Peekable};

#[derive(Debug)]
pub enum Error {
    ExpectedKeyword(Location, &'static str),
    ExpectedAnyIdent(Location),
    ExpectedOpenDelim(Location, DelimKind),
    ExpectedCloseDelim(Location, DelimKind),
    ExpectedPunct(Location, PunctKind),
    EOF,
}

pub struct Context<'a> {
    tokens: Peekable<TokenStream<'a>>,
}

impl<'a> Context<'a> {
    pub fn new(tokens: TokenStream<'a>) -> Self {
        Self {
            tokens: tokens.peekable(),
        }
    }

    pub fn peek(&mut self) -> Option<&Token> {
        self.tokens.peek()
    }

    pub fn next(&mut self) -> Option<Token> {
        self.tokens.next()
    }

    pub fn cur_loc(&mut self) -> Result<Location> {
        self.peek().map_or(Err(Error::EOF.into()), |t| Ok(*t.loc()))
    }

    pub fn expect_keyword(&mut self, kwd: &'static str) -> Result<Token> {
        if let Some(tok) = self.peek() {
            return match tok.kind() {
                TokenKind::Ident(i) if i == &kwd => Ok(self.next().unwrap()),
                _ => Err(Error::ExpectedKeyword(*tok.loc(), kwd).into()),
            };
        }
        Err(Error::EOF.into())
    }

    pub fn expect_any_ident(&mut self) -> Result<Token> {
        if let Some(tok) = self.peek() {
            return match tok.kind() {
                TokenKind::Ident(_) => Ok(self.next().unwrap()),
                _ => Err(Error::ExpectedAnyIdent(*tok.loc()).into()),
            };
        }
        Err(Error::EOF.into())
    }

    pub fn expect_open_delim(&mut self, delim: DelimKind) -> Result<Token> {
        if let Some(tok) = self.peek() {
            return match tok.kind() {
                TokenKind::OpenDelim(d) if d == &delim => Ok(self.next().unwrap()),
                _ => Err(Error::ExpectedOpenDelim(*tok.loc(), delim).into()),
            };
        }
        Err(Error::EOF.into())
    }

    pub fn expect_close_delim(&mut self, delim: DelimKind) -> Result<Token> {
        match self.peek() {
            Some(tok) => match tok.kind() {
                TokenKind::CloseDelim(d) if d == &delim => Ok(self.next().unwrap()),
                _ => Err(Error::ExpectedCloseDelim(*tok.loc(), delim).into()),
            },
            None => Err(Error::EOF.into()),
        }
    }

    pub fn expect_punct(&mut self, punct: PunctKind) -> Result<Token> {
        match self.peek() {
            Some(tok) => match tok.kind() {
                TokenKind::Punct(p) if p == &punct => Ok(self.next().unwrap()),
                _ => Err(Error::ExpectedPunct(*tok.loc(), punct).into()),
            },
            None => Err(Error::EOF.into()),
        }
    }

    pub fn skip_close_delim(&mut self, delim: DelimKind) -> bool {
        self.expect_close_delim(delim).is_ok()
    }

    pub fn skip_punct(&mut self, punct: PunctKind) -> bool {
        self.expect_punct(punct).is_ok()
    }
}

impl StdErr for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
