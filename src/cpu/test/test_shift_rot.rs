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
use instructions::{Instr, Reg8, Immediate, Op};
use super::test_instr;

fn rotate_left_carry_helper(value: u8) {
    let op = Op::rlca;
    let cpu = test_instr(
        Instr { op: op, imm: Immediate::None },
        &[0x00],
        |cpu| {
            cpu.regs.f = Flags::all();
            cpu.regs.write8(Reg8::A, value);
        }
    );
   
    assert_eq!(cpu.last_cycles, 1);
    assert_eq!(cpu.regs.read8(Reg8::A), value.rotate_left(1));
    assert!(!cpu.regs.f.contains(ZERO));
    assert!(!cpu.regs.f.contains(SUB));
    assert!(!cpu.regs.f.contains(HCARRY));
    assert_eq!(cpu.regs.f.contains(CARRY), value & 0x80 != 0);
}

#[test]
fn test_rotate_left_carry() {
    rotate_left_carry_helper(0x0F);
    rotate_left_carry_helper(0x80);
    rotate_left_carry_helper(0xCC);
}

fn rotate_left_helper(value: u8, carry: bool) {
    let op = Op::rla;
    let cpu = test_instr(
        Instr { op: op, imm: Immediate::None },
        &[0x00],
        |cpu| {
            cpu.regs.f = Flags::all();
            cpu.regs.f.force(CARRY, carry);
            cpu.regs.write8(Reg8::A, value);
        }
    );
  
    assert_eq!(cpu.last_cycles, 1);
    assert_eq!(cpu.regs.read8(Reg8::A), (value << 1) + carry as u8);
    assert!(!cpu.regs.f.contains(ZERO));
    assert!(!cpu.regs.f.contains(SUB));
    assert!(!cpu.regs.f.contains(HCARRY));
    assert_eq!(cpu.regs.f.contains(CARRY), value & 0x80 != 0);
}

#[test]
fn test_rotate_left() {
    rotate_left_helper(0x80, true);
    rotate_left_helper(0x80, false);
    rotate_left_helper(0xCC, true);
    rotate_left_helper(0xCC, false);
}

fn rotate_right_carry_helper(value: u8) {
    let op = Op::rrca;
    let cpu = test_instr(
        Instr { op: op, imm: Immediate::None },
        &[0x00],
        |cpu| {
            cpu.regs.f = Flags::all();
            cpu.regs.write8(Reg8::A, value);
        }
    );
   
    assert_eq!(cpu.last_cycles, 1);
    assert_eq!(cpu.regs.read8(Reg8::A), value.rotate_right(1));
    assert!(!cpu.regs.f.contains(ZERO));
    assert!(!cpu.regs.f.contains(SUB));
    assert!(!cpu.regs.f.contains(HCARRY));
    assert_eq!(cpu.regs.f.contains(CARRY), value & 0x01 != 0);
}

#[test]
fn test_rotate_right_carry() {
    rotate_right_carry_helper(0x01);
    rotate_right_carry_helper(0xFE);
}

fn rotate_right_helper(value: u8, carry: bool) {
    let op = Op::rra;
    let cpu = test_instr(
        Instr { op: op, imm: Immediate::None },
        &[0x00],
        |cpu| {
            cpu.regs.f = Flags::all();
            cpu.regs.f.force(CARRY, carry);
            cpu.regs.write8(Reg8::A, value);
        }
    );
   
    assert_eq!(cpu.last_cycles, 1);
    assert_eq!(cpu.regs.read8(Reg8::A), (value >> 1) | ((carry as u8) << 7));
    assert!(!cpu.regs.f.contains(ZERO));
    assert!(!cpu.regs.f.contains(SUB));
    assert!(!cpu.regs.f.contains(HCARRY));
    assert_eq!(cpu.regs.f.contains(CARRY), value & 0x01 != 0);
}

#[test]
fn test_rotate_right() {
    rotate_right_helper(0x01, true);
    rotate_right_helper(0x01, false);
    rotate_right_helper(0xFE, true);
    rotate_right_helper(0xFE, false);
}