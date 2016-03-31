use cpu::registers::{Flags, ZERO, SUB, HCARRY, CARRY};
use instructions::{Instr, Reg8, Reg16, Immediate, Op};
use super::{run_test, test_instr};

fn inc16(src: Reg16, value: u16) {
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
    let regs = [Reg16::BC, Reg16::DE, Reg16::HL];

    for src in regs.iter() {
        inc16(*src, 0x0004);
        inc16(*src, 0xffff); // Overflow
    }
}