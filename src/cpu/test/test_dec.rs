use hardware::Bus;
use cpu::registers::{Flags, ZERO, SUB, HCARRY, CARRY};
use instructions::{Instr, Reg8, Reg16, Immediate, Op,};
use super::test_instr;

fn dec16_helper(src: Reg16, value: u16) {
    let op = Op::dec16_reg { src: src };
    let cpu = test_instr(
        Instr { op: op, imm: Immediate::None },
        &[0x00],
        |cpu| {
            cpu.regs.f = Flags::all();
            cpu.regs.write16(src, value);
        }
    );

    assert_eq!(cpu.last_cycles, 2);
    assert_eq!(cpu.regs.read16(src), value.wrapping_sub(1));
    assert_eq!(cpu.regs.f, Flags::all());
}

#[test]
fn test_dec16() {
    // DEC Reg16
    let regs = [Reg16::BC, Reg16::DE, Reg16::HL, Reg16::SP];

    for src in regs.iter() {
        dec16_helper(*src, 0x0001); // Test zero
        dec16_helper(*src, 0x0000); // Test overflow
        dec16_helper(*src, 0xffff);
    }
}

fn tests_dec8(value: u8, actual: u8, flags: Flags) {
    let expected = value.wrapping_sub(1);
    assert_eq!(actual, expected);
    assert_eq!(flags.contains(ZERO), expected == 0);
    assert!(flags.contains(SUB));
    assert_eq!(flags.contains(HCARRY), 
               (value & 0x0f).wrapping_sub(1) & 0x10 != 0);
    assert!(flags.contains(CARRY));
}

fn dec8_helper(src: Reg8, value: u8) {
    let op = Op::dec8_reg { src: src };
    let cpu = test_instr(
        Instr { op: op, imm: Immediate::None },
        &[0x00],
        |cpu| {
            cpu.regs.f = Flags::all();
            cpu.regs.write8(src, value);
        }
    );

    
    assert_eq!(cpu.last_cycles, 1);
    tests_dec8(value, cpu.regs.read8(src), cpu.regs.f);
}

#[test]
fn test_dec8() {
    // DEC Reg8
    let regs = [Reg8::A, Reg8::B, Reg8::C, Reg8::D, Reg8::E, Reg8::H, Reg8::L];

    for src in regs.iter() {
        dec8_helper(*src, 0x01); // Test zero
        dec8_helper(*src, 0x00); // Test overflow
        dec8_helper(*src, 0xf0); // Test hcarry
    }
}

fn dec8_ind_helper(src: Reg16, value: u8) {
    let op = Op::dec8_ind;
    let cpu = test_instr(
        Instr { op: op, imm: Immediate::None },
        &[0x00, value],
        |cpu| {
            cpu.regs.f = Flags::all();
            cpu.regs.write16(src, 0x0001);
        }
    );

    assert_eq!(cpu.last_cycles, 3);
    tests_dec8(value, cpu.bus.read(0x0001), cpu.regs.f);
}

#[test]
fn test_dec8_ind() {
    // DEC (HL)
    dec8_ind_helper(Reg16::HL, 0x01); // Test zero
    dec8_ind_helper(Reg16::HL, 0xf0); // Test hcarry
    dec8_ind_helper(Reg16::HL, 0x00); // Test overflow
}