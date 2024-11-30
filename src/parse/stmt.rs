use winnow::combinator::{fail, opt};
use winnow::{PResult, Parser};

use crate::lex::Token;

use super::expr::{parse_expr, Expr};

/// A line of code.
#[derive(Debug, PartialEq)]
pub enum Stmt {
    Expression(Expr),
}

pub fn parse_stmt(input: &mut &[Token]) -> PResult<Stmt> {
    if let Some(expr) = opt(parse_expr).parse_next(input)? {
        Ok(Stmt::Expression(expr))
    } else {
        fail(input)
    }
}
