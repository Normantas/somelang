use winnow::combinator::{fail, opt};
use winnow::error::{ErrMode, ErrorKind, ParserError, StrContext};
use winnow::token::any;
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
    Binary(Literal, BinaryOp, Literal),
}

pub fn parse_expr(input: &mut &[Token]) -> PResult<Expr> {
    if let Some((left_literal, binary_op, right_literal)) = opt(parse_binary_op).parse_next(input)? {
        Ok(Expr::Binary(
            left_literal,
            binary_op,
            right_literal,
        ))
    } else if let Some(literal) = opt(parse_literal).parse_next(input)? {
        Ok(Expr::Literal(literal))
    } else {
        fail.context(StrContext::Label("no matching parser found"))
            .parse_next(input)
    }
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    Integer(i64),
    Float(f64),
    Bool(bool),
    String(String),
}

fn parse_literal(input: &mut &[Token]) -> PResult<Literal> {
    match any.parse_next(input)? {
        Token::Number(number) => Ok(Literal::Integer(number)),
        Token::String(string) => Ok(Literal::String(string)),
        _ => Err(ErrMode::from_error_kind(input, ErrorKind::Fail)),
    }
}

#[derive(Debug, PartialEq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
}

fn parse_binary_op(input: &mut &[Token]) -> PResult<(Literal, BinaryOp, Literal)> {
    let parse_binary = |input: &mut &[Token]| match any.parse_next(input)? {
        Token::Add => Ok(BinaryOp::Add),
        Token::Sub => Ok(BinaryOp::Sub),
        Token::Mul => Ok(BinaryOp::Mul),
        Token::Div => Ok(BinaryOp::Div),
        _ => Err(ErrMode::from_error_kind(input, ErrorKind::Verify)),
    };

    (parse_literal, parse_binary, parse_literal).parse_next(input)
}
