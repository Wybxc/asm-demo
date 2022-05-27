use std::fmt::{Debug, Display};

pub enum AsmCode {
    Mov(DstSrc),
    Add(DstSrc),
    Sub(DstSrc),
    Shl(DstSrc),
    Shr(DstSrc),
    Push(Src),
    Pop(Dst),
    Call(Src),
    Ret,
    Jmp(Src),
    Nop,
}

pub struct DstSrc(pub Dst, pub Src);

pub enum Dst {
    Reg(u8),
    Offset(u8, i8),
    Address(u32),
}

pub enum Src {
    Reg(u8),
    Offset(u8, i8),
    Address(u32),
    Immediate(i32),
}

pub const EAX: u8 = 0;
pub const ECX: u8 = 1;
pub const EDX: u8 = 2;
pub const EBX: u8 = 3;
pub const ESP: u8 = 4;
pub const EBP: u8 = 5;
pub const ESI: u8 = 6;
pub const EDI: u8 = 7;

fn reg_name(reg: u8) -> &'static str {
    match reg {
        EAX => "eax",
        ECX => "ecx",
        EDX => "edx",
        EBX => "ebx",
        ESP => "esp",
        EBP => "ebp",
        ESI => "esi",
        EDI => "edi",
        _ => panic!("未知寄存器"),
    }
}

impl Display for AsmCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AsmCode::Mov(DstSrc(dst, src)) => write!(f, "mov {}, {}", dst, src),
            AsmCode::Add(DstSrc(dst, src)) => write!(f, "add {}, {}", dst, src),
            AsmCode::Sub(DstSrc(dst, src)) => write!(f, "sub {}, {}", dst, src),
            AsmCode::Shl(DstSrc(dst, src)) => write!(f, "shl {}, {}", dst, src),
            AsmCode::Shr(DstSrc(dst, src)) => write!(f, "shr {}, {}", dst, src),
            AsmCode::Push(src) => write!(f, "push {}", src),
            AsmCode::Pop(dst) => write!(f, "pop {}", dst),
            AsmCode::Call(src) => write!(f, "call {}", src),
            AsmCode::Ret => write!(f, "ret"),
            AsmCode::Jmp(src) => write!(f, "jmp {}", src),
            AsmCode::Nop => write!(f, "nop"),
        }
    }
}

impl Display for Dst {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Dst::Reg(reg) => write!(f, "{}", reg_name(*reg)),
            Dst::Offset(reg, offset) => write!(f, "[{}+0x{:02x}]", reg_name(*reg), offset),
            Dst::Address(address) => write!(f, "0x{:08x}", address),
        }
    }
}

impl Display for Src {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Src::Reg(reg) => write!(f, "{}", reg_name(*reg)),
            Src::Offset(reg, offset) => write!(f, "[{}+0x{:02x}]", reg_name(*reg), offset),
            Src::Address(address) => write!(f, "0x{:08x}", address),
            Src::Immediate(value) => write!(f, "{}", value),
        }
    }
}

impl Debug for AsmCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "{}", self)
    }
}

impl Debug for Dst {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "{}", self)
    }
}

impl Debug for Src {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "{}", self)
    }
}

