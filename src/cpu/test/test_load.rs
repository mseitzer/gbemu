use super::{run_test, test_instr};
use instructions::{Instr, Reg8, Reg16, Immediate, Op};

#[test]
fn test_load8() {
    // LD Reg8, Reg8
    let regs = [Reg8::A, Reg8::B, Reg8::C, Reg8::D, Reg8::E, Reg8::H, Reg8::L];

    for dest in regs.iter() {
        for src in regs.iter() {
            let op = Op::ld8_rr { dest: *dest, src: *src };
            let cpu = test_instr(
                Instr { op: op, imm: Immediate::None },
                &[0x00],
                |cpu| {
                    cpu.regs.write8(*dest, 0x00);
                    cpu.regs.write8(*src, 0x42);
                }
            );

            assert_eq!(cpu.last_cycles, 1);
            assert_eq!(cpu.regs.read8(*dest), 0x42);
            assert_eq!(cpu.regs.read8(*src), 0x42);
        }
    }
}

#[test]
fn test_indirect_load8() {
    // LD Reg8, (HL)
    let regs = [Reg8::A, Reg8::B, Reg8::C, Reg8::D, Reg8::E, Reg8::H, Reg8::L];

    for dest in regs.iter() {
        let op = Op::ld8_ind_reg { dest: *dest, src: Reg16::HL };
        let cpu = test_instr(
            Instr { op: op, imm: Immediate::None },
            &[0x00, 0x42],
            |cpu| {
                cpu.regs.write8(*dest, 0x00);
                cpu.regs.write16(Reg16::HL, 0x0001);
            }
        );

        assert_eq!(cpu.last_cycles, 2);
        assert_eq!(cpu.regs.read8(*dest), 0x42);
        match *dest {
            Reg8::H => assert_eq!(cpu.regs.read16(Reg16::HL), 0x4201),
            Reg8::L => assert_eq!(cpu.regs.read16(Reg16::HL), 0x0042),
            _       => assert_eq!(cpu.regs.read16(Reg16::HL), 0x0001)
        };
    }
}