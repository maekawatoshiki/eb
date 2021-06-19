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
    Punct(PunctKind),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DelimKind {
    Paren,
    Bracket,
    Brace,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PunctKind {
    Plus,
    Minus,
    Star,
    Slash,
    Eq,
    Colon,
    DoubleSemicolon,
    Comma,
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

    pub fn kind(&self) -> &TokenKind {
        &self.kind
    }

    pub fn loc(&self) -> &Location {
        &self.loc
    }
}

impl<'a> TokenKind<'a> {
    pub fn as_ident(&self) -> Option<&'a str> {
        match self {
            Self::Ident(i) => Some(i),
            _ => None,
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            ":" => Some(Self::Punct(PunctKind::Colon)),
            ";;" => Some(Self::Punct(PunctKind::DoubleSemicolon)),
            "," => Some(Self::Punct(PunctKind::Comma)),
            "(" => Some(Self::OpenDelim(DelimKind::Paren)),
            ")" => Some(Self::CloseDelim(DelimKind::Paren)),
            "{" => Some(Self::OpenDelim(DelimKind::Brace)),
            "}" => Some(Self::CloseDelim(DelimKind::Brace)),
            "[" => Some(Self::OpenDelim(DelimKind::Bracket)),
            "]" => Some(Self::CloseDelim(DelimKind::Bracket)),
            "+" => Some(Self::Punct(PunctKind::Plus)),
            "-" => Some(Self::Punct(PunctKind::Minus)),
            "*" => Some(Self::Punct(PunctKind::Star)),
            "/" => Some(Self::Punct(PunctKind::Slash)),
            "==" => Some(Self::Punct(PunctKind::Eq)),
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
        let bgn = self.body.as_str().as_ptr() as usize;
        let loc = |source: &str| -> Location { Location((source.as_ptr() as usize - bgn) as u32) };
        if let Some((source, token)) = preceded(
            spaces,
            alt((
                map(digit1, |i: &str| Token::new(TokenKind::Int(i), loc(i))),
                map(delimiter, |s: &str| {
                    Token::new(TokenKind::from_str(s).unwrap(), loc(s))
                }),
                map(symbol, |s: &str| {
                    Token::new(TokenKind::from_str(s).unwrap(), loc(s))
                }),
                map(identifier, |i: &str| {
                    Token::new(TokenKind::Ident(i), loc(i))
                }),
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
    alt((
        tag(":"),
        tag(";;"),
        tag(","),
        tag("+"),
        tag("-"),
        tag("*"),
        tag("/"),
        tag("=="),
    ))(source)
}

pub fn delimiter(source: &str) -> IResult<&str, &str, VerboseError<&str>> {
    alt((tag("("), tag(")"), tag("["), tag("]"), tag("{"), tag("}")))(source)
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
