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

#[test]
fn test_swap_reg() {
    // SWAP Reg8
    let tester = |src, value| {
        let op = Op::swap { src: src };
        let cpu = test_instr(
            Instr { op: op, imm: Immediate::None },
            &[0x00],
            |cpu| {
                cpu.regs.f = Flags::all();
                cpu.regs.write8(src, value);
            }
        );
        
        let actual = cpu.regs.read8(src);
        assert_eq!(cpu.last_cycles, 2);
        assert_eq!(actual, ((value & 0x0f) << 4) | (value >> 4));
        assert_eq!(actual == 0, cpu.regs.f.contains(ZERO));
        assert!(!cpu.regs.f.contains(SUB));
        assert!(!cpu.regs.f.contains(HCARRY));
        assert!(!cpu.regs.f.contains(CARRY));
    };

    let regs = [Reg8::A, Reg8::B, Reg8::C, Reg8::D, Reg8::E, Reg8::H, Reg8::L];
    for reg in regs.iter() {
        tester(*reg, 0x00);
        tester(*reg, 0x31);
    }
}

#[test]
fn test_swap_ind() {
    // SWAP (HL)
    let tester = |value| {
        let op = Op::swap_ind;
        let cpu = test_instr(
            Instr { op: op, imm: Immediate::None },
            &[0x00, value],
            |cpu| {
                cpu.regs.f = Flags::all();
                cpu.regs.write16(Reg16::HL, 0x0001);
            }
        );
        
        let actual = cpu.bus.read(0x0001);
        assert_eq!(cpu.last_cycles, 4);
        assert_eq!(actual, ((value & 0x0f) << 4) | (value >> 4));
        assert_eq!(actual == 0, cpu.regs.f.contains(ZERO));
        assert!(!cpu.regs.f.contains(SUB));
        assert!(!cpu.regs.f.contains(HCARRY));
        assert!(!cpu.regs.f.contains(CARRY));
    };
    
    tester(0x00);
    tester(0xf5);
}