pub fn asm_to_bytes(asm: &AsmCode) -> Vec<u8> {
    match asm {
        // MOV
        AsmCode::Mov(DstSrc(Dst::Reg(dst), Src::Reg(src))) => {
            vec![0x8B, 0xC0 | (dst << 3) | src]
        }
        AsmCode::Mov(DstSrc(Dst::Reg(dst), Src::Offset(reg, offset))) => {
            let mut bytes = vec![0x8B, 0x40 | (dst << 3) | reg];
            bytes.extend(offset.to_le_bytes().iter());
            bytes
        }
        AsmCode::Mov(DstSrc(Dst::Reg(dst), Src::Address(address))) => {
            let mut bytes = vec![0x8B, 0x0D | (dst << 3)];
            bytes.extend(address.to_le_bytes().iter());
            bytes
        }
        AsmCode::Mov(DstSrc(Dst::Reg(dst), Src::Immediate(imm))) => {
            let mut bytes = vec![0xB8 | dst];
            bytes.extend(imm.to_le_bytes().iter());
            bytes
        }
        AsmCode::Mov(DstSrc(Dst::Offset(reg, offset), Src::Reg(src))) => {
            let mut bytes = vec![0x89, 0x40 | (src << 3) | reg];
            bytes.extend(offset.to_le_bytes().iter());
            bytes
        }
        AsmCode::Mov(DstSrc(Dst::Offset(reg, offset), Src::Immediate(imm))) => {
            let mut bytes = vec![0xC7, 0x40 | reg];
            bytes.extend(offset.to_le_bytes().iter());
            bytes.extend(imm.to_le_bytes().iter());
            bytes
        }

        // ADD
        AsmCode::Add(DstSrc(Dst::Reg(dst), Src::Reg(src))) => {
            vec![0x03, 0xC0 | (dst << 3) | src]
        }
        AsmCode::Add(DstSrc(Dst::Reg(dst), Src::Immediate(imm))) => {
            let mut bytes = vec![0x81, 0xC0 | dst];
            bytes.extend(imm.to_le_bytes().iter());
            bytes
        }

        // SUB
        AsmCode::Sub(DstSrc(Dst::Reg(dst), Src::Reg(src))) => {
            vec![0x2B, 0xC0 | (dst << 3) | src]
        }
        AsmCode::Sub(DstSrc(Dst::Reg(dst), Src::Immediate(imm))) => {
            let mut bytes = vec![0x81, 0xE8 | dst];
            bytes.extend(imm.to_le_bytes().iter());
            bytes
        }

        // SHL
        AsmCode::Shl(DstSrc(Dst::Reg(dst), Src::Reg(src))) => {
            vec![0xD3, 0xE0 | (dst << 3) | src]
        }
        AsmCode::Shl(DstSrc(Dst::Reg(dst), Src::Immediate(imm))) => {
            if *imm == 1 {
                vec![0xD1, 0xE0 | dst]
            } else {
                vec![0xC1, 0xE0 | dst, imm.to_le_bytes()[0]]
            }
        }

        // SHR
        AsmCode::Shr(DstSrc(Dst::Reg(dst), Src::Reg(src))) => {
            vec![0xD3, 0xE8 | (dst << 3) | src]
        }
        AsmCode::Shr(DstSrc(Dst::Reg(dst), Src::Immediate(imm))) => {
            if *imm == 1 {
                vec![0xD1, 0xE8 | dst]
            } else {
                vec![0xC1, 0xE8 | dst, imm.to_le_bytes()[0]]
            }
        }

        // PUSH
        AsmCode::Push(Src::Reg(src)) => vec![0x50 | src],
        AsmCode::Push(Src::Immediate(imm)) => {
            let mut bytes = vec![0x68];
            bytes.extend(imm.to_le_bytes().iter());
            bytes
        }

        // POP
        AsmCode::Pop(Dst::Reg(dst)) => vec![0x58 | dst],

        // CALL
        AsmCode::Call(Src::Reg(src)) => vec![0xFF, 0xD0 | src],
        AsmCode::Call(Src::Immediate(imm)) => {
            let mut bytes = vec![0xE8];
            bytes.extend(imm.to_le_bytes().iter());
            bytes
        }

        // RET
        AsmCode::Ret => vec![0xC3],

        // JMP
        AsmCode::Jmp(Src::Reg(src)) => vec![0xFF, 0xE0 | src],
        AsmCode::Jmp(Src::Immediate(addr)) if -128 <= *addr && *addr <= 127 => {
            vec![0xEB, addr.to_le_bytes()[0]]
        }
        AsmCode::Jmp(Src::Immediate(addr)) => {
            let mut bytes = vec![0xE9];
            bytes.extend(addr.to_le_bytes().iter());
            bytes
        }

        // NOP
        AsmCode::Nop => vec![0x90],
        _ => panic!("不支持的指令: {}", asm),
    }
}
