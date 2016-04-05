use super::{run_test, test_instr};
use cpu::registers::{Flags, ZERO, SUB, HCARRY, CARRY};
use instructions::{Instr, Reg8, Reg16, Immediate, Op};

#[test]
fn test_nop() {
    let cpu = run_test(
        &[0x00],
        |_| {}
    );

    assert_eq!(cpu.total_cycles, 1);
}

// TODO:
/* fn test_daa() {

}*/

#[test]
fn test_scf() {
    let cpu = test_instr(
        Instr { op: Op::scf, imm: Immediate::None },
        &[0x00],
        |cpu| {
            cpu.regs.f = ZERO | SUB | HCARRY;
        }
    );

    assert_eq!(cpu.last_cycles, 1);
    assert_eq!(cpu.regs.f, ZERO | CARRY);
}