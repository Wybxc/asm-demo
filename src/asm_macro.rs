#![feature(proc_macro_span)]

mod asm_parse;
mod asm_x86;
use proc_macro::{Delimiter, Group, Literal, Punct, Spacing, TokenStream, TokenTree};

#[proc_macro]
pub fn asm_code(input: TokenStream) -> TokenStream {
    let token = input.into_iter().next().expect("语法错误：输入为空");
    let asm_codes = match token {
        TokenTree::Group(group) if group.delimiter() == Delimiter::Bracket => {
            let mut tokens = group.stream().into_iter();
            asm_parse::parse(&mut tokens)
        }
        _ => panic!("语法错误：没有有效的方括号"),
    };
    let bytes = asm_codes
        .into_iter()
        .flat_map(|asm_code| asm_x86::asm_to_bytes(&asm_code))
        .flat_map(|byte| {
            [
                TokenTree::Literal(Literal::u8_suffixed(byte)),
                TokenTree::Punct(Punct::new(',', Spacing::Alone)),
            ]
        })
        .collect();
    TokenTree::Group(Group::new(Delimiter::Bracket, bytes)).into()
}
