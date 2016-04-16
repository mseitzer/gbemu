use hardware::Bus;
use cpu::registers::{Flags, ZERO, SUB, HCARRY, CARRY};
use instructions::{Instr, Reg8, Reg16, Immediate, Op, Addr};
use super::{TestHardware, run_test, test_instr};

#[test]
fn test_swap_reg() {
    // SWAP Reg8
    let tester = |src, value| {
        let op = Op::swap { src: src };
        let cpu = test_instr(
            Instr { op: op, imm: Immediate::None },
            &[0x00],
            |cpu| {
                cpu.regs.f = Flags::all();
                cpu.regs.write8(src, value);
            }
        );
        
        let actual = cpu.regs.read8(src);
        assert_eq!(cpu.last_cycles, 2);
        assert_eq!(actual, ((value & 0x0f) << 4) | (value >> 4));
        assert_eq!(actual == 0, cpu.regs.f.contains(ZERO));
        assert!(!cpu.regs.f.contains(SUB));
        assert!(!cpu.regs.f.contains(HCARRY));
        assert!(!cpu.regs.f.contains(CARRY));
    };

    let regs = [Reg8::A, Reg8::B, Reg8::C, Reg8::D, Reg8::E, Reg8::H, Reg8::L];
    for reg in regs.iter() {
        tester(*reg, 0x00);
        tester(*reg, 0x31);
    }
}

#[test]
fn test_swap_ind() {
    // SWAP (HL)
    let tester = |value| {
        let op = Op::swap_ind;
        let cpu = test_instr(
            Instr { op: op, imm: Immediate::None },
            &[0x00, value],
            |cpu| {
                cpu.regs.f = Flags::all();
                cpu.regs.write16(Reg16::HL, 0x0001);
            }
        );
        
        let actual = cpu.bus.read(0x0001);
        assert_eq!(cpu.last_cycles, 4);
        assert_eq!(actual, ((value & 0x0f) << 4) | (value >> 4));
        assert_eq!(actual == 0, cpu.regs.f.contains(ZERO));
        assert!(!cpu.regs.f.contains(SUB));
        assert!(!cpu.regs.f.contains(HCARRY));
        assert!(!cpu.regs.f.contains(CARRY));
    };
    
    tester(0x00);
    tester(0xf5);
}
