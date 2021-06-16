pub mod function;

use crate::lexer::{
    location::Location,
    token::{Token, TokenKind, TokenStream},
};
use anyhow::Result;
use std::{error::Error as StdErr, fmt, iter::Peekable};

#[derive(Debug)]
pub enum Error {
    ExpectedKeyword(Location, &'static str),
    ExpectedAnyIdent(Location),
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

    pub fn skip_ident(&mut self, ident: &str) -> bool {
        if let Some(tok) = self.peek() {
            return matches!(tok.kind(), TokenKind::Ident(i) if i == &ident);
        }
        false
    }

    pub fn expect_keyword(&mut self, kwd: &'static str) -> Result<()> {
        if let Some(tok) = self.peek() {
            return match tok.kind() {
                TokenKind::Ident(i) if i == &kwd => Ok(()),
                _ => Err(Error::ExpectedKeyword(*tok.loc(), kwd).into()),
            };
        }
        Err(Error::EOF.into())
    }

    pub fn expect_any_ident(&mut self) -> Result<&Token> {
        if let Some(tok) = self.peek() {
            return match tok.kind() {
                TokenKind::Ident(_) => Ok(tok),
                _ => Err(Error::ExpectedAnyIdent(*tok.loc()).into()),
            };
        }
        Err(Error::EOF.into())
    }
}

impl StdErr for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[test]
fn parse1() {
    // use location::Location;
    use crate::lexer::{source::Source, tokenize};

    let source = Source::String(r#"func f(x i32) i32: x;;"#.to_string());
    let mut ctx = Context::new(tokenize(&source));
    function::parse(&mut ctx).expect("fail to parse");
}
