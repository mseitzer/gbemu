use hardware::Bus;
use cpu::registers::{Flags, ZERO, SUB, HCARRY, CARRY};
use instructions::{Instr, Reg8, Reg16, Immediate, Op, Addr, Condition};
use super::{TestHardware, run_test, test_instr};

fn cmp_cond_flags(cond: Condition, flags: Flags) -> bool {
    match cond {
        Condition::NZ => !flags.contains(ZERO),
        Condition::NC => !flags.contains(CARRY),
        Condition::Z => flags.contains(ZERO),
        Condition::C => flags.contains(CARRY),
    }
}

#[test]
fn test_jp_rel() {
    // JR Reg8
    let op = Op::jp_rel;
    let cpu = test_instr(
        Instr { op: op, imm: Immediate::Imm8(0x42) },
        &[0x00],
        |cpu| {
            cpu.regs.pc = 0x100;
        }
    );

    assert_eq!(cpu.last_cycles, 3);
    assert_eq!(cpu.regs.pc, 0x142);
}

#[test]
fn test_jp_rel_neg() {
    // JR Reg8
    let op = Op::jp_rel;
    let cpu = test_instr(
        Instr { op: op, imm: Immediate::Imm8(0xf0) },
        &[0x00],
        |cpu| {
            cpu.regs.pc = 42;
        }
    );

    assert_eq!(cpu.last_cycles, 3);
    assert_eq!(cpu.regs.pc, 26);
}

fn jp_rel_cond_helper(pc: u16, ofs: i8, cond: Condition, flags: Flags) {
    let op = Op::jp_rel_cond { cond: cond };
    let cpu = test_instr(
        Instr { op: op, imm: Immediate::Imm8(ofs as u8) },
        &[0x00],
        |cpu| {
            cpu.regs.f = flags;
            cpu.regs.pc = pc;
        }
    );

    if cmp_cond_flags(cond, flags) {
        assert_eq!(cpu.last_cycles, 3);
        assert_eq!(cpu.regs.pc, (pc as i32 + ofs as i32) as u16);
    } else {
        assert_eq!(cpu.last_cycles, 2);
        assert_eq!(cpu.regs.pc, pc);
    }
}

#[test]
fn test_jp_rel_cond() {
    let conditions = [
        (Condition::NZ, Flags::empty(), ZERO),
        (Condition::NC, Flags::empty(), CARRY),
        (Condition::Z, ZERO, Flags::empty()),
        (Condition::C, CARRY, Flags::empty()),
    ];

    for &(cond, flag_true, flag_false) in conditions.iter() {
        jp_rel_cond_helper(0x1000, 0x42, cond, flag_false);
        jp_rel_cond_helper(0x2000, 0x42, cond, flag_true);
        jp_rel_cond_helper(0x3000, -0x42, cond, flag_false);
        jp_rel_cond_helper(0x4000, -0x42, cond, flag_true);
    }
}