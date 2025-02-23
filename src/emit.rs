use std::{env::consts::OS, fmt::format, sync::LazyLock};

use crate::{
    lexer::TokenType,
    parser_asm::{ASMFunction, ASMProgram, Instruction, Operand, Register},
};

const LINUX_MUMBO_JUMBO: &str = ".section .note.GNU-stack,\"\",@progbits\n";
const MACOS_FUNC_PREFIX: &str = "_";

static FUNC_PREFIX: LazyLock<&'static str> =
    std::sync::LazyLock::new(|| if OS == "macos" { MACOS_FUNC_PREFIX } else { "" });

pub fn emit_code(ast: Vec<ASMProgram>) -> String {
    let mut out = String::new();
    for ast in ast {
        match ast {
            ASMProgram::Function(func) => {
                emit_function(func, &mut out);
            }
        };
    }
    if OS == "linux" {
        out += LINUX_MUMBO_JUMBO;
    }
    out
}

fn emit_function(func: ASMFunction, out: &mut String) {
    *out += &format(format_args!(
        r"   .globl {}{}
{}{}:
",
        FUNC_PREFIX.to_string(),
        func.ident.substring,
        FUNC_PREFIX.to_string(),
        func.ident.substring
    ));
    emit_instructions(func.statements, out);
}

fn emit_instructions(statements: Vec<Instruction>, out: &mut String) {
    for inst in statements {
        match inst {
            Instruction::Ret => {
                *out += "   ret\n";
            }
            Instruction::Mov { src, dst } => {
                *out += &format(format_args!("   movl    {},{}\n", op(src), op(dst)));
            }
        }
    }
}

fn op(op: Operand) -> String {
    match op {
        Operand::Register(r) => match r {
            Register::Eax => "%eax".into(),
        },
        Operand::Imm(imm) => match imm.token_type {
            TokenType::Constant => format(format_args!("${}", imm.substring)),
            _ => todo!(),
        },
    }
}
