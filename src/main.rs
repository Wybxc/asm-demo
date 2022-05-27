mod asm;
use asm::*;
use asm_demo::asm_code;

fn main() {
    let asm = asm_function![
        mov  ecx, 0;
        call 0x02;
        jmp  0x06;
        push 0x40c3e0;
        ret;
        ret;
    ];
    println!("{}", asm);
}
