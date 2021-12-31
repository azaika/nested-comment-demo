mod types;
mod lexer;

use thiserror::Error;
use types::{Stmt, Tok, Spanned};

#[derive(Debug, Error, Clone, PartialEq)]
enum ParseErrorKind<'a> {
    #[error("parse error: unexpected end of file")]
    Eof,
    #[error("lexer error: {0}")]
    Lexical(lexer::Error),
    #[error("parse error: found extra token `{0:?}`")]
    ExtraToken(Tok<'a>),
    #[error("parse error: unrecognized token `{0:?}`, expected `{1}`")]
    UnrecognizedToken(Tok<'a>, String)
}

type ParseError<'a> = Spanned<ParseErrorKind<'a>>;

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub grammer);

type LalrpopError<'a> = lalrpop_util::ParseError<usize, Tok<'a>, Spanned<lexer::Error>>;

fn convert_error(err: LalrpopError) -> ParseError {
    match err {
        LalrpopError::InvalidToken { location } => Spanned::new(ParseErrorKind::Eof, (location, location)),
        LalrpopError::ExtraToken { token: (lo, tok, hi) } => Spanned::new(ParseErrorKind::ExtraToken(tok), (lo, hi)),
        LalrpopError::User { error } => {
            Spanned::new(
                ParseErrorKind::Lexical(error.item),
                error.span
            )
        },
        LalrpopError::UnrecognizedToken {
            token: (lo, tok, hi),
            expected
        } => {
            // take only first expected candidate
            assert!(!expected.is_empty());
            let expected = expected[0].clone();

            Spanned::new(ParseErrorKind::UnrecognizedToken(tok, expected), (lo, hi))
        }
        LalrpopError::UnrecognizedEOF { location, .. } => Spanned::new(ParseErrorKind::Eof, (location, location)),
    }
}

fn parse<'input>(src: &'input str) -> Result<Vec<Stmt>, ParseError<'input>> {
    let lex = lexer::Lexer::new(src);
    let parser = grammer::ProgramParser::new();

    parser.parse(lex).map_err(convert_error)
}

fn load_file(path: &str) -> String {
    std::fs::read_to_string(path).unwrap()
}

fn main() {
    let src = load_file("test.txt");
    let ast = parse(&src).unwrap();
    println!("{:#?}", ast);
}
