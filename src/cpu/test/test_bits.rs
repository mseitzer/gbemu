use hardware::Bus;
use cpu::registers::{Flags, ZERO, SUB, HCARRY, CARRY};
use instructions::{Instr, Reg8, Reg16, Immediate, Op, Addr};
use super::{TestHardware, run_test, test_instr};

#[test]
fn test_bit_reg() {
    // BIT x, Reg8
    let tester = |src, bit, value, empty| {
        let op = Op::bit { src: src, bit: bit };
        let cpu = test_instr(
            Instr { op: op, imm: Immediate::None },
            &[0x00],
            |cpu| {
                cpu.regs.f = if empty { Flags::empty() } else { Flags::all() };
                cpu.regs.write8(src, value);
            }
        );
        
        assert_eq!(cpu.last_cycles, 2);
        assert_eq!(value & (1 << bit) == 0, cpu.regs.f.contains(ZERO));
        assert!(!cpu.regs.f.contains(SUB));
        assert!(cpu.regs.f.contains(HCARRY));
        assert_eq!(empty, !cpu.regs.f.contains(CARRY));
    };

    let regs = [Reg8::A, Reg8::B, Reg8::C, Reg8::D, Reg8::E, Reg8::H, Reg8::L];
    for reg in regs.iter() {
        for bit in 0..8 {
            tester(*reg, bit, 0b11111111, true);
            tester(*reg, bit, 0b11111111, false);
            tester(*reg, bit, 0x00000000, true);
            tester(*reg, bit, 0x00000000, false);
        }
    }
}

#[test]
fn test_bit_ind() {
    // BIT x, (HL)
    let tester = |bit, value, empty| {
        let op = Op::bit_ind { bit: bit };
        let cpu = test_instr(
            Instr { op: op, imm: Immediate::None },
            &[0x00, value],
            |cpu| {
                cpu.regs.f = if empty { Flags::empty() } else { Flags::all() };
                cpu.regs.write16(Reg16::HL, 0x0001);
            }
        );
        
        assert_eq!(cpu.last_cycles, 3);
        assert_eq!(value & (1 << bit) == 0, cpu.regs.f.contains(ZERO));
        assert!(!cpu.regs.f.contains(SUB));
        assert!(cpu.regs.f.contains(HCARRY));
        assert_eq!(empty, !cpu.regs.f.contains(CARRY));
    };

    for bit in 0..8 {
        tester(bit, 0b11111111, true);
        tester(bit, 0b11111111, false);
        tester(bit, 0x00000000, true);
        tester(bit, 0x00000000, false);
    }
}