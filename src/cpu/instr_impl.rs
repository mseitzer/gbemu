// This file is part of GBEmu.
// Copyright (C) 2016 Max Seitzer <contact@max-seitzer.de>
//
// GBEmu is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// GBEmu is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with GBEmu.  If not, see <http://www.gnu.org/licenses/>.
use hardware::Bus;
use super::registers::{ZERO, CARRY};

impl<B> super::Cpu<B> where B: Bus {
    #[inline(always)]
    pub fn push(&mut self, value: u16) {
        self.regs.sp = self.regs.sp.wrapping_sub(2);
        let addr = self.regs.sp;
        self.write_word(addr, value);
    }

    #[inline(always)]
    pub fn pop(&mut self) -> u16 {
        let addr = self.regs.sp;
        let value = self.read_word(addr);
        self.regs.sp = self.regs.sp.wrapping_add(2);
        return value;
    }

    #[inline(always)]
    pub fn rotate_left(&mut self, value: u8) -> u8 {
        // Bit 7 is shifted to carry, carry is shifted to bit 0
        let old_carry = self.regs.f.contains(CARRY);
        self.regs.f = CARRY.test(get_bit!(value, 7) == 1);
        let res = value << 1 | (old_carry as u8);
        self.regs.f.force(ZERO, res == 0);
        res
    }

    #[inline(always)]
    pub fn rotate_left_carry(&mut self, value: u8) -> u8 {
        // Bit 7 is shifted to carry and bit 0
        self.regs.f = CARRY.test(get_bit!(value, 7) == 1);
        let res = value << 1 | (self.regs.f.contains(CARRY) as u8);
        self.regs.f.force(ZERO, res == 0);
        res
    }

    #[inline(always)]
    pub fn rotate_right(&mut self, value: u8) -> u8 {
        // Bit 0 is shifted to carry, carry is shifted to bit 7
        let old_carry = self.regs.f.contains(CARRY);
        self.regs.f = CARRY.test(get_bit!(value, 0) == 1);
        let res = (old_carry as u8) << 7 | value >> 1;
        self.regs.f.force(ZERO, res == 0);
        res
    }

    #[inline(always)]
    pub fn rotate_right_carry(&mut self, value: u8) -> u8 {
        // Bit 0 is shifted to carry and bit 7
        self.regs.f = CARRY.test(get_bit!(value, 0) == 1);
        let res = (self.regs.f.contains(CARRY) as u8) << 7 | value >> 1;
        self.regs.f.force(ZERO, res == 0);
        res
    }

    #[inline(always)]
    pub fn shift_left_arithmetic(&mut self, value: u8) -> u8 {
        // Bit 7 is shifted to carry
        self.regs.f = CARRY.test(get_bit!(value, 7) == 1);
        let res = value << 1;
        self.regs.f.force(ZERO, res == 0);
        res
    }

    #[inline(always)]
    pub fn shift_right_arithmetic(&mut self, value: u8) -> u8 {
        // Bit 0 is shifted to carry, bit 7 stays the same
        self.regs.f = CARRY.test(get_bit!(value, 0) == 1);
        let res = value >> 1;
        self.regs.f.force(ZERO, res == 0);
        res
    }

    #[inline(always)]
    pub fn shift_right_logical(&mut self, value: u8) -> u8 {
        // Bit 0 is shifted to carry, bit 7 is reset
        self.regs.f = CARRY.test(get_bit!(value, 0) == 1);
        let res = reset_bit!(value >> 1, 7);
        self.regs.f.force(ZERO, res == 0);
        res
    }

}