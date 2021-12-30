use plex::lexer;
use thiserror::Error;

use crate::types::{ Spanned, Tok };

#[derive(Debug, Clone, PartialEq, Error)]
pub enum Error {
    #[error("unclosed comment")]
    UnclosedComment,
    #[error("unrecognized token `{0}`")]
    UnrecognizedToken(String),
    #[error("integer constant `{0}` is too large")]
    TooLargeInteger(String)
}

enum LexState<'a> {
    Token(Tok<'a>),
    Skip,
    Err(Error)
}

lexer! {
    fn next_token(text: 'input) -> LexState<'input>;

    r"[\t\n\r ]" => LexState::Skip,
    r"\+" => LexState::Token(Tok::Plus),
    r"\-" => LexState::Token(Tok::Minus),
    r"\*" => LexState::Token(Tok::Star),
    r"/" => LexState::Token(Tok::Slash),
    r"\(" => LexState::Token(Tok::LPar),
    r"\)" => LexState::Token(Tok::RPar),
    r"let" => LexState::Token(Tok::Let),
    r"=" => LexState::Token(Tok::Equal),
    r";" => LexState::Token(Tok::SemiColon),
    r"print" => LexState::Token(Tok::Print),
    r"[a-z][0-9A-Za-z_]*" => LexState::Token(Tok::Ident(text)),
    r"[0-9]+" => {
        if let Ok(i) = text.parse() {
            LexState::Token(Tok::Num(i))
        } else {
            LexState::Err(Error::TooLargeInteger(text.to_string()))
        }
    },
    r"." => LexState::Err(Error::UnrecognizedToken(text.to_owned()))
}

#[derive(Debug, Clone)]
pub struct Lexer<'input> {
    original: &'input str,
    remaining: &'input str
}

// lalrpop が受理する形式に合わせる
type Result<'a> = std::result::Result<(usize, Tok<'a>, usize), Spanned<Error>>;

impl<'input> Iterator for Lexer<'input> {
    type Item = Result<'input>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let state = next_token(self.remaining);
            if state.is_none() {
                // EOF
                return None;
            }

            let (state, remaining) = state.unwrap();

            let lo = self.original.len() - self.remaining.len();
            let hi = self.original.len() - remaining.len();
            self.remaining = remaining;

            match state {
                LexState::Token(tok) => {
                    // 正しいトークンなら lalrpop が受理する形に変換
                    return Some(Ok((lo, tok, hi)));
                },
                LexState::Skip => continue, // 空白は読み飛ばす
                LexState::Err(e) => {
                    // エラーであればそれを返す
                    return Some(Err(Spanned::new(e, (lo, hi))));
                },
            }
        }
    }
}