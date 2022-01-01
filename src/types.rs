// ソースコード中の位置情報を範囲で表す
pub type Span = (usize, usize);

#[derive(Debug, Clone, PartialEq)]
pub struct Spanned<T> {
    pub item: T,
    pub span: Span
}

impl<T> Spanned<T> {
    pub fn new(item: T, span: Span) -> Self {
        Self {
            item,
            span
        }
    }
}

// レキサーで用いるトークン
#[derive(Debug, Clone, PartialEq)]
pub enum Tok<'a> {
    Num(i64),
    Ident(&'a str),
    Plus,
    Minus,
    Star,
    Slash,
    LPar,
    RPar,
    Let,
    Equal,
    SemiColon,
    Print,
}

// 二項演算の種類
#[derive(Debug, Clone, PartialEq)]
pub enum BinOpKind {
    Add,
    Sub,
    Mul,
    Div,
}

// 式

#[derive(Debug, Clone, PartialEq)]
pub enum ExprKind<'a> {
    Num(i64),
    Var(&'a str),
    BinOp(BinOpKind, Box<Expr<'a>>, Box<Expr<'a>>),
}

pub type Expr<'a> = Spanned<ExprKind<'a>>;

// 文

#[derive(Debug, Clone, PartialEq)]
pub enum StmtKind<'a> {
    Let(&'a str, Box<Expr<'a>>),
    Print(Box<Expr<'a>>),
}

pub type Stmt<'a> = Spanned<StmtKind<'a>>;