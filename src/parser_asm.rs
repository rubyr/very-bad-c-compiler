use crate::{
    error::CompilerError,
    lexer::Token,
    parser_c::{CAst, CExpression, CProgram, CStatement},
};

#[derive(Debug, Clone)]
pub enum ASMProgram {
    Function(ASMFunction),
}

#[derive(Debug, Clone)]
pub struct ASMFunction {
    ident: Token,
    statement: Vec<Instruction>,
}

#[derive(Debug, Clone)]
pub enum Register {
    Eax,
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Mov { src: Operand, dst: Operand },
    Ret,
}

#[derive(Debug, Clone)]
pub enum Operand {
    Imm(Token),
    Register(Register),
}

#[derive(Debug, Clone)]
struct CParserError {
    line: usize,
    index: usize,
    string: String,
}

impl CompilerError for CParserError {
    fn line(&self) -> usize {
        self.line
    }

    fn index(&self) -> usize {
        self.index
    }

    fn to_string(&self) -> String {
        self.string.clone()
    }
}

pub fn parse_ast(ast: &mut CAst) -> Option<Vec<ASMProgram>> {
    let mut p = vec![];
    while !ast.is_empty() {
        p.push(ASMProgram::Function(parse_fn(ast.pop_front()?)?));
    }
    Some(p)
}

fn parse_fn(func: CProgram) -> Option<ASMFunction> {
    if let CProgram::Function(func) = func {
        let ident = func.ident;
        let mut body = vec![];
        for statement in func.statement {
            body.push(parse_statement(statement)?);
        }
        return Some(ASMFunction {
            ident,
            statement: body.concat(),
        });
    }
    None
}

fn parse_statement(st: CStatement) -> Option<Vec<Instruction>> {
    match st {
        CStatement::Return(ex) => Some(vec![
            Instruction::Mov {
                src: parse_expression(ex)?,
                dst: Operand::Register(Register::Eax),
            },
            Instruction::Ret,
        ]),
        _ => None,
    }
}

fn parse_expression(ex: CExpression) -> Option<Operand> {
    match ex {
        CExpression::Constant(token) => Some(Operand::Imm(token)),
        _ => None,
    }
}
