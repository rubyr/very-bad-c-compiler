// TODO: remove this once things are starting to become more in place
#![allow(dead_code)]

mod emit;
mod error;
mod lexer;
mod parser_asm;
mod parser_c;

use clap::Parser;
use emit::emit_code;
use error::{len_errors, ERRORS};
use lexer::lex;
use parser_asm::parse_ast;
use parser_c::parse_program;
use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
    process::Command,
};

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

    if len_errors() > 0 {
        errors!().print();
        return Err(());
    }
    if args.parse {
        return Ok(());
    }

    let asm = parse_ast(&mut parsed.expect("couldn't get ast for some reason"));

    if len_errors() > 0 {
        errors!().print();
        return Err(());
    }
    if args.codegen {
        return Ok(());
    }

    let out_asm_path = {
        let mut p = file_path.clone();
        p.set_extension("s");
        p
    };
    let out_exec_path = {
        let mut p = file_path.clone();
        p.set_extension("");
        p
    };

    let mut file = File::create(&out_asm_path).unwrap_or_else(|_| {
        panic!(
            "couldn't open output file at {}",
            out_asm_path.to_str().unwrap()
        )
    });

    write!(file, "{}", emit_code(asm.unwrap())).expect("failed to write output!");

    if args.asm_only {
        return Ok(());
    }
    // use gcc to assemble and link the output assembly file because the book
    // doesn't cover doing that
    Command::new("gcc")
        .args([
            out_asm_path.to_str().unwrap(),
            "-o",
            out_exec_path.to_str().unwrap(),
        ])
        .output()
        .expect("couldn't link output assembly");

    fs::remove_file(out_asm_path).expect("failed to remove assembly");

    Ok(())
}
