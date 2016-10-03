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
use cpu::registers::{Flags, ZERO, SUB, HCARRY, CARRY};
use instructions::{Instr, Reg16, Immediate, Op};
use super::test_instr;

fn add16_helper(src: Reg16, value1: u16, value2: u16) {
    let op = Op::add16_reg { src: src };
    let cpu = test_instr(
        Instr { op: op, imm: Immediate::None },
        &[0x00],
        |cpu| {
            cpu.regs.f = Flags::all();
            cpu.regs.write16(Reg16::HL, value1);
            cpu.regs.write16(src, value2);
        }
    );
    
    let actual = cpu.regs.read16(Reg16::HL);
    assert_eq!(cpu.last_cycles, 2);
    assert_eq!(actual, value1.wrapping_add(value2));
    assert!(cpu.regs.f.contains(ZERO));
    assert!(!cpu.regs.f.contains(SUB));
    assert_eq!(cpu.regs.f.contains(HCARRY), 
               (value1 & 0x0fff).wrapping_add(value2 & 0x0fff) & 0x1000 != 0);
    assert_eq!(cpu.regs.f.contains(CARRY), value1.overflowing_add(value2).1);
}

#[test]
fn test_add16() {
    // ADD HL, Reg16
    let regs = [Reg16::BC, Reg16::DE, Reg16::SP];

    for src in regs.iter() {
        add16_helper(*src, 0xfff0, 0x0010); // Test overflow
        add16_helper(*src, 0x0fff, 0x0001); // Test hcarry
    }

    add16_helper(Reg16::HL, 0xfff0, 0xfff0); // Test overflow
    add16_helper(Reg16::HL, 0x0fff, 0x0fff); // Test hcarry
}
