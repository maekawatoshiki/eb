pub mod location;
pub mod source;
pub mod token;

use source::Source;

pub fn tokenize<'a>(source: &'a Source) -> token::TokenStream<'a> {
    token::TokenStream::new(source)
}

#[test]
fn tokenize1() {
    use location::Location;
    use token::Token;

    let source = Source::String(r#"func f(x i32) i32: x;;"#.to_string());
    let tokenize: Vec<Token> = tokenize(&source).into_iter().collect();
    let correct = vec![
        Token::new(token::TokenKind::Ident("func"), Location(0)),
        Token::new(token::TokenKind::Ident("f"), Location(5)),
        Token::new(
            token::TokenKind::OpenDelim(token::DelimKind::Paren),
            Location(6),
        ),
        Token::new(token::TokenKind::Ident("x"), Location(7)),
        Token::new(token::TokenKind::Ident("i32"), Location(9)),
        Token::new(
            token::TokenKind::CloseDelim(token::DelimKind::Paren),
            Location(12),
        ),
        Token::new(token::TokenKind::Ident("i32"), Location(14)),
        Token::new(
            token::TokenKind::Punct(token::PunctKind::Colon),
            Location(17),
        ),
        Token::new(token::TokenKind::Ident("x"), Location(19)),
        Token::new(
            token::TokenKind::Punct(token::PunctKind::DoubleSemicolon),
            Location(20),
        ),
    ];
    assert_eq!(tokenize.len(), correct.len());
    assert!(tokenize.iter().zip(correct.iter()).all(|(a, b)| a == b))
}

#[test]
fn tokenize2() {
    use location::Location;
    use token::{DelimKind, PunctKind, Token, TokenKind};

    let source = Source::String(
        r#"
        func f(x y i32) i32: 
            x + 1 == y - 1;;
        "#
        .to_string(),
    );
    let tokenize: Vec<Token> = tokenize(&source).into_iter().collect();
    let correct = vec![
        Token::new(TokenKind::Ident("func"), Location(9)),
        Token::new(TokenKind::Ident("f"), Location(14)),
        Token::new(TokenKind::OpenDelim(DelimKind::Paren), Location(15)),
        Token::new(TokenKind::Ident("x"), Location(16)),
        Token::new(TokenKind::Ident("y"), Location(18)),
        Token::new(TokenKind::Ident("i32"), Location(20)),
        Token::new(TokenKind::CloseDelim(DelimKind::Paren), Location(23)),
        Token::new(TokenKind::Ident("i32"), Location(25)),
        Token::new(TokenKind::Punct(token::PunctKind::Colon), Location(28)),
        Token::new(TokenKind::Ident("x"), Location(43)),
        Token::new(TokenKind::Punct(PunctKind::Plus), Location(45)),
        Token::new(TokenKind::Int("1"), Location(47)),
        Token::new(TokenKind::Punct(PunctKind::Eq), Location(49)),
        Token::new(TokenKind::Ident("y"), Location(52)),
        Token::new(TokenKind::Punct(PunctKind::Minus), Location(54)),
        Token::new(TokenKind::Int("1"), Location(56)),
        Token::new(
            TokenKind::Punct(token::PunctKind::DoubleSemicolon),
            Location(57),
        ),
    ];
    assert_eq!(tokenize.len(), correct.len());
    assert!(tokenize.iter().zip(correct.iter()).all(|(a, b)| a == b))
}
