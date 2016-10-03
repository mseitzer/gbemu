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
use cpu::registers::{Flags, ZERO, SUB, HCARRY, CARRY};
use instructions::{Instr, Reg8, Reg16, Immediate, Op};
use super::test_instr;

fn inc16_helper(src: Reg16, value: u16) {
    let op = Op::inc16_reg { src: src };
    let cpu = test_instr(
        Instr { op: op, imm: Immediate::None },
        &[0x00],
        |cpu| {
            cpu.regs.f = Flags::all();
            cpu.regs.write16(src, value);
        }
    );

    assert_eq!(cpu.last_cycles, 2);
    assert_eq!(cpu.regs.read16(src), value.wrapping_add(1));
    assert_eq!(cpu.regs.f, Flags::all());
}

#[test]
fn test_inc16() {
    // INC Reg16
    let regs = [Reg16::BC, Reg16::DE, Reg16::HL, Reg16::SP];

    for src in regs.iter() {
        inc16_helper(*src, 0x0004);
        inc16_helper(*src, 0xffff); // Test overflow
    }
}

fn tests_inc8(value: u8, actual: u8, flags: Flags) {
    let expected = value.wrapping_add(1);
    assert_eq!(actual, expected);
    assert_eq!(flags.contains(ZERO), expected == 0);
    assert!(!flags.contains(SUB));
    assert_eq!(flags.contains(HCARRY), 
               (value & 0x0f).wrapping_add(1) & 0x10 != 0);
    assert!(flags.contains(CARRY));
}

fn inc8_helper(src: Reg8, value: u8) {
    let op = Op::inc8_reg { src: src };
    let cpu = test_instr(
        Instr { op: op, imm: Immediate::None },
        &[0x00],
        |cpu| {
            cpu.regs.f = Flags::all();
            cpu.regs.write8(src, value);
        }
    );

    
    assert_eq!(cpu.last_cycles, 1);
    tests_inc8(value, cpu.regs.read8(src), cpu.regs.f);
}

#[test]
fn test_inc8() {
    // INC Reg8
    let regs = [Reg8::A, Reg8::B, Reg8::C, Reg8::D, Reg8::E, Reg8::H, Reg8::L];

    for src in regs.iter() {
        inc8_helper(*src, 0x42);
        inc8_helper(*src, 0x0f); // Test hcarry
        inc8_helper(*src, 0xff); // Test overflow
    }
}

fn inc8_ind_helper(src: Reg16, value: u8) {
    let op = Op::inc8_ind;
    let cpu = test_instr(
        Instr { op: op, imm: Immediate::None },
        &[0x00, value],
        |cpu| {
            cpu.regs.f = Flags::all();
            cpu.regs.write16(src, 0x0001);
        }
    );

    assert_eq!(cpu.last_cycles, 3);
    tests_inc8(value, cpu.bus.read(0x0001), cpu.regs.f);
}

#[test]
fn test_inc8_ind() {
    // INC (HL)
    inc8_ind_helper(Reg16::HL, 0x42);
    inc8_ind_helper(Reg16::HL, 0x0f); // Test hcarry
    inc8_ind_helper(Reg16::HL, 0xff); // Test overflow
}