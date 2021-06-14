pub mod location;
pub mod source;
pub mod token;

use source::Source;
use token::Token;

pub fn tokens<'a>(source: &'a Source) -> impl Iterator<Item = Token> + 'a {
    token::TokenStream::new(source)
}

#[test]
fn parse1() {
    use location::Location;

    let source = Source::String("func f(x i32) i32: x;;".to_string());
    let tokens: Vec<Token> = tokens(&source).into_iter().collect();
    let correct = vec![
        Token::new(token::TokenKind::Ident("func"), Location(0)),
        Token::new(token::TokenKind::Ident("f"), Location(4)),
        Token::new(
            token::TokenKind::OpenDelim(token::DelimKind::Paren),
            Location(6),
        ),
        Token::new(token::TokenKind::Ident("x"), Location(7)),
        Token::new(token::TokenKind::Ident("i32"), Location(8)),
        Token::new(
            token::TokenKind::OpenDelim(token::DelimKind::Paren),
            Location(12),
        ),
        Token::new(token::TokenKind::Ident("i32"), Location(13)),
        Token::new(token::TokenKind::Colon, Location(17)),
        Token::new(token::TokenKind::Ident("x"), Location(18)),
        Token::new(token::TokenKind::DoubleSemicolon, Location(20)),
    ];
    assert_eq!(tokens.len(), correct.len());
    assert!(tokens.iter().zip(correct.iter()).all(|(a, b)| a == b))
}
