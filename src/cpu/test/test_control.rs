use hardware::Bus;
use cpu::IntEnable;
use cpu::registers::{Flags, ZERO, CARRY};
use instructions::{Instr, Reg16, Immediate, Op, Condition};
use super::test_instr;

fn conditions() -> [(Condition, Flags, Flags); 4] {
    [(Condition::NZ, Flags::empty(), ZERO),
    (Condition::NC, Flags::empty(), CARRY),
    (Condition::Z, ZERO, Flags::empty()),
    (Condition::C, CARRY, Flags::empty())]
} 


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
    // JR Imm8
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
    // JR Imm8
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
    // JR Cond, Imm8
    for &(cond, flag_true, flag_false) in conditions().iter() {
        jp_rel_cond_helper(0x1000, 0x42, cond, flag_false);
        jp_rel_cond_helper(0x2000, 0x42, cond, flag_true);
        jp_rel_cond_helper(0x3000, -0x42, cond, flag_false);
        jp_rel_cond_helper(0x4000, -0x42, cond, flag_true);
    }
}

#[test]
fn test_jp() {
    // JP Imm16
    let op = Op::jp;
    let cpu = test_instr(
        Instr { op: op, imm: Immediate::Imm16(0x4200) },
        &[0x00],
        |cpu| {
            cpu.regs.pc = 0x30;
        }
    );

    assert_eq!(cpu.last_cycles, 4);
    assert_eq!(cpu.regs.pc, 0x4200);
}

fn jp_cond_helper(pc: u16, target: u16, cond: Condition, flags: Flags) {
    let op = Op::jp_cond { cond: cond };
    let cpu = test_instr(
        Instr { op: op, imm: Immediate::Imm16(target) },
        &[0x00],
        |cpu| {
            cpu.regs.f = flags;
            cpu.regs.pc = pc;
        }
    );

    if cmp_cond_flags(cond, flags) {
        assert_eq!(cpu.last_cycles, 4);
        assert_eq!(cpu.regs.pc, target);
    } else {
        assert_eq!(cpu.last_cycles, 3);
        assert_eq!(cpu.regs.pc, pc);
    }
}

#[test]
fn test_jp_cond() {
    // JP Cond, Imm16
    for &(cond, flag_true, flag_false) in conditions().iter() {
        jp_cond_helper(0x1000, 0x4444, cond, flag_false);
        jp_cond_helper(0x2000, 0xffff, cond, flag_true);
    }
}

#[test]
fn test_jp_indirect() {
    // JP (HL)
    let cpu = test_instr(
        Instr { op: Op::jp_ind, imm: Immediate::None},
        &[0x00],
        |cpu| {
            cpu.regs.pc = 0x333;
            cpu.regs.write16(Reg16::HL, 0x5555);
        }
    );

    assert_eq!(cpu.last_cycles, 1);
    assert_eq!(cpu.regs.pc, 0x5555);
}

#[test]
fn test_call() {
    // CALL Imm16
    let cpu = test_instr(
        Instr { op: Op::call, imm: Immediate::Imm16(0x3344) },
        &[0x00, 0x00, 0x00, 0x00],
        |cpu| {
            cpu.regs.pc = 0x0211;
            cpu.regs.sp = 0x0003;
        }
    );

    assert_eq!(cpu.last_cycles, 6);
    assert_eq!(cpu.regs.pc, 0x3344);
    assert_eq!(cpu.regs.sp, 0x0001);
    assert_eq!(cpu.bus.read(0x0001), 0x11);
    assert_eq!(cpu.bus.read(0x0002), 0x02);
}

