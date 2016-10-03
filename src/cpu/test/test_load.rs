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
use instructions::{Instr, Reg8, Reg16, Immediate, Op, Addr};
use super::super::Cpu;
use super::{TestHardware, test_instr};

#[test]
fn test_immediate_load8() {
    // LD Reg8, imm8
    let regs = [Reg8::A, Reg8::B, Reg8::C, Reg8::D, Reg8::E, Reg8::H, Reg8::L];

    for dest in regs.iter() {
        let op = Op::ld8_imm { dest: *dest };
        let cpu = test_instr(
            Instr { op: op, imm: Immediate::Imm8(0x33) },
            &[0x00],
            |cpu| {
                cpu.regs.write8(*dest, 0x00);
            }
        );

        assert_eq!(cpu.last_cycles, 2);
        assert_eq!(cpu.regs.read8(*dest), 0x33);
    }
}

#[test]
fn test_register_load8() {
    // LD Reg8, Reg8
    let regs = [Reg8::A, Reg8::B, Reg8::C, Reg8::D, Reg8::E, Reg8::H, Reg8::L];

    for dest in regs.iter() {
        for src in regs.iter() {
            let op = Op::ld8_rr { dest: *dest, src: *src };
            let cpu = test_instr(
                Instr { op: op, imm: Immediate::None },
                &[0x00],
                |cpu| {
                    cpu.regs.write8(*dest, 0x00);
                    cpu.regs.write8(*src, 0x42);
                }
            );

            assert_eq!(cpu.last_cycles, 1);
            assert_eq!(cpu.regs.read8(*dest), 0x42);
            assert_eq!(cpu.regs.read8(*src), 0x42);
        }
    }
}

#[test]
fn test_indirect_load8_hl() {
    // LD Reg8, (HL)
    let regs = [Reg8::A, Reg8::B, Reg8::C, Reg8::D, Reg8::E, Reg8::H, Reg8::L];

    for dest in regs.iter() {
        let op = Op::ld8_ind { dest: *dest, src: Addr::HL };
        let cpu = test_instr(
            Instr { op: op, imm: Immediate::None },
            &[0x00, 0x42],
            |cpu| {
                cpu.regs.write8(*dest, 0x00);
                cpu.regs.write16(Reg16::HL, 0x0001);
            }
        );

        assert_eq!(cpu.last_cycles, 2);
        assert_eq!(cpu.regs.read8(*dest), 0x42);
        match *dest {
            Reg8::H => assert_eq!(cpu.regs.read16(Reg16::HL), 0x4201),
            Reg8::L => assert_eq!(cpu.regs.read16(Reg16::HL), 0x0042),
            _       => assert_eq!(cpu.regs.read16(Reg16::HL), 0x0001)
        };
    }
}

fn indirect_load_helper(dest: Reg8, addr: Addr, src: Reg16) -> Cpu<TestHardware> {
    let op = Op::ld8_ind { dest: dest, src: addr};
    let cpu = test_instr(
        Instr { op: op, imm: Immediate::None },
        &[0x00, 0x42],
        |cpu| {
            cpu.regs.write8(dest, 0x00);
            cpu.regs.write16(src, 0x0001);
        }
    );
    assert_eq!(cpu.last_cycles, 2);
    cpu
}

#[test]
fn test_indirect_load8_bc() {
    // LD A, (BC)
    let cpu = indirect_load_helper(Reg8::A, Addr::BC, Reg16::BC);
    assert_eq!(cpu.bus.read(0x0001), 0x42);
    assert_eq!(cpu.regs.read8(Reg8::A), 0x42);
    assert_eq!(cpu.regs.read16(Reg16::BC), 0x0001);
}

#[test]
fn test_indirect_load8_de() {
    // LD A, (DE)
    let cpu = indirect_load_helper(Reg8::A, Addr::DE, Reg16::DE);
    assert_eq!(cpu.bus.read(0x0001), 0x42);
    assert_eq!(cpu.regs.read8(Reg8::A), 0x42);
    assert_eq!(cpu.regs.read16(Reg16::DE), 0x0001);
}

#[test]
fn test_indirect_load8_hli() {
    // LD A, (HL+)
    let cpu = indirect_load_helper(Reg8::A, Addr::HLI, Reg16::HL);
    assert_eq!(cpu.bus.read(0x0001), 0x42);
    assert_eq!(cpu.regs.read8(Reg8::A), 0x42);
    assert_eq!(cpu.regs.read16(Reg16::HL), 0x0002);
}

#[test]
fn test_indirect_load8_hld() {
    // LD A, (HL-)
    let cpu = indirect_load_helper(Reg8::A, Addr::HLD, Reg16::HL);
    assert_eq!(cpu.bus.read(0x0001), 0x42);
    assert_eq!(cpu.regs.read8(Reg8::A), 0x42);
    assert_eq!(cpu.regs.read16(Reg16::HL), 0x0000);
}

#[test]
fn test_immediate_load16() {
    // LD Reg16, Imm16
    let regs = [Reg16::BC, Reg16::DE, Reg16::HL, Reg16::SP];

    for dest in regs.iter() {
        let op = Op::ld16_imm { dest: *dest };
        let cpu = test_instr(
            Instr { op: op, imm: Immediate::Imm16(0x4488) },
            &[0x00],
            |cpu| {
                cpu.regs.write16(*dest, 0x3333);
            }
        );

        assert_eq!(cpu.last_cycles, 3);
        assert_eq!(cpu.regs.read16(*dest), 0x4488);
    }
}
