use super::{run_test, test_instr};
use cpu::{CpuFlags, ZERO, SUB, HCARRY, CARRY};
use instructions::{Instr, Reg8, Reg16, Immediate, Op};

fn inc16(src: Reg16, value: u16) {
    let op = Op::inc16_reg { src: src };
    let cpu = test_instr(
        Instr { op: op, imm: Immediate::None },
        &[0x00],
        |cpu| {
            cpu.flags = CpuFlags::all();
            cpu.write_reg16(src, value);
        }
    );

    assert_eq!(cpu.last_cycles, 2);
    assert_eq!(cpu.read_reg16(src), value.wrapping_add(1));
    assert_eq!(cpu.flags, CpuFlags::all());
}

#[test]
fn test_inc16() {
    let regs = [Reg16::BC, Reg16::DE, Reg16::HL];

    for src in regs.iter() {
        inc16(*src, 0x0004);
        inc16(*src, 0xffff); // Overflow
    }
}