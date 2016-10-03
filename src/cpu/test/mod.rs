use cpu::Cpu;
use cpu::debug::DebugInfo;
use hardware::Bus;
use int_controller::Interrupt;
use instructions::{Instr, Op, Addr, Condition};
use events::Events;

mod test_misc;
mod test_load;
mod test_store;
mod test_add;
mod test_and;
mod test_xor;
mod test_or;
mod test_inc;
mod test_dec;
mod test_shift_rot;
mod test_control;
mod test_swap;
mod test_bits;

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
    fn update(&mut self, _: u8) -> Events { Events::empty() }
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

    let opcodes = vec![
        (vec![0x00], Op::nop, None),
        (vec![0x01, 0x99, 0x11], Op::ld16_imm { dest: BC }, Imm16(0x1199)),
        (vec![0x02], Op::st8_ind { dest: Addr::BC, src: A }, None),
        (vec![0x03], Op::inc16_reg { src: BC }, None),
        (vec![0x04], Op::inc8_reg { src: B }, None),
        (vec![0x05], Op::dec8_reg { src: B }, None),
        (vec![0x06, 0xff], Op::ld8_imm { dest: B }, Imm8(0xff)),
        (vec![0x07], Op::rlca, None),
        (vec![0x08, 0xaa, 0xbb], Op::st16_sp, Imm16(0xbbaa)),
        (vec![0x09], Op::add16_reg { src: BC }, None),
        (vec![0x0A], Op::ld8_ind { dest: A, src: Addr::BC }, None),
        (vec![0x0B], Op::dec16_reg { src: BC }, None),
        (vec![0x0C], Op::inc8_reg { src: C }, None),
        (vec![0x0D], Op::dec8_reg { src: C }, None),
        (vec![0x0E, 0xff], Op::ld8_imm { dest: C }, Imm8(0xff)),
        (vec![0x0F], Op::rrca, None),

        (vec![0x10], Op::stop, None),
        (vec![0x11, 0x99, 0x11], Op::ld16_imm { dest: DE }, Imm16(0x1199)),
        (vec![0x12], Op::st8_ind { dest: Addr::DE, src: A }, None),
        (vec![0x13], Op::inc16_reg { src: DE }, None),
        (vec![0x14], Op::inc8_reg { src: D }, None),
        (vec![0x15], Op::dec8_reg { src: D }, None),
        (vec![0x16, 0xff], Op::ld8_imm { dest: D }, Imm8(0xff)),
        (vec![0x17], Op::rla, None),
        (vec![0x18, 0xac], Op::jp_rel, Imm8(0xac)),
        (vec![0x19], Op::add16_reg { src: DE }, None),
        (vec![0x1A], Op::ld8_ind { dest: A, src: Addr::DE }, None),
        (vec![0x1B], Op::dec16_reg { src: DE }, None),
        (vec![0x1C], Op::inc8_reg { src: E }, None),
        (vec![0x1D], Op::dec8_reg { src: E }, None),
        (vec![0x1E, 0xff], Op::ld8_imm { dest: E }, Imm8(0xff)),
        (vec![0x1F], Op::rra, None),

        (vec![0x20, 0xbc], Op::jp_rel_cond { cond: Condition::NZ }, Imm8(0xbc)),
        (vec![0x21, 0x99, 0x11], Op::ld16_imm { dest: HL }, Imm16(0x1199)),
        (vec![0x22], Op::st8_ind { dest: Addr::HLI, src: A}, None),
        (vec![0x23], Op::inc16_reg { src: HL }, None),
        (vec![0x24], Op::inc8_reg { src: H }, None),
        (vec![0x25], Op::dec8_reg { src: H }, None),
        (vec![0x26, 0xff], Op::ld8_imm { dest: H }, Imm8(0xff)),
        (vec![0x27], Op::daa, None),
        (vec![0x28, 0xec], Op::jp_rel_cond { cond: Condition::Z }, Imm8(0xec)),
        (vec![0x29], Op::add16_reg { src: HL }, None),
        (vec![0x2A], Op::ld8_ind { dest: A, src: Addr::HLI }, None),
        (vec![0x2B], Op::dec16_reg { src: HL }, None),
        (vec![0x2C], Op::inc8_reg { src: L }, None),
        (vec![0x2D], Op::dec8_reg { src: L }, None),
        (vec![0x2E, 0xff], Op::ld8_imm { dest: L }, Imm8(0xff)),
        (vec![0x2F], Op::cpl, None),

        (vec![0x30, 0xcc], Op::jp_rel_cond { cond: Condition::NC }, Imm8(0xcc)),
        (vec![0x31, 0x99, 0x11], Op::ld16_imm { dest: SP }, Imm16(0x1199)),
        (vec![0x32], Op::st8_ind { dest: Addr::HLD, src: A }, None),
        (vec![0x33], Op::inc16_reg { src: SP }, None),
        (vec![0x34], Op::inc8_ind, None),
        (vec![0x35], Op::dec8_ind, None),
        (vec![0x36, 0xdd], Op::st8_ind_imm, Imm8(0xdd)),
        (vec![0x37], Op::scf, None),
        (vec![0x38, 0xfc], Op::jp_rel_cond { cond: Condition::C }, Imm8(0xfc)),
        (vec![0x39], Op::add16_reg { src: SP }, None),
        (vec![0x3A], Op::ld8_ind { dest: A, src: Addr::HLD }, None),
        (vec![0x3B], Op::dec16_reg { src: SP }, None),
        (vec![0x3C], Op::inc8_reg { src: A }, None),
        (vec![0x3D], Op::dec8_reg { src: A }, None),
        (vec![0x3E, 0xff], Op::ld8_imm { dest: A }, Imm8(0xff)),
        (vec![0x3F], Op::ccf, None),

        (vec![0x40], Op::ld8_rr { dest: B, src: B }, None),
        (vec![0x41], Op::ld8_rr { dest: B, src: C }, None),
        (vec![0x42], Op::ld8_rr { dest: B, src: D }, None),
        (vec![0x43], Op::ld8_rr { dest: B, src: E }, None),
        (vec![0x44], Op::ld8_rr { dest: B, src: H }, None),
        (vec![0x45], Op::ld8_rr { dest: B, src: L }, None),
        (vec![0x46], Op::ld8_ind { dest: B, src: Addr::HL }, None),
        (vec![0x47], Op::ld8_rr { dest: B, src: A }, None),
        (vec![0x48], Op::ld8_rr { dest: C, src: B }, None),
        (vec![0x49], Op::ld8_rr { dest: C, src: C }, None),
        (vec![0x4A], Op::ld8_rr { dest: C, src: D }, None),
        (vec![0x4B], Op::ld8_rr { dest: C, src: E }, None),
        (vec![0x4C], Op::ld8_rr { dest: C, src: H }, None),
        (vec![0x4D], Op::ld8_rr { dest: C, src: L }, None),
        (vec![0x4E], Op::ld8_ind { dest: C, src: Addr::HL }, None),
        (vec![0x4F], Op::ld8_rr { dest: C, src: A }, None),

        (vec![0x50], Op::ld8_rr { dest: D, src: B }, None),
        (vec![0x51], Op::ld8_rr { dest: D, src: C }, None),
        (vec![0x52], Op::ld8_rr { dest: D, src: D }, None),
        (vec![0x53], Op::ld8_rr { dest: D, src: E }, None),
        (vec![0x54], Op::ld8_rr { dest: D, src: H }, None),
        (vec![0x55], Op::ld8_rr { dest: D, src: L }, None),
        (vec![0x56], Op::ld8_ind { dest: D, src: Addr::HL }, None),
        (vec![0x57], Op::ld8_rr { dest: D, src: A }, None),
        (vec![0x58], Op::ld8_rr { dest: E, src: B }, None),
        (vec![0x59], Op::ld8_rr { dest: E, src: C }, None),
        (vec![0x5A], Op::ld8_rr { dest: E, src: D }, None),
        (vec![0x5B], Op::ld8_rr { dest: E, src: E }, None),
        (vec![0x5C], Op::ld8_rr { dest: E, src: H }, None),
        (vec![0x5D], Op::ld8_rr { dest: E, src: L }, None),
        (vec![0x5E], Op::ld8_ind { dest: E, src: Addr::HL }, None),
        (vec![0x5F], Op::ld8_rr { dest: E, src: A }, None),

        (vec![0x60], Op::ld8_rr { dest: H, src: B }, None),
        (vec![0x61], Op::ld8_rr { dest: H, src: C }, None),
        (vec![0x62], Op::ld8_rr { dest: H, src: D }, None),
        (vec![0x63], Op::ld8_rr { dest: H, src: E }, None),
        (vec![0x64], Op::ld8_rr { dest: H, src: H }, None),
        (vec![0x65], Op::ld8_rr { dest: H, src: L }, None),
        (vec![0x66], Op::ld8_ind { dest: H, src: Addr::HL }, None),
        (vec![0x67], Op::ld8_rr { dest: H, src: A }, None),
        (vec![0x68], Op::ld8_rr { dest: L, src: B }, None),
        (vec![0x69], Op::ld8_rr { dest: L, src: C }, None),
        (vec![0x6A], Op::ld8_rr { dest: L, src: D }, None),
        (vec![0x6B], Op::ld8_rr { dest: L, src: E }, None),
        (vec![0x6C], Op::ld8_rr { dest: L, src: H }, None),
        (vec![0x6D], Op::ld8_rr { dest: L, src: L }, None),
        (vec![0x6E], Op::ld8_ind { dest: L, src: Addr::HL }, None),
        (vec![0x6F], Op::ld8_rr { dest: L, src: A }, None),

        (vec![0x70], Op::st8_ind { dest: Addr::HL, src: B }, None),
        (vec![0x71], Op::st8_ind { dest: Addr::HL, src: C }, None),
        (vec![0x72], Op::st8_ind { dest: Addr::HL, src: D }, None),
        (vec![0x73], Op::st8_ind { dest: Addr::HL, src: E }, None),
        (vec![0x74], Op::st8_ind { dest: Addr::HL, src: H }, None),
        (vec![0x75], Op::st8_ind { dest: Addr::HL, src: L }, None),
        (vec![0x76], Op::halt, None),
        (vec![0x77], Op::st8_ind { dest: Addr::HL, src: A }, None),
        (vec![0x78], Op::ld8_rr { dest: A, src: B }, None),
        (vec![0x79], Op::ld8_rr { dest: A, src: C }, None),
        (vec![0x7A], Op::ld8_rr { dest: A, src: D }, None),
        (vec![0x7B], Op::ld8_rr { dest: A, src: E }, None),
        (vec![0x7C], Op::ld8_rr { dest: A, src: H }, None),
        (vec![0x7D], Op::ld8_rr { dest: A, src: L }, None),
        (vec![0x7E], Op::ld8_ind { dest: A, src: Addr::HL }, None),
        (vec![0x7F], Op::ld8_rr { dest: A, src: A }, None),

        (vec![0xA0], Op::and8_reg { src: B }, None),
        (vec![0xA1], Op::and8_reg { src: C }, None),
        (vec![0xA2], Op::and8_reg { src: D }, None),
        (vec![0xA3], Op::and8_reg { src: E }, None),
        (vec![0xA4], Op::and8_reg { src: H }, None),
        (vec![0xA5], Op::and8_reg { src: L }, None),
        (vec![0xA6], Op::and8_ind, None),
        (vec![0xA7], Op::and8_reg { src: A }, None),
        (vec![0xA8], Op::xor8_reg { src: B }, None),
        (vec![0xA9], Op::xor8_reg { src: C }, None),
        (vec![0xAA], Op::xor8_reg { src: D }, None),
        (vec![0xAB], Op::xor8_reg { src: E }, None),
        (vec![0xAC], Op::xor8_reg { src: H }, None),
        (vec![0xAD], Op::xor8_reg { src: L }, None),
        (vec![0xAE], Op::xor8_ind, None),
        (vec![0xAF], Op::xor8_reg { src: A }, None),

        (vec![0xB0], Op::or8_reg { src: B }, None),
        (vec![0xB1], Op::or8_reg { src: C }, None),
        (vec![0xB2], Op::or8_reg { src: D }, None),
        (vec![0xB3], Op::or8_reg { src: E }, None),
        (vec![0xB4], Op::or8_reg { src: H }, None),
        (vec![0xB5], Op::or8_reg { src: L }, None),
        (vec![0xB6], Op::or8_ind, None),
        (vec![0xB7], Op::or8_reg { src: A }, None),

        (vec![0xC0], Op::ret_cond { cond: Condition::NZ }, None),
        (vec![0xC2, 0x01, 0xfc], Op::jp_cond { cond: Condition::NZ }, Imm16(0xfc01)),
        (vec![0xC3, 0xaa, 0xbb], Op::jp, Imm16(0xbbaa)),
        (vec![0xC4, 0xff, 0xee], Op::call_cond { cond: Condition::NZ }, Imm16(0xeeff)),
        (vec![0xC7], Op::rst { target: 0x0000 }, None),
        (vec![0xC8], Op::ret_cond { cond: Condition::Z }, None),
        (vec![0xC9], Op::ret, None),
        (vec![0xCA, 0x02, 0xec], Op::jp_cond { cond: Condition::Z }, Imm16(0xec02)),
        (vec![0xCC, 0x88, 0x77], Op::call_cond { cond: Condition::Z }, Imm16(0x7788)),
        (vec![0xCD, 0xdd, 0xcc], Op::call, Imm16(0xccdd)),
        (vec![0xCF], Op::rst { target: 0x0008 }, None),

        (vec![0xD0], Op::ret_cond { cond: Condition::NC }, None),
        (vec![0xD2, 0x33, 0x0c], Op::jp_cond { cond: Condition::NC }, Imm16(0x0c33)),
        (vec![0xD4, 0x66, 0x77], Op::call_cond { cond: Condition::NC }, Imm16(0x7766)),
        (vec![0xD7], Op::rst { target: 0x0010 }, None),
        (vec![0xD8], Op::ret_cond { cond: Condition::C }, None),
        (vec![0xD9], Op::reti, None),
        (vec![0xDA, 0x56, 0x44], Op::jp_cond { cond: Condition::C }, Imm16(0x4456)),
        (vec![0xDC, 0xff, 0x77], Op::call_cond { cond: Condition::C }, Imm16(0x77ff)),
        (vec![0xDF], Op::rst { target: 0x0018 }, None),

        (vec![0xE6, 0x34], Op::and8_imm, Imm8(0x34)),
        (vec![0xE7], Op::rst { target: 0x0020 }, None),
        (vec![0xE9], Op::jp_ind, None),
        (vec![0xEE, 0x36], Op::xor8_imm, Imm8(0x36)),
        (vec![0xEF], Op::rst { target: 0x0028 }, None),

        (vec![0xF6, 0x35], Op::or8_imm, Imm8(0x35)),
        (vec![0xF7], Op::rst { target: 0x0030 }, None),
        (vec![0xFF], Op::rst { target: 0x0038 }, None),

        (vec![0xCB, 0x30], Op::swap { src: B }, None),
        (vec![0xCB, 0x31], Op::swap { src: C }, None),
        (vec![0xCB, 0x32], Op::swap { src: D }, None),
        (vec![0xCB, 0x33], Op::swap { src: E }, None),
        (vec![0xCB, 0x34], Op::swap { src: H }, None),
        (vec![0xCB, 0x35], Op::swap { src: L }, None),
        (vec![0xCB, 0x36], Op::swap_ind, None),
        (vec![0xCB, 0x37], Op::swap { src: A }, None),

        (vec![0xCB, 0x40], Op::bit { src: B, bit: 0 }, None),
        (vec![0xCB, 0x41], Op::bit { src: C, bit: 0 }, None),
        (vec![0xCB, 0x42], Op::bit { src: D, bit: 0 }, None),
        (vec![0xCB, 0x43], Op::bit { src: E, bit: 0 }, None),
        (vec![0xCB, 0x44], Op::bit { src: H, bit: 0 }, None),
        (vec![0xCB, 0x45], Op::bit { src: L, bit: 0 }, None),
        (vec![0xCB, 0x46], Op::bit_ind     { bit: 0 }, None),
        (vec![0xCB, 0x47], Op::bit { src: A, bit: 0 }, None),
        (vec![0xCB, 0x48], Op::bit { src: B, bit: 1 }, None),
        (vec![0xCB, 0x49], Op::bit { src: C, bit: 1 }, None),
        (vec![0xCB, 0x4A], Op::bit { src: D, bit: 1 }, None),
        (vec![0xCB, 0x4B], Op::bit { src: E, bit: 1 }, None),
        (vec![0xCB, 0x4C], Op::bit { src: H, bit: 1 }, None),
        (vec![0xCB, 0x4D], Op::bit { src: L, bit: 1 }, None),
        (vec![0xCB, 0x4E], Op::bit_ind     { bit: 1 }, None),
        (vec![0xCB, 0x4F], Op::bit { src: A, bit: 1 }, None),

        (vec![0xCB, 0x50], Op::bit { src: B, bit: 2 }, None),
        (vec![0xCB, 0x51], Op::bit { src: C, bit: 2 }, None),
        (vec![0xCB, 0x52], Op::bit { src: D, bit: 2 }, None),
        (vec![0xCB, 0x53], Op::bit { src: E, bit: 2 }, None),
        (vec![0xCB, 0x54], Op::bit { src: H, bit: 2 }, None),
        (vec![0xCB, 0x55], Op::bit { src: L, bit: 2 }, None),
        (vec![0xCB, 0x56], Op::bit_ind     { bit: 2 }, None),
        (vec![0xCB, 0x57], Op::bit { src: A, bit: 2 }, None),
        (vec![0xCB, 0x58], Op::bit { src: B, bit: 3 }, None),
        (vec![0xCB, 0x59], Op::bit { src: C, bit: 3 }, None),
        (vec![0xCB, 0x5A], Op::bit { src: D, bit: 3 }, None),
        (vec![0xCB, 0x5B], Op::bit { src: E, bit: 3 }, None),
        (vec![0xCB, 0x5C], Op::bit { src: H, bit: 3 }, None),
        (vec![0xCB, 0x5D], Op::bit { src: L, bit: 3 }, None),
        (vec![0xCB, 0x5E], Op::bit_ind     { bit: 3 }, None),
        (vec![0xCB, 0x5F], Op::bit { src: A, bit: 3 }, None),

        (vec![0xCB, 0x60], Op::bit { src: B, bit: 4 }, None),
        (vec![0xCB, 0x61], Op::bit { src: C, bit: 4 }, None),
        (vec![0xCB, 0x62], Op::bit { src: D, bit: 4 }, None),
        (vec![0xCB, 0x63], Op::bit { src: E, bit: 4 }, None),
        (vec![0xCB, 0x64], Op::bit { src: H, bit: 4 }, None),
        (vec![0xCB, 0x65], Op::bit { src: L, bit: 4 }, None),
        (vec![0xCB, 0x66], Op::bit_ind     { bit: 4 }, None),
        (vec![0xCB, 0x67], Op::bit { src: A, bit: 4 }, None),
        (vec![0xCB, 0x68], Op::bit { src: B, bit: 5 }, None),
        (vec![0xCB, 0x69], Op::bit { src: C, bit: 5 }, None),
        (vec![0xCB, 0x6A], Op::bit { src: D, bit: 5 }, None),
        (vec![0xCB, 0x6B], Op::bit { src: E, bit: 5 }, None),
        (vec![0xCB, 0x6C], Op::bit { src: H, bit: 5 }, None),
        (vec![0xCB, 0x6D], Op::bit { src: L, bit: 5 }, None),
        (vec![0xCB, 0x6E], Op::bit_ind     { bit: 5 }, None),
        (vec![0xCB, 0x6F], Op::bit { src: A, bit: 5 }, None),

        (vec![0xCB, 0x70], Op::bit { src: B, bit: 6 }, None),
        (vec![0xCB, 0x71], Op::bit { src: C, bit: 6 }, None),
        (vec![0xCB, 0x72], Op::bit { src: D, bit: 6 }, None),
        (vec![0xCB, 0x73], Op::bit { src: E, bit: 6 }, None),
        (vec![0xCB, 0x74], Op::bit { src: H, bit: 6 }, None),
        (vec![0xCB, 0x75], Op::bit { src: L, bit: 6 }, None),
        (vec![0xCB, 0x76], Op::bit_ind     { bit: 6 }, None),
        (vec![0xCB, 0x77], Op::bit { src: A, bit: 6 }, None),
        (vec![0xCB, 0x78], Op::bit { src: B, bit: 7 }, None),
        (vec![0xCB, 0x79], Op::bit { src: C, bit: 7 }, None),
        (vec![0xCB, 0x7A], Op::bit { src: D, bit: 7 }, None),
        (vec![0xCB, 0x7B], Op::bit { src: E, bit: 7 }, None),
        (vec![0xCB, 0x7C], Op::bit { src: H, bit: 7 }, None),
        (vec![0xCB, 0x7D], Op::bit { src: L, bit: 7 }, None),
        (vec![0xCB, 0x7E], Op::bit_ind     { bit: 7 }, None),
        (vec![0xCB, 0x7F], Op::bit { src: A, bit: 7 }, None),
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
