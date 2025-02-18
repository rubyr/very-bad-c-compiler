// TODO: remove this once things are starting to become more in place
#![allow(dead_code)]

mod error;
mod lexer;
mod parser;

use clap::Parser;
use error::{len_errors, ERRORS};
use lexer::lex;
use parser::parse_program;
use std::{fs, path::PathBuf};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    file: String,

    #[arg(long, default_value_t = false)]
    lex: bool,

    #[arg(long, default_value_t = false)]
    parse: bool,

    #[arg(long, default_value_t = false)]
    codegen: bool,

    #[arg(short = 'S', default_value_t = false)]
    asm_only: bool,
}

// ideally i think we'd eventually want to implement Display on errors and then
// return a `Result<(), ErrorList>` or something? but for now we're just
// returning nothing because we're LAZY
fn main() -> Result<(), ()> {
    let args = Args::parse();
    let file_path = PathBuf::from(args.file);

    if file_path.extension().and_then(|e| e.to_str()) != Some("c") {
        panic!("invalid file path provided");
    }

    let src = fs::read_to_string(&file_path).unwrap_or_else(|_| {
        panic!(
            "could not open provided file: {}",
            file_path.to_string_lossy()
        )
    });

    let mut lexed = lex(src);

    if len_errors() > 0 {
        errors!().print();
        return Err(());
    }
    if args.lex {
        return Ok(());
    }

    let parsed = parse_program(&mut lexed);

    dbg!(parsed);

    if len_errors() > 0 {
        errors!().print();
        return Err(());
    }
    if args.parse {
        return Ok(());
    }

    Ok(())
}
