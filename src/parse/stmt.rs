use winnow::combinator::{fail, opt, repeat_till};
use winnow::token::one_of;
use winnow::{seq, PResult, Parser};

use crate::lex::Token;
use crate::parse::expr::parse_literal;

use super::expr::{parse_expr, Expr};
use super::Literal;

/// A line of code.
#[derive(Debug, PartialEq)]
pub enum Stmt {
    FunctionCall(FunctionCall),
    SetVariable(SetVariable),
}

pub fn parse_stmt(input: &mut &[Token]) -> PResult<Stmt> {
    if let Some(function_call) = opt(parse_function_call).parse_next(input)? {
        Ok(Stmt::FunctionCall(function_call))
    } else if let Some(set_variable) = opt(parse_set_variable).parse_next(input)? {
        Ok(Stmt::SetVariable(set_variable))
    } else {
        fail(input)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionCall {
    pub name: String,
    pub args: Vec<Literal>,
}

fn parse_function_call(input: &mut &[Token]) -> PResult<FunctionCall> {
    seq! {FunctionCall {
        name: one_of(|t| matches!(t, Token::Ident(_))).map(|t| match t {
            Token::Ident(v) => v,
            _ => unreachable!(),
        }),
        _: one_of(Token::LParen),
        args: repeat_till(0.., parse_literal, one_of(Token::RParen)).map(|x| {
            x.0
        }),
    }}
    .parse_next(input)
}

#[derive(Debug, PartialEq, Clone)]
pub struct SetVariable {
    pub name: String,
    pub value: Expr,
}

fn parse_set_variable(input: &mut &[Token]) -> PResult<SetVariable> {
    seq! {SetVariable {
        _: one_of(Token::Set),
        name: one_of(|t| matches!(t, Token::Ident(_))).map(|t| match t {
            Token::Ident(v) => v,
            _ => unreachable!(),
        }),
        _: one_of(Token::Arrow),
        value: parse_expr,
    }}
    .parse_next(input)
}
