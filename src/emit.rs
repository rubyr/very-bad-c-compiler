use std::fmt::format;

use crate::{
    lexer::TokenType,
    parser_asm::{ASMFunction, ASMProgram, Instruction, Operand, Register},
};

const LINUX_MUMBO_JUMBO: &'static str = ".section .note.GNU-stack,\"\",@progbits";

pub fn emit_code(ast: Vec<ASMProgram>) -> String {
    let mut out = String::new();
    for ast in ast {
        match ast {
            ASMProgram::Function(func) => {
                emit_function(func, &mut out);
            }
        };
    }
    out += LINUX_MUMBO_JUMBO;
    out
}

fn emit_function(func: ASMFunction, out: &mut String) {
    *out += &format(format_args!(
        r"   .globl {}
{}:
",
        func.ident.substring, func.ident.substring
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
