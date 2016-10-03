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
use instructions::{Instr, Reg8, Reg16, Immediate, Op};
use super::test_instr;

#[test]
fn test_or8_reg() {
    // OR Reg8
    let tester = |src, value1, value2| {
        let op = Op::or8_reg { src: src };
        let cpu = test_instr(
            Instr { op: op, imm: Immediate::None },
            &[0x00],
            |cpu| {
                cpu.regs.f = Flags::all();
                cpu.regs.write8(src, value2);
                cpu.regs.write8(Reg8::A, value1);
            }
        );
        
        let actual = cpu.regs.read8(Reg8::A);
        assert_eq!(cpu.last_cycles, 1);
        if let Reg8::A = src {
            assert_eq!(actual, value1 | value1);
        } else {
            assert_eq!(actual, value1 | value2);
        }
        assert_eq!(actual == 0, cpu.regs.f.contains(ZERO));
        assert!(!cpu.regs.f.contains(SUB));
        assert!(!cpu.regs.f.contains(HCARRY));
        assert!(!cpu.regs.f.contains(CARRY));
    };

    let regs = [Reg8::A, Reg8::B, Reg8::C, Reg8::D, Reg8::E, Reg8::H, Reg8::L];
    for reg in regs.iter() {
        tester(*reg, 0x00, 0x00);
        tester(*reg, 0x11, 0x22);
    }
}

#[test]
fn test_or8_ind() {
    // OR (HL)
    let tester = |value1, value2| {
        let op = Op::or8_ind;
        let cpu = test_instr(
            Instr { op: op, imm: Immediate::None },
            &[0x00, value2],
            |cpu| {
                cpu.regs.f = Flags::all();
                cpu.regs.write8(Reg8::A, value1);
                cpu.regs.write16(Reg16::HL, 0x01);
            }
        );
        
        let actual = cpu.regs.read8(Reg8::A);
        assert_eq!(cpu.last_cycles, 2);
        assert_eq!(actual, value1 | value2);
        assert_eq!(actual == 0, cpu.regs.f.contains(ZERO));
        assert!(!cpu.regs.f.contains(SUB));
        assert!(!cpu.regs.f.contains(HCARRY));
        assert!(!cpu.regs.f.contains(CARRY));
    };
    
    tester(0x00, 0x00);
    tester(0xf5, 0x43);
}

#[test]
fn test_or8_imm() {
    // OR Imm8
    let tester = |value1, value2| {
        let op = Op::or8_imm;
        let cpu = test_instr(
            Instr { op: op, imm: Immediate::Imm8(value2) },
            &[0x00],
            |cpu| {
                cpu.regs.f = Flags::all();
                cpu.regs.write8(Reg8::A, value1);
            }
        );
        
        let actual = cpu.regs.read8(Reg8::A);
        assert_eq!(cpu.last_cycles, 2);
        assert_eq!(actual, value1 | value2);
        assert_eq!(actual == 0, cpu.regs.f.contains(ZERO));
        assert!(!cpu.regs.f.contains(SUB));
        assert!(!cpu.regs.f.contains(HCARRY));
        assert!(!cpu.regs.f.contains(CARRY));
    };
    
    tester(0x00, 0x00);
    tester(0x65, 0x44);
}
