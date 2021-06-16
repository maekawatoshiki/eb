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
        Token::new(token::TokenKind::Ident("f"), Location(4)),
        Token::new(
            token::TokenKind::OpenDelim(token::DelimKind::Paren),
            Location(6),
        ),
        Token::new(token::TokenKind::Ident("x"), Location(7)),
        Token::new(token::TokenKind::Ident("i32"), Location(8)),
        Token::new(
            token::TokenKind::CloseDelim(token::DelimKind::Paren),
            Location(12),
        ),
        Token::new(token::TokenKind::Ident("i32"), Location(13)),
        Token::new(token::TokenKind::Colon, Location(17)),
        Token::new(token::TokenKind::Ident("x"), Location(18)),
        Token::new(token::TokenKind::DoubleSemicolon, Location(20)),
    ];
    assert_eq!(tokenize.len(), correct.len());
    assert!(tokenize.iter().zip(correct.iter()).all(|(a, b)| a == b))
}

#[test]
fn tokenize2() {
    use location::Location;
    use token::{BinOpKind, DelimKind, Token, TokenKind};

    let source = Source::String(
        r#"
        func f(x y i32) i32: 
            x + 1 == y - 1;;
        "#
        .to_string(),
    );
    let tokenize: Vec<Token> = tokenize(&source).into_iter().collect();
    for token in &tokenize {
        println!("{:?}", token);
    }
    let correct = vec![
        Token::new(TokenKind::Ident("func"), Location(0)),
        Token::new(TokenKind::Ident("f"), Location(13)),
        Token::new(TokenKind::OpenDelim(DelimKind::Paren), Location(15)),
        Token::new(TokenKind::Ident("x"), Location(16)),
        Token::new(TokenKind::Ident("y"), Location(17)),
        Token::new(TokenKind::Ident("i32"), Location(19)),
        Token::new(TokenKind::CloseDelim(DelimKind::Paren), Location(23)),
        Token::new(TokenKind::Ident("i32"), Location(24)),
        Token::new(TokenKind::Colon, Location(28)),
        Token::new(TokenKind::Ident("x"), Location(29)),
        Token::new(TokenKind::BinOp(BinOpKind::Plus), Location(44)),
        Token::new(TokenKind::Int("1"), Location(46)),
        Token::new(TokenKind::BinOp(BinOpKind::Eq), Location(48)),
        Token::new(TokenKind::Ident("y"), Location(51)),
        Token::new(TokenKind::BinOp(BinOpKind::Minus), Location(53)),
        Token::new(TokenKind::Int("1"), Location(55)),
        Token::new(TokenKind::DoubleSemicolon, Location(57)),
    ];
    assert_eq!(tokenize.len(), correct.len());
    assert!(tokenize.iter().zip(correct.iter()).all(|(a, b)| a == b))
}
