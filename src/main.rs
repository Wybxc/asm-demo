mod asm;
use asm::*;
use asm_macro::asm_code;

fn main() {
    let asm = asm_function![
        mov  ecx, $0;  // Commit
        call 0x02;
        jmp  0x06;
        push 0x40c3e0;
        ret;
        ret;
    ];
    println!("{}", asm.apply_templates(&[0x123456]));
}
