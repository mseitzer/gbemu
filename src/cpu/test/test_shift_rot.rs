use hardware::Bus;
use cpu::registers::{Flags, ZERO, SUB, HCARRY, CARRY};
use instructions::{Instr, Reg8, Reg16, Immediate, Op, Addr};
use super::{TestHardware, run_test, test_instr};


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
