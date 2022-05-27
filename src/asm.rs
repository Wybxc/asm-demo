use std::fmt::Display;

pub struct AsmFunction<const N: usize, const M: usize> {
    asm: [u8; N],
    templates: [usize; M],
}

impl<const N: usize, const M: usize> AsmFunction<N, M> {
    pub fn new((asm, templates): ([u8; N], [usize; M])) -> Self {
        AsmFunction { asm, templates }
    }

    #[allow(dead_code)]
    pub fn as_bytes(&self) -> &[u8] {
        &self.asm
    }

    pub fn apply_templates_mut(&mut self, values: &[i32; M]) -> &mut Self {
        for (i, &template) in self.templates.iter().enumerate() {
            let le = values[i].to_le_bytes();
            self.asm[template] = le[0];
            self.asm[template + 1] = le[1];
            self.asm[template + 2] = le[2];
            self.asm[template + 3] = le[3];
        }
        self
    }

    pub fn apply_templates(&self, values: &[i32; M]) -> Self {
        let mut new_asm = self.clone();
        new_asm.apply_templates_mut(values);
        new_asm
    }
}

impl<const N: usize, const M: usize> Clone for AsmFunction<N, M> {
    fn clone(&self) -> Self {
        AsmFunction {
            asm: self.asm,
            templates: self.templates,
        }
    }
}

impl<const N: usize, const M: usize> Display for AsmFunction<N, M> {
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
