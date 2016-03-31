use cpu::Cpu;
use cpu::debug::DebugInfo;
use hardware::Bus;
use int_controller::Interrupt;
use instructions::{Instr, Op, Immediate, Reg8, Reg16};

mod test_misc;
mod test_load;
mod test_store;
mod test_alu;

struct TestHardware {
    memory: Vec<u8>
}

impl Bus for TestHardware {
    fn read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn write(&mut self, addr: u16, value: u8) {
        self.memory[addr as usize] = value;
    }

    fn has_irq(&self) -> bool { false }
    fn ack_irq(&mut self) -> Option<Interrupt> { None }
    fn update(&mut self, cycles: u8) {}
}

fn create_hardware(memory: Vec<u8>) -> TestHardware {
    TestHardware {
        memory: memory
    }
}

fn run_test<F: Fn(&mut Cpu<TestHardware>)->()> (code: &[u8], init: F) 
    -> Cpu<TestHardware> {
    let mut cpu = Cpu::new(create_hardware(code.to_vec()));
    init(&mut cpu);

    let mut db = DebugInfo::new();
    cpu.single_step(&mut db);

    return cpu;
}

fn test_instr<F: Fn(&mut Cpu<TestHardware>)->()> (instr: Instr, memory: &[u8], init: F) 
    -> Cpu<TestHardware> {
    let mut cpu = Cpu::new(create_hardware(memory.to_vec()));
    init(&mut cpu);

    cpu.execute_instr(instr);

    return cpu;
}

