# asm-demo

用 Rust 的过程宏写的小型 80386 汇编器（支持部分指令）。

## 基础用法

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

## 扩展语法（模板）

在一个32位整数前加上 `$` 可以将其转换为模板，如 `$1` 表示默认值为 1 的模板。

```rust
let mut asm = asm_function![
    mov  ecx, $0;
    push $0;
];
```

使用 `apply_templates_mut` 或 `apply_templates` 可以应用模板：

```rust
asm.apply_templates_mut(&[0x123456, 0xABCDEF]);
println!("{}", asm);
// b9 56 34 12 00 68 ef cd
// ab 00
```