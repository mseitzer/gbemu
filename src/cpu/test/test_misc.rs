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
use super::{run_test, test_instr};
use cpu::registers::{ZERO, SUB, HCARRY, CARRY};
use instructions::{Instr, Reg8, Immediate, Op};

#[test]
fn test_nop() {
    // NOP
    let cpu = run_test(
        &[0x00],
        |_| {}
    );

    assert_eq!(cpu.total_cycles, 1);
}

// TODO:
/* fn test_daa() {

}*/

#[test]
fn test_scf() {
    // SCF
    let cpu = test_instr(
        Instr { op: Op::scf, imm: Immediate::None },
        &[0x00],
        |cpu| {
            cpu.regs.f = ZERO | SUB | HCARRY;
        }
    );

    assert_eq!(cpu.last_cycles, 1);
    assert_eq!(cpu.regs.f, ZERO | CARRY);
}

#[test]
fn test_cpl() {
    // CPL
    let cpu = test_instr(
        Instr { op: Op::cpl, imm: Immediate::None },
        &[0x00],
        |cpu| {
            cpu.regs.f = ZERO | CARRY;
            cpu.regs.write8(Reg8::A, 0b10100101);
        }
    );

    assert_eq!(cpu.last_cycles, 1);
    assert_eq!(cpu.regs.f, ZERO | SUB | HCARRY | CARRY);
    assert_eq!(cpu.regs.read8(Reg8::A), 0b01011010);
}

#[test]
fn test_ccf() {
    // CCF
    let cpu = test_instr(
        Instr { op: Op::ccf, imm: Immediate::None },
        &[0x00],
        |cpu| {
            cpu.regs.f = ZERO | SUB | HCARRY | CARRY;
        }
    );

    assert_eq!(cpu.last_cycles, 1);
    assert_eq!(cpu.regs.f, ZERO);
}