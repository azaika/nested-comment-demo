use thiserror::Error;
use crate::types::{Stmt, Tok, Spanned};
use crate::lexer;

#[derive(Debug, Clone, PartialEq, Error)]
pub enum ErrorKind<'a> {
    #[error("parse error: unexpected end of file")]
    Eof,
    #[error("lexer error: {0}")]
    Lexical(lexer::Error),
    #[error("parse error: found extra token `{0:?}`")]
    ExtraToken(Tok<'a>),
    #[error("parse error: unrecognized token `{0:?}`, expected `{1}`")]
    UnrecognizedToken(Tok<'a>, String)
}

pub type Error<'a> = Spanned<ErrorKind<'a>>;

type LalrpopError<'a> = lalrpop_util::ParseError<usize, Tok<'a>, Spanned<lexer::Error>>;

lalrpop_mod!(pub grammer);

fn convert_error(err: LalrpopError) -> Error {
    match err {
        LalrpopError::InvalidToken { location } => Spanned::new(ErrorKind::Eof, (location, location)),
        LalrpopError::ExtraToken { token: (lo, tok, hi) } => Spanned::new(ErrorKind::ExtraToken(tok), (lo, hi)),
        LalrpopError::User { error } => Spanned::new(ErrorKind::Lexical(error.item), error.span),
        LalrpopError::UnrecognizedToken {
            token: (lo, tok, hi),
            expected
        } => {
            // take only first expected candidate
            assert!(!expected.is_empty());
            let expected = expected[0].clone();

            Spanned::new(ErrorKind::UnrecognizedToken(tok, expected), (lo, hi))
        },
        LalrpopError::UnrecognizedEOF { location, .. } => Spanned::new(ErrorKind::Eof, (location, location)),
    }
}

pub fn parse<'input>(src: &'input str) -> Result<Vec<Stmt>, Error> {
    let lex = lexer::Lexer::new(src);
    let parser = grammer::ProgramParser::new();

    parser.parse(lex).map_err(convert_error)
}
