use hardware::Bus;
use super::{run_test, test_instr};
use instructions::{Instr, Reg8, Reg16, Immediate, Op};

#[test]
fn test_indirect_store8() {
    // LD (HL), Reg8
    let regs = [Reg8::A, Reg8::B, Reg8::C, Reg8::D, Reg8::E, Reg8::H, Reg8::L];

    for src in regs.iter() {
        let op = Op::st8_ind_reg { dest: Reg16::HL, src: *src };
        let cpu = test_instr(
            Instr { op: op, imm: Immediate::None },
            &[0x00, 0xff],
            |cpu| {
                cpu.regs.write8(*src, 0x42);
                cpu.regs.write16(Reg16::HL, 0x0001);
            }
        );

        assert_eq!(cpu.last_cycles, 2);
        match *src {
            Reg8::H => assert_eq!(cpu.bus.read(0x0001), 0x00),
            Reg8::L => assert_eq!(cpu.bus.read(0x0001), 0x01),
            _       => assert_eq!(cpu.bus.read(0x0001), 0x42)
        };
        assert_eq!(cpu.regs.read16(Reg16::HL), 0x0001);
    }
}