# asm-demo

用 Rust 的过程宏写的小型 80386 汇编器（支持部分指令）。

```rust
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
// b9 00 00 00 00 e8 02 00
// 00 00 eb 06 68 e0 c3 40
// 00 c3 c3
```