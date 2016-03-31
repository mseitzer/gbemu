use super::{run_test, test_instr};
use instructions::{Instr, Reg8, Reg16, Immediate, Op};

#[test]
fn test_nop() {
    let cpu = run_test(
        &[0x00], // NOP
        |_| {}
    );

    assert_eq!(cpu.total_cycles, 1);
}