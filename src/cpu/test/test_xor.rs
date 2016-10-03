use cpu::registers::{Flags, ZERO, SUB, HCARRY, CARRY};
use instructions::{Instr, Reg8, Reg16, Immediate, Op};
use super::test_instr;

#[test]
fn test_xor8_reg() {
    // XOR Reg8
    let tester = |src, value1, value2| {
        let op = Op::xor8_reg { src: src };
        let cpu = test_instr(
            Instr { op: op, imm: Immediate::None },
            &[0x00],
            |cpu| {
                cpu.regs.f = Flags::all();
                cpu.regs.write8(src, value2);
                cpu.regs.write8(Reg8::A, value1);
            }
        );
        
        let actual = cpu.regs.read8(Reg8::A);
        assert_eq!(cpu.last_cycles, 1);
        if let Reg8::A = src {
            assert_eq!(actual, value1 ^ value1);
        } else {
            assert_eq!(actual, value1 ^ value2);
        }
        assert_eq!(actual == 0, cpu.regs.f.contains(ZERO));
        assert!(!cpu.regs.f.contains(SUB));
        assert!(!cpu.regs.f.contains(HCARRY));
        assert!(!cpu.regs.f.contains(CARRY));
    };

    let regs = [Reg8::A, Reg8::B, Reg8::C, Reg8::D, Reg8::E, Reg8::H, Reg8::L];
    for reg in regs.iter() {
        tester(*reg, 0xff, 0xff);
        tester(*reg, 0x12, 0xe2);
    }
}

#[test]
fn test_xor8_ind() {
    // XOR (HL)
    let tester = |value1, value2| {
        let op = Op::xor8_ind;
        let cpu = test_instr(
            Instr { op: op, imm: Immediate::None },
            &[0x00, value2],
            |cpu| {
                cpu.regs.f = Flags::all();
                cpu.regs.write8(Reg8::A, value1);
                cpu.regs.write16(Reg16::HL, 0x01);
            }
        );
        
        let actual = cpu.regs.read8(Reg8::A);
        assert_eq!(cpu.last_cycles, 2);
        assert_eq!(actual, value1 ^ value2);
        assert_eq!(actual == 0, cpu.regs.f.contains(ZERO));
        assert!(!cpu.regs.f.contains(SUB));
        assert!(!cpu.regs.f.contains(HCARRY));
        assert!(!cpu.regs.f.contains(CARRY));
    };
    
    tester(0xff, 0xff);
    tester(0xf5, 0x43);
}

#[test]
fn test_xor8_imm() {
    // XOR Imm8
    let tester = |value1, value2| {
        let op = Op::xor8_imm;
        let cpu = test_instr(
            Instr { op: op, imm: Immediate::Imm8(value2) },
            &[0x00],
            |cpu| {
                cpu.regs.f = Flags::all();
                cpu.regs.write8(Reg8::A, value1);
            }
        );
        
        let actual = cpu.regs.read8(Reg8::A);
        assert_eq!(cpu.last_cycles, 2);
        assert_eq!(actual, value1 ^ value2);
        assert_eq!(actual == 0, cpu.regs.f.contains(ZERO));
        assert!(!cpu.regs.f.contains(SUB));
        assert!(!cpu.regs.f.contains(HCARRY));
        assert!(!cpu.regs.f.contains(CARRY));
    };
    
    tester(0xff, 0xff);
    tester(0x65, 0x44);
}
