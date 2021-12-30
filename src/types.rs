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

#[derive(Debug, Clone, PartialEq)]
pub enum Tok<'a> {
    Var(&'a str),
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

#[derive(Debug, Clone, PartialEq)]
pub enum BinOpKind {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExprKind<'a> {
    Var(&'a str),
    BinOp(BinOpKind, Box<Expr<'a>>, Box<Expr<'a>>),
}

pub type Expr<'a> = Spanned<ExprKind<'a>>;

#[derive(Debug, Clone, PartialEq)]
pub enum StmtKind<'a> {
    Let(&'a str, Box<Expr<'a>>),
    Print(Expr<'a>),
}

pub type Stmt<'a> = Spanned<StmtKind<'a>>;