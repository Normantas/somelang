use std::{fs, path::PathBuf};

use anyhow::Context;
use clap::{Parser, Subcommand};

mod interpreter;
mod lex;
mod parse;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Compile { file: PathBuf },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Command::Compile { file } => {
            let source =
                fs::read_to_string(file).with_context(|| format!("while reading file {file:?}"))?;

            let tokens = lex::lex(&source);
            let ast = parse::parse(tokens)?;

            println!("{ast:#?}");

            interpreter::interpet(ast)?;
        }
    }

    Ok(())
}
