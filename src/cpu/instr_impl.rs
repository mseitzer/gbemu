use instructions;
use instructions::{Instr, Op, Reg8, Reg16};
use super::{ZERO, SUB, HCARRY, CARRY};
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

    #[inline(always)]
    pub fn rotate_left(&mut self, value: u8) -> u8 {
        // Bit 7 is shifted to carry, carry is shifted to bit 0
        let old_carry = self.flags.contains(CARRY);
        self.flags = CARRY.test(get_bit!(value, 7) == 1);
        let res = value << 1 | (old_carry as u8);
        self.flags = self.flags | ZERO.test(res == 0);
        res
    }

    #[inline(always)]
    pub fn rotate_left_carry(&mut self, value: u8) -> u8 {
        // Bit 7 is shifted to carry and bit 0
        self.flags = CARRY.test(get_bit!(value, 7) == 1);
        let res = value << 1 | (self.flags.contains(CARRY) as u8);
        self.flags = self.flags | ZERO.test(res == 0);
        res
    }

    #[inline(always)]
    pub fn rotate_right(&mut self, value: u8) -> u8 {
        // Bit 0 is shifted to carry, carry is shifted to bit 7
        let old_carry = self.flags.contains(CARRY);
        self.flags = CARRY.test(get_bit!(value, 0) == 1);
        let res = (old_carry as u8) << 7 | value >> 1;
        self.flags = self.flags | ZERO.test(res == 0);
        res
    }

    #[inline(always)]
    pub fn rotate_right_carry(&mut self, value: u8) -> u8 {
        // Bit 0 is shifted to carry and bit 7
        self.flags = CARRY.test(get_bit!(value, 0) == 1);
        let res = (self.flags.contains(CARRY) as u8) << 7 | value >> 1;
        self.flags = self.flags | ZERO.test(res == 0);
        res
    }

    #[inline(always)]
    pub fn shift_left_arithmetic(&mut self, value: u8) -> u8 {
        // Bit 7 is shifted to carry
        self.flags = CARRY.test(get_bit!(value, 7) == 1);
        let res = value << 1;
        self.flags = self.flags | ZERO.test(res == 0);
        res
    }

    #[inline(always)]
    pub fn shift_right_arithmetic(&mut self, value: u8) -> u8 {
        // Bit 0 is shifted to carry, bit 7 stays the same
        self.flags = CARRY.test(get_bit!(value, 0) == 1);
        let res = value >> 1;
        self.flags = self.flags | ZERO.test(res == 0);
        res
    }

    #[inline(always)]
    pub fn shift_right_logical(&mut self, value: u8) -> u8 {
        // Bit 0 is shifted to carry, bit 7 is reset
        self.flags = CARRY.test(get_bit!(value, 0) == 1);
        let res = reset_bit!(value >> 1, 7);
        self.flags = self.flags | ZERO.test(res == 0);
        res
    }

}