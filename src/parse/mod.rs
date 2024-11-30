use function::Function;
use logos::Lexer;

use crate::lex::{LexingError, Token};

mod expr;
mod function;
mod stmt;

pub enum ModuleType {
    Bin,
    Lib,
}

/// AST (Abstract Syntax Tree) and metadata of a module.
pub struct Module {
    functions: Vec<Function>,
    module_type: ModuleType,
}

pub fn parse(token_stream: Lexer<'_, Token>) -> anyhow::Result<Module> {
    let tokens: Vec<Token> = token_stream.collect::<Result<Vec<Token>, LexingError>>()?;

    for token in &tokens {
        println!("{token:?}");
    }

    let mut slice: &[Token] = &tokens;
    match function::parse_function(&mut slice) {
        Ok(output) => println!("{output:?}"),
        Err(err) => println!("{err}"),
    }

    todo!();
}
