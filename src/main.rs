mod lexer;

use lexer::lex;
use std::{fs, path::PathBuf};
use clap::Parser;

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

fn main() {
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

    dbg!(lex(src));
}
