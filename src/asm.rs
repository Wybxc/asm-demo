use std::fmt::Display;

pub struct AsmFunction<const N: usize> {
    asm: [u8; N],
}

impl<const N: usize> AsmFunction<N> {
    pub fn new(asm: [u8; N]) -> AsmFunction<N> {
        AsmFunction { asm }
    }

    #[allow(dead_code)]
    pub fn as_bytes(&self) -> &[u8] {
        &self.asm
    }
}

impl<const N: usize> Display for AsmFunction<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (i, byte) in self.asm.iter().enumerate() {
            write!(f, "{:02x} ", byte)?;
            if i % 8 == 7 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! asm_function {
    ($($t: tt)*) => {
        AsmFunction::new(asm_code!([$($t)*;]))
    };
}
