use winnow::combinator::{fail, opt};
use winnow::error::{ErrMode, ErrorKind, ParserError, StrContext};
use winnow::stream::Stream;
use winnow::{PResult, Parser};

use crate::lex::Token;

/// Something that can be reduced to a value (literals, operators, etc.).
///
/// Example:
/// '''
/// 5
/// 6*2
/// 12.3
/// true
/// "hello"
///
/// x = 5   <-- this is NOT an expression, becouse it modifies a variable
/// '''
///
/// Remember: expressions are statements (but not necessarily vice versa).
#[derive(Debug, PartialEq)]
pub enum Expr {
    Literal(Literal),
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    Integer(i64),
    Float(f64),
    Bool(bool),
    String(String),
}

pub fn parse_expr(input: &mut &[Token]) -> PResult<Expr> {
    if let Some(literal) = opt(parse_literal).parse_next(input)? {
        Ok(Expr::Literal(literal))
    } else {
        fail.context(StrContext::Label("expr"))
            .parse_next(input)
    }
}

fn parse_literal(input: &mut &[Token]) -> PResult<Literal> {
    let token = input
        .next_token()
        .ok_or_else(|| ErrMode::from_error_kind(input, ErrorKind::Token))?;

    match token {
        Token::Number(number) => Ok(Literal::Integer(number)),
        Token::String(string) => Ok(Literal::String(string)),
        _ => Err(ErrMode::from_error_kind(input, ErrorKind::Fail)),
    }
}
