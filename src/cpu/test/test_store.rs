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

fn indirect_store_helper(src: Reg8, reg: Reg16, addr: Addr, addr_value: u16, value: u8) 
    -> Cpu<TestHardware> {

    let op = Op::st8_ind { dest: addr, src: src };
    let cpu = test_instr(
        Instr { op: op, imm: Immediate::None },
        &[0x00, 0xff],
        |cpu| {
            cpu.regs.write8(src, value);
            cpu.regs.write16(reg, addr_value);
        }
    );

    assert_eq!(cpu.last_cycles, 2);
    cpu
}

#[test]
fn test_indirect_store8_hl() {
    // LD (HL), Reg8
    let regs = [Reg8::A, Reg8::B, Reg8::C, Reg8::D, Reg8::E, Reg8::H, Reg8::L];

    for src in regs.iter() {
        let cpu = indirect_store_helper(*src, Reg16::HL, Addr::HL, 0x0001, 0x42);
        match *src {
            Reg8::H => {
                assert_eq!(cpu.bus.read(0x0001), 0x00);
                assert_eq!(cpu.regs.read8(Reg8::H), 0x00);
            },
            Reg8::L => {
                assert_eq!(cpu.bus.read(0x0001), 0x01);
                assert_eq!(cpu.regs.read8(Reg8::L), 0x01);
            }
            _       => {
                assert_eq!(cpu.bus.read(0x0001), 0x42);
                assert_eq!(cpu.regs.read8(*src), 0x42);
            }
        };
        assert_eq!(cpu.regs.read16(Reg16::HL), 0x0001);
    }
}


#[test]
fn test_indirect_store8_bc() {
    // LD (BC), Reg8
    let cpu = indirect_store_helper(Reg8::A, Reg16::BC, Addr::BC, 0x0001, 0x42);
    assert_eq!(cpu.bus.read(0x0001), 0x42);
    assert_eq!(cpu.regs.read8(Reg8::A), 0x42);
    assert_eq!(cpu.regs.read16(Reg16::BC), 0x0001);
}

#[test]
fn test_indirect_store8_de() {
    // LD (DE), Reg8
    let cpu = indirect_store_helper(Reg8::A, Reg16::DE, Addr::DE, 0x0001, 0x42);
    assert_eq!(cpu.bus.read(0x0001), 0x42);
    assert_eq!(cpu.regs.read8(Reg8::A), 0x42);
    assert_eq!(cpu.regs.read16(Reg16::DE), 0x0001);
}

#[test]
fn test_indirect_store8_hli() {
    // LD (HL+), Reg8
    let cpu = indirect_store_helper(Reg8::A, Reg16::HL, Addr::HLI, 0x0001, 0x42);
    assert_eq!(cpu.bus.read(0x0001), 0x42);
    assert_eq!(cpu.regs.read8(Reg8::A), 0x42);
    assert_eq!(cpu.regs.read16(Reg16::HL), 0x0002);
}

#[test]
fn test_indirect_store8_hld() {
    // LD (HL-), Reg8
    let cpu = indirect_store_helper(Reg8::A, Reg16::HL, Addr::HLD, 0x0001, 0x42);
    assert_eq!(cpu.bus.read(0x0001), 0x42);
    assert_eq!(cpu.regs.read8(Reg8::A), 0x42);
    assert_eq!(cpu.regs.read16(Reg16::HL), 0x0000);
}

#[test]
fn test_indirect_immediate_store8() {
    // LD (HL), Imm8
    let op = Op::st8_ind_imm;
    let cpu = test_instr(
        Instr { op: op, imm: Immediate::Imm8(0x66) },
        &[0x00, 0xff],
        |cpu| {
            cpu.regs.write16(Reg16::HL, 0x0001);
        }
    );

    assert_eq!(cpu.last_cycles, 3);
    assert_eq!(cpu.bus.read(0x0001), 0x66);
    assert_eq!(cpu.regs.read16(Reg16::HL), 0x0001);
}

#[test]
fn test_sp_store() {
    // LD (Imm16), SP
    let cpu = test_instr(
        Instr { op: Op::st16_sp, imm: Immediate::Imm16(0x0001) },
        &[0x00, 0xff, 0xaa],
        |cpu| {
            cpu.regs.write16(Reg16::SP, 0xbbdd);
        }
    );
    assert_eq!(cpu.last_cycles, 5);
    assert_eq!(cpu.bus.read(0x0001), 0xdd);
    assert_eq!(cpu.bus.read(0x0002), 0xbb);
    assert_eq!(cpu.regs.read16(Reg16::SP), 0xbbdd);
}