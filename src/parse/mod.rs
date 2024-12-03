use winnow::combinator::repeat;
use winnow::{PResult, Parser};

use function::{parse_function, Function};
use logos::Lexer;

use crate::lex::{LexingError, Token};

mod expr;
mod function;
mod stmt;

#[derive(Debug, PartialEq)]
pub enum ModuleType {
    Bin,
    Lib,
}

/// AST (Abstract Syntax Tree) and metadata of a module.
#[derive(Debug, PartialEq)]
pub struct Module {
    pub functions: Vec<Function>,
    pub module_type: ModuleType,
}

pub fn parse(token_stream: Lexer<'_, Token>) -> anyhow::Result<Module> {
    let tokens: Vec<Token> = token_stream.collect::<Result<Vec<Token>, LexingError>>()?;

    for token in &tokens {
        println!("{token:?}");
    }

    let mut slice: &[Token] = &tokens;
    match parse_module(&mut slice) {
        Ok(module) => Ok(module),
        Err(err) => Err(anyhow::anyhow!(err)),
    }
}

pub fn parse_module(input: &mut &[Token]) -> PResult<Module> {
    let functions: Vec<Function> = repeat(0..,
        parse_function
    ).parse_next(input)?;

    // TODO: Change this
    let module_type = if functions.iter().any(|f| f.name == "main") {
        ModuleType::Bin
    } else {
        ModuleType::Lib
    };

    Ok(Module {
        functions,
        module_type,
    })
}