fn call_cond_helper(pc: u16, target: u16, cond: Condition, flags: Flags) {
    let op = Op::call_cond { cond: cond };
    let cpu = test_instr(
        Instr { op: op, imm: Immediate::Imm16(target) },
        &[0x00, 0x00, 0x00, 0x00],
        |cpu| {
            cpu.regs.f = flags;
            cpu.regs.pc = pc;
            cpu.regs.sp = 0x0003;
        }
    );

    if cmp_cond_flags(cond, flags) {
        assert_eq!(cpu.last_cycles, 6);
        assert_eq!(cpu.regs.pc, target);
        assert_eq!(cpu.regs.sp, 1);
        assert_eq!(cpu.bus.read(0x0001), (pc & 0xff) as u8);
        assert_eq!(cpu.bus.read(0x0002), (pc >> 8) as u8);
    } else {
        assert_eq!(cpu.last_cycles, 3);
        assert_eq!(cpu.regs.pc, pc);
        assert_eq!(cpu.regs.sp, 3);
        assert_eq!(cpu.bus.read(0x0001), 0x00);
        assert_eq!(cpu.bus.read(0x0002), 0x00);
    }
}

#[test]
fn test_call_cond() {
    // CALL Cond, Imm16
    for &(cond, flag_true, flag_false) in conditions().iter() {
        call_cond_helper(0x10ff, 0x444, cond, flag_false);
        call_cond_helper(0x2010, 0xffee, cond, flag_true);
    }
}

#[test]
fn test_ret() {
    // RET
    let cpu = test_instr(
        Instr { op: Op::ret, imm: Immediate::None },
        &[0x00, 0xaa, 0xbb, 0x00],
        |cpu| {
            cpu.regs.pc = 0x0211;
            cpu.regs.sp = 0x0001;
        }
    );

    assert_eq!(cpu.last_cycles, 4);
    assert_eq!(cpu.regs.pc, 0xbbaa);
    assert_eq!(cpu.regs.sp, 0x0003);
}

#[test]
fn test_reti() {
    // RETI
    let cpu = test_instr(
        Instr { op: Op::reti, imm: Immediate::None },
        &[0x00, 0xbb, 0xcc, 0x00],
        |cpu| {
            cpu.regs.pc = 0x5511;
            cpu.regs.sp = 0x0001;
            cpu.int_enable = IntEnable::Pending;
        }
    );

    assert_eq!(cpu.last_cycles, 4);
    assert_eq!(cpu.regs.pc, 0xccbb);
    assert_eq!(cpu.regs.sp, 0x0003);
    assert_eq!(cpu.int_enable, IntEnable::No);
    assert_eq!(cpu.int_flag, true);
}

fn ret_cond_helper(pc: u16, target: u16, cond: Condition, flags: Flags) {
    let op = Op::ret_cond { cond: cond };
    let cpu = test_instr(
        Instr { op: op, imm: Immediate::Imm16(target) },
        &[0x00, target as u8, (target >> 8) as u8, 0x00],
        |cpu| {
            cpu.regs.f = flags;
            cpu.regs.pc = pc;
            cpu.regs.sp = 0x0001;
        }
    );

    if cmp_cond_flags(cond, flags) {
        assert_eq!(cpu.last_cycles, 5);
        assert_eq!(cpu.regs.pc, target);
        assert_eq!(cpu.regs.sp, 3);
    } else {
        assert_eq!(cpu.last_cycles, 2);
        assert_eq!(cpu.regs.pc, pc);
        assert_eq!(cpu.regs.sp, 1);
    }
}

#[test]
fn test_ret_cond() {
    // RET Cond
    for &(cond, flag_true, flag_false) in conditions().iter() {
        ret_cond_helper(0x10fa, 0x344, cond, flag_false);
        ret_cond_helper(0x2b15, 0xf4e9, cond, flag_true);
    }
}

#[test]
fn test_reset() {
    // RST Target
    let targets: [u16; 8] = [0x00, 0x08, 0x10, 0x18, 0x20, 0x28, 0x30, 0x38];

    for target in targets.iter() {
        let cpu = test_instr(
            Instr { op: Op::rst { target: *target }, imm: Immediate::None },
            &[0x00, 0x00, 0x00, 0x00],
            |cpu| {
                cpu.regs.pc = 0x5214;
                cpu.regs.sp = 0x0003;
            }
        );

        assert_eq!(cpu.last_cycles, 4);
        assert_eq!(cpu.regs.pc, *target as u16);
        assert_eq!(cpu.regs.sp, 0x0001);
        assert_eq!(cpu.bus.read(0x0001), 0x14);
        assert_eq!(cpu.bus.read(0x0002), 0x52);
    }
}