use super::{location::Location, source::Source};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while1},
    character::complete::{char, digit1, multispace0},
    combinator::map,
    error::VerboseError,
    multi::many1,
    sequence::{preceded, terminated, tuple},
    IResult,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Token<'a> {
    kind: TokenKind<'a>,
    loc: Location,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind<'a> {
    Int(&'a str),
    Ident(&'a str),
    OpenDelim(DelimKind),
    CloseDelim(DelimKind),
    Colon,
    DoubleSemicolon,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DelimKind {
    Paren,
    Bracket,
    Brace,
}

pub struct TokenStream<'a> {
    source: &'a Source,
    tokens: Vec<Token<'a>>,
    body: &'a String,
    cur: &'a str,
}

impl<'a> Token<'a> {
    pub fn new(kind: TokenKind<'a>, loc: Location) -> Self {
        Self { kind, loc }
    }
}

impl<'a> TokenKind<'a> {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            ":" => Some(Self::Colon),
            ";;" => Some(Self::DoubleSemicolon),
            _ => None,
        }
    }
}

impl DelimKind {
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '(' | ')' => Some(DelimKind::Paren),
            '{' | '}' => Some(DelimKind::Brace),
            '[' | ']' => Some(DelimKind::Bracket),
            _ => None,
        }
    }
}

impl<'a> TokenStream<'a> {
    pub fn new(source: &'a Source) -> Self {
        let body = source.body();
        Self {
            source,
            tokens: vec![],
            body,
            cur: body.as_str(),
        }
    }

    pub fn source(&self) -> &Source {
        self.source
    }
}

impl<'a> Iterator for TokenStream<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let loc =
            Location((self.cur.as_ptr() as usize - self.body.as_str().as_ptr() as usize) as u32);
        if let Some((source, token)) = preceded(
            spaces,
            alt((
                map(digit1, |i: &str| Token::new(TokenKind::Int(i), loc)),
                map(delimiter, |c: char| {
                    Token::new(TokenKind::OpenDelim(DelimKind::from_char(c).unwrap()), loc)
                }),
                map(symbol, |s: &str| {
                    Token::new(TokenKind::from_str(s).unwrap(), loc)
                }),
                map(identifier, |i: &str| Token::new(TokenKind::Ident(i), loc)),
            )),
        )(self.cur)
        .ok()
        {
            self.cur = source;
            self.tokens.push(token.clone());
            return Some(token);
        }
        None
    }
}

pub fn symbol(source: &str) -> IResult<&str, &str, VerboseError<&str>> {
    alt((tag(":"), tag(";;")))(source)
}

pub fn delimiter(source: &str) -> IResult<&str, char, VerboseError<&str>> {
    alt((
        char('('),
        char(')'),
        char('['),
        char(']'),
        char('{'),
        char('}'),
    ))(source)
}

pub fn identifier(source: &str) -> IResult<&str, &str, VerboseError<&str>> {
    take_while1(|c: char| c.is_alphanumeric() || c == '_')(source)
}

pub fn spaces(source: &str) -> IResult<&str, (), VerboseError<&str>> {
    alt((
        map(
            many1(tuple((
                multispace0,
                preceded(tag("//"), terminated(take_until("\n"), char('\n'))),
                multispace0,
            ))),
            |_| (),
        ),
        map(multispace0, |_| ()),
    ))(source)
}
