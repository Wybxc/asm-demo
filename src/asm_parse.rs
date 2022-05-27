use super::asm_x86::*;
use proc_macro::{token_stream::IntoIter as TokenStreamIter, Delimiter, Ident, Literal, TokenTree};

pub fn parse(tokens: &mut TokenStreamIter) -> Vec<AsmCode> {
    let mut asm_codes = Vec::new();
    while let Some(asm_code) = parse_instruction(tokens) {
        parse_punct(tokens, ';').expect("语法错误：缺少分号");
        asm_codes.push(asm_code);
    }
    asm_codes
}

fn parse_instruction(tokens: &mut TokenStreamIter) -> Option<AsmCode> {
    let token = tokens.next()?;
    Some(match token {
        TokenTree::Ident(ident) => match ident
            .span()
            .source_text()
            .map(|s| s.to_lowercase())
            .as_deref()
        {
            Some("mov") => AsmCode::Mov(parse_dst_src(tokens)),
            Some("add") => AsmCode::Add(parse_dst_src(tokens)),
            Some("sub") => AsmCode::Sub(parse_dst_src(tokens)),
            Some("shl") => AsmCode::Shl(parse_dst_src(tokens)),
            Some("shr") => AsmCode::Shr(parse_dst_src(tokens)),
            Some("push") => AsmCode::Push(parse_src(tokens)),
            Some("pop") => AsmCode::Pop(parse_dst(tokens)),
            Some("call") => AsmCode::Call(parse_src(tokens)),
            Some("ret") => AsmCode::Ret,
            Some("jmp") => AsmCode::Jmp(parse_src(tokens)),
            Some("nop") => AsmCode::Nop,
            Some(s) => panic!("未知指令：{}", s),
            None => panic!("未知指令"),
        },
        TokenTree::Punct(punct) if punct.as_char() == ';' => parse_instruction(tokens)?, // 跳过多余的分号
        _ => panic!("语法错误：在指令开始处遇到意外的符号`{}`", token),
    })
}

fn parse_punct(tokens: &mut TokenStreamIter, punct_char: char) -> Option<()> {
    match tokens.next() {
        Some(TokenTree::Punct(punct)) if punct.as_char() == punct_char => Some(()),
        _ => None,
    }
}

fn parse_dst_src(tokens: &mut TokenStreamIter) -> DstSrc {
    let dst = parse_dst(tokens);
    parse_punct(tokens, ',').expect("语法错误：缺少逗号");
    let src = parse_src(tokens);
    DstSrc(dst, src)
}

fn parse_dst(tokens: &mut TokenStreamIter) -> Dst {
    let token = tokens.next().expect("语法错误：提前终止");
    match token {
        TokenTree::Ident(ident) => Dst::Reg(parse_reg(&ident)),
        TokenTree::Group(group) if group.delimiter() == Delimiter::Bracket => {
            let mut tokens = group.stream().into_iter();
            let reg = tokens.next().expect("语法错误：提前终止");
            match reg {
                TokenTree::Ident(ident) => {
                    let (reg, offset) = parse_offset(&ident, &mut tokens);
                    Dst::Offset(reg, offset)
                }
                TokenTree::Literal(lit) => {
                    let address = parse_literal(&lit);
                    Dst::Address(address)
                }
                _ => panic!("语法错误：在地址处遇到意外的符号`{}`", reg),
            }
        }
        _ => panic!("语法错误：在目标处遇到意外的符号`{}`", token),
    }
}

fn parse_src(tokens: &mut TokenStreamIter) -> Src {
    let token = tokens.next().expect("语法错误：提前终止");
    match token {
        TokenTree::Ident(ident) => Src::Reg(parse_reg(&ident)),
        TokenTree::Literal(literal) => Src::Immediate(parse_literal(&literal)),
        TokenTree::Group(group) if group.delimiter() == Delimiter::Bracket => {
            let mut tokens = group.stream().into_iter();
            let reg = tokens.next().expect("语法错误：提前终止");
            match reg {
                TokenTree::Ident(ident) => {
                    let (reg, offset) = parse_offset(&ident, &mut tokens);
                    Src::Offset(reg, offset)
                }
                TokenTree::Literal(lit) => {
                    let address = parse_literal(&lit);
                    Src::Address(address)
                }
                _ => panic!("语法错误：在地址处遇到意外的符号`{}`", reg),
            }
        }
        _ => panic!("语法错误：在值处遇到意外的符号`{}`", token),
    }
}

fn parse_reg(ident: &Ident) -> u8 {
    match ident
        .span()
        .source_text()
        .map(|s| s.to_lowercase())
        .as_deref()
    {
        Some("eax") => EAX,
        Some("ebx") => EBX,
        Some("ecx") => ECX,
        Some("edx") => EDX,
        Some("esp") => ESP,
        Some("ebp") => EBP,
        Some("esi") => ESI,
        Some("edi") => EDI,
        Some(s) => panic!("未知寄存器：`{}`", s),
        None => panic!("未知寄存器"),
    }
}

fn parse_literal<T>(literal: &Literal) -> T
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{
    let token = literal.span().source_text().expect("语法错误：提前终止");
    match syn::parse_str::<syn::LitInt>(token.as_str())
        .unwrap()
        .base10_parse::<T>()
    {
        Ok(value) => value,
        Err(_) => panic!("语法错误：在字面量处遇到意外的符号`{}`", token),
    }
}

fn parse_offset(ident: &Ident, tokens: &mut TokenStreamIter) -> (u8, i8) {
    let reg = parse_reg(ident);
    if parse_punct(tokens, '+').is_some() {
        let offset = tokens.next().expect("语法错误：提前终止");
        match offset {
            TokenTree::Literal(lit) => {
                let offset = parse_literal(&lit);
                (reg, offset)
            }
            _ => panic!("语法错误：在地址偏移处遇到意外的符号`{}`", offset),
        }
    } else {
        (reg, 0)
    }
}
