use instructions;
use instructions::{Instr, Op, Reg8, Reg16};
use super::super::mem_map;
use super::super::hardware::Bus;

impl<B> super::Cpu<B> where B: Bus {
    #[inline(always)]
    pub fn push(&mut self, value: u16) {
        self.sp = self.sp.wrapping_sub(2);
        let addr = self.sp;
        self.write_word(addr, value);
    }

    #[inline(always)]
    pub fn pop(&mut self) -> u16 {
        let addr = self.sp;
        let value = self.read_word(addr);
        self.sp = self.sp.wrapping_add(2);
        return value;
    }
}