#[test]
fn test_opcodes() {
    use instructions::Reg8::*;
    use instructions::Reg16::*;
    use instructions::Immediate::*;

    let mut opcodes = vec![
        (vec![0x00], Op::nop, None),
        (vec![0x01, 0x99, 0x11], Op::ld16_imm { dest: BC }, Imm16(0x1199)),
        (vec![0x02], Op::st8_ind_reg { dest: BC, src: A }, None),
        (vec![0x03], Op::inc16_reg { src: BC }, None),

        (vec![0x11, 0x99, 0x11], Op::ld16_imm { dest: DE }, Imm16(0x1199)),
        (vec![0x12], Op::st8_ind_reg { dest: DE, src: A }, None),
        (vec![0x13], Op::inc16_reg { src: DE }, None),

        (vec![0x21, 0x99, 0x11], Op::ld16_imm { dest: HL }, Imm16(0x1199)),
        (vec![0x22], Op::st8_ind_inc, None),
        (vec![0x23], Op::inc16_reg { src: HL }, None),

        (vec![0x31, 0x99, 0x11], Op::ld16_imm { dest: SP }, Imm16(0x1199)),
        (vec![0x32], Op::st8_ind_dec, None),
        (vec![0x33], Op::inc16_reg { src: SP }, None),

        (vec![0x40], Op::ld8_rr { dest: B, src: B }, None),
        (vec![0x41], Op::ld8_rr { dest: B, src: C }, None),
        (vec![0x42], Op::ld8_rr { dest: B, src: D }, None),
        (vec![0x43], Op::ld8_rr { dest: B, src: E }, None),
        (vec![0x44], Op::ld8_rr { dest: B, src: H }, None),
        (vec![0x45], Op::ld8_rr { dest: B, src: L }, None),
        (vec![0x46], Op::ld8_ind_reg { dest: B, src: HL }, None),
        (vec![0x47], Op::ld8_rr { dest: B, src: A }, None),
        (vec![0x48], Op::ld8_rr { dest: C, src: B }, None),
        (vec![0x49], Op::ld8_rr { dest: C, src: C }, None),
        (vec![0x4A], Op::ld8_rr { dest: C, src: D }, None),
        (vec![0x4B], Op::ld8_rr { dest: C, src: E }, None),
        (vec![0x4C], Op::ld8_rr { dest: C, src: H }, None),
        (vec![0x4D], Op::ld8_rr { dest: C, src: L }, None),
        (vec![0x4E], Op::ld8_ind_reg { dest: C, src: HL }, None),
        (vec![0x4F], Op::ld8_rr { dest: C, src: A }, None),

        (vec![0x50], Op::ld8_rr { dest: D, src: B }, None),
        (vec![0x51], Op::ld8_rr { dest: D, src: C }, None),
        (vec![0x52], Op::ld8_rr { dest: D, src: D }, None),
        (vec![0x53], Op::ld8_rr { dest: D, src: E }, None),
        (vec![0x54], Op::ld8_rr { dest: D, src: H }, None),
        (vec![0x55], Op::ld8_rr { dest: D, src: L }, None),
        (vec![0x56], Op::ld8_ind_reg { dest: D, src: HL }, None),
        (vec![0x57], Op::ld8_rr { dest: D, src: A }, None),
        (vec![0x58], Op::ld8_rr { dest: E, src: B }, None),
        (vec![0x59], Op::ld8_rr { dest: E, src: C }, None),
        (vec![0x5A], Op::ld8_rr { dest: E, src: D }, None),
        (vec![0x5B], Op::ld8_rr { dest: E, src: E }, None),
        (vec![0x5C], Op::ld8_rr { dest: E, src: H }, None),
        (vec![0x5D], Op::ld8_rr { dest: E, src: L }, None),
        (vec![0x5E], Op::ld8_ind_reg { dest: E, src: HL }, None),
        (vec![0x5F], Op::ld8_rr { dest: E, src: A }, None),

        (vec![0x60], Op::ld8_rr { dest: H, src: B }, None),
        (vec![0x61], Op::ld8_rr { dest: H, src: C }, None),
        (vec![0x62], Op::ld8_rr { dest: H, src: D }, None),
        (vec![0x63], Op::ld8_rr { dest: H, src: E }, None),
        (vec![0x64], Op::ld8_rr { dest: H, src: H }, None),
        (vec![0x65], Op::ld8_rr { dest: H, src: L }, None),
        (vec![0x66], Op::ld8_ind_reg { dest: H, src: HL }, None),
        (vec![0x67], Op::ld8_rr { dest: H, src: A }, None),
        (vec![0x68], Op::ld8_rr { dest: L, src: B }, None),
        (vec![0x69], Op::ld8_rr { dest: L, src: C }, None),
        (vec![0x6A], Op::ld8_rr { dest: L, src: D }, None),
        (vec![0x6B], Op::ld8_rr { dest: L, src: E }, None),
        (vec![0x6C], Op::ld8_rr { dest: L, src: H }, None),
        (vec![0x6D], Op::ld8_rr { dest: L, src: L }, None),
        (vec![0x6E], Op::ld8_ind_reg { dest: L, src: HL }, None),
        (vec![0x6F], Op::ld8_rr { dest: L, src: A }, None),

        (vec![0x70], Op::st8_ind_reg { dest: HL, src: B }, None),
        (vec![0x71], Op::st8_ind_reg { dest: HL, src: C }, None),
        (vec![0x72], Op::st8_ind_reg { dest: HL, src: D }, None),
        (vec![0x73], Op::st8_ind_reg { dest: HL, src: E }, None),
        (vec![0x74], Op::st8_ind_reg { dest: HL, src: H }, None),
        (vec![0x75], Op::st8_ind_reg { dest: HL, src: L }, None),
        (vec![0x76], Op::halt, None),
        (vec![0x77], Op::st8_ind_reg { dest: HL, src: A }, None),
        (vec![0x78], Op::ld8_rr { dest: A, src: B }, None),
        (vec![0x79], Op::ld8_rr { dest: A, src: C }, None),
        (vec![0x7A], Op::ld8_rr { dest: A, src: D }, None),
        (vec![0x7B], Op::ld8_rr { dest: A, src: E }, None),
        (vec![0x7C], Op::ld8_rr { dest: A, src: H }, None),
        (vec![0x7D], Op::ld8_rr { dest: A, src: L }, None),
        (vec![0x7E], Op::ld8_ind_reg { dest: A, src: HL }, None),
        (vec![0x7F], Op::ld8_rr { dest: A, src: A }, None),
    ];

    for (code, op, imm) in opcodes {
        let len = code.len();
        let mut cpu = Cpu::new(create_hardware(code));
        let instr = cpu.fetch_instr();
        assert_eq!(instr.op, op);
        assert_eq!(instr.imm, imm);
        assert_eq!(cpu.regs.pc as usize, len);
    }
}
