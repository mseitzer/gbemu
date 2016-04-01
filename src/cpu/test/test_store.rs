use hardware::Bus;
use super::super::Cpu;
use super::{TestHardware, run_test, test_instr};
use instructions::{Instr, Reg8, Reg16, Immediate, Op, Addr};

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