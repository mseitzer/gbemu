use super::hardware::{self, Bus};
use super::int_controller::Interrupt;
use super::mem_map;
use super::instructions;
use super::instructions::{Instr, Immediate, Op, Condition, Reg8, Reg16};

use std::fmt;

mod instr_impl;
pub mod debug;

#[cfg(test)]
mod test;

pub struct Cpu<B: Bus> {
    pc: u16,
    sp: u16,

    flags: CpuFlags,
    
    int_flag: bool,
    int_toggle_pending: bool,

    /*
    General purpose registers
    | A | F |
    | B | C |
    | D | E |
    | H | L |
    */
    regs: [u8; 8],

    total_cycles: u64,
    last_cycles: u8, // Cycles of last instruction

    bus: B,
}

bitflags! {
    flags CpuFlags: u8 {
        const ZERO     = 1 << 7,
        const SUB      = 1 << 6,
        const HCARRY   = 1 << 5,
        const CARRY    = 1 << 4,
    }
}

impl CpuFlags {
    fn test(&self, test: bool) -> CpuFlags {
        if test { *self } else { CpuFlags::empty() }
    }

    fn force(&mut self, flag: CpuFlags, insert: bool) {
        if insert {
            self.insert(flag)
        } else {
            self.remove(flag)
        }
    }
}

impl<B> Cpu<B> where B: Bus {
    pub fn new(bus: B) -> Cpu<B> {
        Cpu {
            pc: 0,
            sp: 0,

            flags: CpuFlags::empty(),

            int_flag: false,
            int_toggle_pending: false,

            regs: [0; 8],

            total_cycles: 0,
            last_cycles: 0,

            bus: bus
        }
    }

    pub fn run(&mut self) {
        loop {
            let instr = self.fetch_instr();

            //println!("Current instruction: {}\n", instr);

            self.execute_instr(instr);
            
            self.handle_updates();
            self.handle_interrupts();
            self.handle_updates();
        }
    }

    fn handle_updates(&mut self) {
        self.total_cycles += self.last_cycles as u64;
        if self.last_cycles != 0 {
            self.bus.update(self.last_cycles);
        }
        self.last_cycles = 0;
    }

    fn handle_interrupts(&mut self) {
        use super::int_controller::Interrupt::*;

        if self.int_flag && self.bus.has_irq(){
            if let Some(int) = self.bus.ack_irq() {
                println!("Interrupt {:?} occured at cycle {}", 
                    int, self.total_cycles);

                self.int_flag = false;

                let pc = self.pc;
                self.push(pc);
                self.pc = int.isr_addr();

                // This might be 3 as well, depending on if the CPU executes 
                // 2 nop instructions before executing the ISR
                self.last_cycles = 5;
            }
        }
    }

    fn fetch_instr(&mut self) -> Instr {
        use super::instructions::Immediate::{None, Imm8, Imm16};

        let opcode = self.extract_instr8();
        match instructions::from_opcode(opcode) {
            (Op::ext, None) => {
                let ext_opcode = self.extract_instr8();
                instructions::from_ext_opcode(ext_opcode)
            },
            (op, imm @ None) => Instr { op: op, imm: imm },
            (op, Imm8(_)) =>
                Instr { op: op, imm: Imm8(self.extract_instr8()) },
            (op, Imm16(_)) =>
                Instr { op: op, imm: Imm16(self.extract_instr16()) }
        }
    }

    fn execute_instr(&mut self, instr: Instr) {
        use instructions::Immediate::{Imm8, Imm16};

        let mut jumped = false;
        let toggle_ints = self.int_toggle_pending;
        self.int_toggle_pending = false;

        match instr.op {
            Op::nop => {},
            Op::stop => {
                // TODO: implement
            },
            Op::halt => {
                // TODO: implement
            },
            Op::di => {
                self.int_toggle_pending = true;
            },
            Op::ei => {
                self.int_toggle_pending = true;
            },

            /* load/store instructions */
            Op::ld8_imm { dest } => {
                self.write_reg8(dest, instr.imm.imm8());
            },
            Op::ld8_rr { dest, src } => {
                let value = self.read_reg8(src);
                self.write_reg8(dest, value);
            },
            Op::ld8_ind_reg { dest, src } => {
                let addr = self.read_reg16(src);
                let value = self.bus.read(addr);
                self.write_reg8(dest, value);
            },
            Op::ld8_ind_imm16 => {
                let value = self.bus.read(instr.imm.imm16());
                self.write_reg8(Reg8::A, value);
            },
            Op::ld8_ind_dec => {
                let addr = self.read_reg16(Reg16::HL);
                let value = self.bus.read(addr);
                self.write_reg8(Reg8::A, value);
                self.write_reg16(Reg16::HL, addr.wrapping_sub(1));
            },
            Op::ld8_ind_inc => {
                let addr = self.read_reg16(Reg16::HL);
                let value = self.bus.read(addr);
                self.write_reg8(Reg8::A, value);
                self.write_reg16(Reg16::HL, addr.wrapping_add(1));
            },

            Op::ld16_sp => {
                self.sp = self.read_reg16(Reg16::HL);
            },
            Op::ld16_sp_imm => {
                self.sp = instr.imm.imm16();
            },
            Op::ld16_imm { dest } => {
                self.write_reg16(dest, instr.imm.imm16());
            },
            Op::ld16_lea => {
                let addr = self.sp + instr.imm.imm8() as u16;
                self.write_reg16(Reg16::HL, addr);
            },

            Op::st8_ind_imm => {
                let addr = self.read_reg16(Reg16::HL);
                self.bus.write(addr, instr.imm.imm8());
            },
            Op::st8_ind_reg { dest, src } => {
                let value = self.read_reg8(src);
                let addr = self.read_reg16(dest);
                self.bus.write(addr, value);
            },
            Op::st8_ind_imm16 => {
                let value = self.read_reg8(Reg8::A);
                self.bus.write(instr.imm.imm16(), value);
            },
            Op::st8_ind_dec => {
                let value = self.read_reg8(Reg8::A);
                let addr = self.read_reg16(Reg16::HL);
                self.bus.write(addr, value);
                self.write_reg16(Reg16::HL, addr.wrapping_sub(1));
            },
            Op::st8_ind_inc => {
                let value = self.read_reg8(Reg8::A);
                let addr = self.read_reg16(Reg16::HL);
                self.bus.write(addr, value);
                self.write_reg16(Reg16::HL, addr.wrapping_add(1));
            },
            Op::st16_sp => {
                let value = self.sp;
                self.write_word(instr.imm.imm16(), value);
            },

            Op::push16 { src } => {
                let value = self.read_reg16(src);
                self.push(value);
            },
            Op::pop16 { dest } => {
                let value = self.pop();
                self.write_reg16(dest, value);
            },

            /* I/O instructions */
            Op::in8_reg => {
                let ofs = self.read_reg8(Reg8::C) as u16;
                let value = self.bus.read(mem_map::IO_LO + ofs);
                self.write_reg8(Reg8::A, value);
            },
            Op::in8_imm => {
                let ofs = instr.imm.imm8() as u16;
                let value = self.bus.read(mem_map::IO_LO + ofs);
                self.write_reg8(Reg8::A, value);
            },
            Op::out8_reg => {
                let value = self.read_reg8(Reg8::A);
                let ofs = self.read_reg8(Reg8::C) as u16;
                self.bus.write(mem_map::IO_LO + ofs, value)
            },
            Op::out8_imm => {
                let value = self.read_reg8(Reg8::A);
                let ofs = instr.imm.imm8() as u16;
                self.bus.write(mem_map::IO_LO + ofs, value)
            },

            /* ALU instructions */
            Op::add8_reg { src } => {
                let a = self.read_reg8(Reg8::A);
                let b = self.read_reg8(src);
                let value = self.alu_add_bytes(a, b, false);
                self.write_reg8(Reg8::A, value);
            },
            Op::add8_ind => {
                let addr = self.read_reg16(Reg16::HL);
                let a = self.read_reg8(Reg8::A);
                let b = self.bus.read(addr);
                let value = self.alu_add_bytes(a, b, false);
                self.write_reg8(Reg8::A, value);
            },
            Op::add8_imm => {
                let a = self.read_reg8(Reg8::A);
                let b = instr.imm.imm8();
                let value = self.alu_add_bytes(a, b, false);
                self.write_reg8(Reg8::A, value);
            },
            Op::add8_sp_imm => {
                let a = self.sp;
                let b = instr.imm.imm8() as u16;
                self.sp = self.alu_add_words(a, b, false);
                self.flags.remove(ZERO);
            },
            Op::add16_reg { src } => {
                let value = self.read_reg16(Reg16::HL).wrapping_add(
                    self.read_reg16(src));
                self.write_reg16(Reg16::HL, value);
            },
            Op::add16_sp => {
                let value = self.read_reg16(Reg16::HL).wrapping_add(
                    self.sp);
                self.write_reg16(Reg16::HL, value);
            },

            Op::adc8_reg { src } => {
                let a = self.read_reg8(Reg8::A);
                let b = self.read_reg8(src);
                let value = self.alu_add_bytes(a, b, true);
                self.write_reg8(Reg8::A, value);
            },
            Op::adc8_ind => {
                let addr = self.read_reg16(Reg16::HL);
                let a = self.read_reg8(Reg8::A);
                let b = self.bus.read(addr);
                let value = self.alu_add_bytes(a, b, true);
                self.write_reg8(Reg8::A, value);
            },
            Op::adc8_imm => {
                let a = self.read_reg8(Reg8::A);
                let b = instr.imm.imm8();
                let value = self.alu_add_bytes(a, b, true);
                self.write_reg8(Reg8::A, value);
            },

            Op::sub8_reg { src } => {
                let a = self.read_reg8(Reg8::A);
                let b = self.read_reg8(src);
                let value = self.alu_sub_bytes(a, b, false);
                self.write_reg8(Reg8::A, value);
            },
            Op::sub8_ind => {
                let addr = self.read_reg16(Reg16::HL);
                let a = self.read_reg8(Reg8::A);
                let b = self.bus.read(addr);
                let value = self.alu_sub_bytes(a, b, false);
                self.write_reg8(Reg8::A, value);
            },
            Op::sub8_imm => {
                let a = self.read_reg8(Reg8::A);
                let b = instr.imm.imm8();
                let value = self.alu_sub_bytes(a, b, false);
                self.write_reg8(Reg8::A, value);
            },

            Op::sbc8_reg { src } => {
                let a = self.read_reg8(Reg8::A);
                let b = self.read_reg8(src);
                let value = self.alu_sub_bytes(a, b, true);
                self.write_reg8(Reg8::A, value);
            },
            Op::sbc8_ind => {
                let addr = self.read_reg16(Reg16::HL);
                let a = self.read_reg8(Reg8::A);
                let b = self.bus.read(addr);
                let value = self.alu_sub_bytes(a, b, true);
                self.write_reg8(Reg8::A, value);
            },
            Op::sbc8_imm => {
                let a = self.read_reg8(Reg8::A);
                let b = instr.imm.imm8();
                let value = self.alu_sub_bytes(a, b, true);
                self.write_reg8(Reg8::A, value);
            },

            Op::and8_reg { src } => {
                let a = self.read_reg8(Reg8::A);
                let b = self.read_reg8(src);
                let value = self.alu_and_bytes(a, b);
                self.write_reg8(Reg8::A, value);
            },
            Op::and8_ind => {
                let addr = self.read_reg16(Reg16::HL);
                let a = self.read_reg8(Reg8::A);
                let b = self.bus.read(addr);
                let value = self.alu_and_bytes(a, b);
                self.write_reg8(Reg8::A, value);
            },
            Op::and8_imm => {
                let a = self.read_reg8(Reg8::A);
                let b = instr.imm.imm8();
                let value = self.alu_and_bytes(a, b);
                self.write_reg8(Reg8::A, value);
            },

            Op::or8_reg { src } => {
                let a = self.read_reg8(Reg8::A);
                let b = self.read_reg8(src);
                let value = self.alu_or_bytes(a, b);
                self.write_reg8(Reg8::A, value);
            },
            Op::or8_ind => {
                let addr = self.read_reg16(Reg16::HL);
                let a = self.read_reg8(Reg8::A);
                let b = self.bus.read(addr);
                let value = self.alu_or_bytes(a, b);
                self.write_reg8(Reg8::A, value);
            },
            Op::or8_imm => {
                let a = self.read_reg8(Reg8::A);
                let b = instr.imm.imm8();
                let value = self.alu_or_bytes(a, b);
                self.write_reg8(Reg8::A, value);
            },

            Op::xor8_reg { src } => {
                let a = self.read_reg8(Reg8::A);
                let b = self.read_reg8(src);
                let value = self.alu_xor_bytes(a, b);
                self.write_reg8(Reg8::A, value);
            },
            Op::xor8_ind => {
                let addr = self.read_reg16(Reg16::HL);
                let a = self.read_reg8(Reg8::A);
                let b = self.bus.read(addr);
                let value = self.alu_xor_bytes(a, b);
                self.write_reg8(Reg8::A, value);
            },
            Op::xor8_imm => {
                let a = self.read_reg8(Reg8::A);
                let b = instr.imm.imm8();
                let value = self.alu_xor_bytes(a, b);
                self.write_reg8(Reg8::A, value);
            },

            Op::inc8_reg { src } => {
                let carry = self.flags.contains(CARRY);
                let a = self.read_reg8(src);
                let value = self.alu_add_bytes(a, 1, false);
                self.flags.force(CARRY, carry);
                self.write_reg8(src, value);
            },
            Op::inc8_ind => {
                let carry = self.flags.contains(CARRY);
                let addr = self.read_reg16(Reg16::HL);
                let a = self.bus.read(addr);
                let value = self.alu_add_bytes(a, 1, false);
                self.flags.force(CARRY, carry);
                self.bus.write(addr, value);
            },
            Op::inc16_reg { src } => {
                let value = self.read_reg16(src).wrapping_add(1);
                self.write_reg16(src, value);
            },
            Op::inc16_sp => {
                self.sp = self.sp.wrapping_add(1);
            },

            Op::dec8_reg { src } => {
                let carry = self.flags.contains(CARRY);
                let a = self.read_reg8(src);
                let value = self.alu_sub_bytes(a, 1, false);
                self.flags.force(CARRY, carry);
                self.write_reg8(src, value);
            },
            Op::dec8_ind => {
                let carry = self.flags.contains(CARRY);
                let addr = self.read_reg16(Reg16::HL);
                let a = self.bus.read(addr);
                let value = self.alu_sub_bytes(a, 1, false);
                self.flags.force(CARRY, carry);
                self.bus.write(addr, value);
            },
            Op::dec16_reg { src } => {
                let value = self.read_reg16(src).wrapping_sub(1);
                self.write_reg16(src, value);
            },
            Op::dec16_sp => {
                self.sp = self.sp.wrapping_sub(1);
            },

            Op::cp8_reg { src } => {
                let a = self.read_reg8(Reg8::A);
                let b = self.read_reg8(src);
                let _ = self.alu_sub_bytes(a, b, false);
            },
            Op::cp8_ind => {
                let addr = self.read_reg16(Reg16::HL);
                let a = self.read_reg8(Reg8::A);
                let b = self.bus.read(addr);
                let _ = self.alu_sub_bytes(a, b, false);
            },
            Op::cp8_imm => {
                let a = self.read_reg8(Reg8::A);
                let b = instr.imm.imm8();
                let _ = self.alu_sub_bytes(a, b, false);
            },

            Op::swap { src } => {
                let value = self.read_reg8(src);
                self.flags = ZERO.test(value == 0);
                self.write_reg8(src, (value & 0xf) << 4 | (value >> 4));
            },
            Op::swap_ind => {
                let addr = self.read_reg16(Reg16::HL);
                let value = self.bus.read(addr);
                self.flags = ZERO.test(value == 0);
                self.bus.write(addr, (value & 0xf) << 4 | (value >> 4));
            },

            /* Rotate & shift instructions */
            Op::rla => {
                let mut value = self.read_reg8(Reg8::A);
                value = self.rotate_left(value);
                self.flags.remove(ZERO);
                self.write_reg8(Reg8::A, value);
            },
            Op::rl { src } => {
                let mut value = self.read_reg8(src);
                value = self.rotate_left(value);
                self.write_reg8(src, value);
            },
            Op::rl_ind => {
                let addr = self.read_reg16(Reg16::HL);
                let mut value = self.bus.read(addr);
                value = self.rotate_left(value);
                self.bus.write(addr, value);
            },

            Op::rlca => {
                let mut value = self.read_reg8(Reg8::A);
                value = self.rotate_left_carry(value);
                self.flags.remove(ZERO);
                self.write_reg8(Reg8::A, value);
            },
            Op::rlc { src } => {
                let mut value = self.read_reg8(src);
                value = self.rotate_left_carry(value);
                self.write_reg8(src, value);
            },
            Op::rlc_ind => {
                let addr = self.read_reg16(Reg16::HL);
                let mut value = self.bus.read(addr);
                value = self.rotate_left_carry(value);
                self.bus.write(addr, value);
            }

            Op::rra => {
                let mut value = self.read_reg8(Reg8::A);
                value = self.rotate_right(value);
                self.flags.remove(ZERO);
                self.write_reg8(Reg8::A, value);
            },
            Op::rr { src } => {
                let mut value = self.read_reg8(src);
                value = self.rotate_right(value);
                self.write_reg8(src, value);
            },
            Op::rr_ind => {
                let addr = self.read_reg16(Reg16::HL);
                let mut value = self.bus.read(addr);
                value = self.rotate_right(value);
                self.bus.write(addr, value);
            },

            Op::rrca => {
                let mut value = self.read_reg8(Reg8::A);
                value = self.rotate_right_carry(value);
                self.flags.remove(ZERO);
                self.write_reg8(Reg8::A, value);
            },
            Op::rrc { src } => {
                let mut value = self.read_reg8(src);
                value = self.rotate_right_carry(value);
                self.write_reg8(src, value);
            },
            Op::rrc_ind => {
                let addr = self.read_reg16(Reg16::HL);
                let mut value = self.bus.read(addr);
                value = self.rotate_right_carry(value);
                self.bus.write(addr, value);
            },

            Op::sla { src } => {
                let mut value = self.read_reg8(src);
                value = self.shift_left_arithmetic(value);
                self.write_reg8(src, value);
            },
            Op::sla_ind => {
                let addr = self.read_reg16(Reg16::HL);
                let mut value = self.bus.read(addr);
                value = self.shift_left_arithmetic(value);
                self.bus.write(addr, value);
            },
            Op::sra { src } => {
                let mut value = self.read_reg8(src);
                value = self.shift_right_arithmetic(value);
                self.write_reg8(src, value);
            },
            Op::sra_ind => {
                let addr = self.read_reg16(Reg16::HL);
                let mut value = self.bus.read(addr);
                value = self.shift_right_arithmetic(value);
                self.bus.write(addr, value);
            },
            Op::srl { src } => {
                let mut value = self.read_reg8(src);
                value = self.shift_right_logical(value);
                self.write_reg8(src, value);
            },
            Op::srl_ind => {
                let addr = self.read_reg16(Reg16::HL);
                let mut value = self.bus.read(addr);
                value = self.shift_right_logical(value);
                self.bus.write(addr, value);
            },

            /* Bit operation instructions */
            Op::bit { src, bit } => {
                let value = self.read_reg8(src);
                self.flags = self.flags | HCARRY |
                             ZERO.test(get_bit!(value, bit) == 0);
                self.flags.remove(SUB);
            },
            Op::bit_ind { bit } => {
                let addr = self.read_reg16(Reg16::HL);
                let value = self.bus.read(addr);
                self.flags = self.flags | HCARRY |
                             ZERO.test(get_bit!(value, bit) == 0);
                self.flags.remove(SUB);
            },
            Op::set { src, bit } => {
                let value = self.read_reg8(src);
                self.write_reg8(src, set_bit!(value, bit));
            },
            Op::set_ind { bit } => {
                let addr = self.read_reg16(Reg16::HL);
                let value = self.bus.read(addr);
                self.bus.write(addr, set_bit!(value, bit));
            },
            Op::res { src, bit } => {
                let value = self.read_reg8(src);
                self.write_reg8(src, reset_bit!(value, bit));
            },
            Op::res_ind { bit } => {
                let addr = self.read_reg16(Reg16::HL);
                let value = self.bus.read(addr);
                self.bus.write(addr, reset_bit!(value, bit));
            },

            /* Jump instructions */
            Op::jp => {
                self.pc = instr.imm.imm16();
            },
            Op::jp_cond { ref cond } => {
                if self.jmp_cond_fulfilled(cond) {
                    self.pc = instr.imm.imm16();
                    jumped = true;
                }
            },
            Op::jp_ind => {
                self.pc = self.read_reg16(Reg16::HL);
            },
            Op::jp_rel => {
                let ofs = instr.imm.imm8() as i8;
                if ofs >= 0 {
                    self.pc = self.pc.wrapping_add(ofs as u16);
                } else {
                    self.pc = self.pc.wrapping_sub(ofs.abs() as u16);
                }
            },
            Op::jp_rel_cond { ref cond } => {
                if self.jmp_cond_fulfilled(cond) {
                    let ofs = instr.imm.imm8() as i8;
                    if ofs >= 0 {
                        self.pc = self.pc.wrapping_add(ofs as u16);
                    } else {
                        self.pc = self.pc.wrapping_sub(ofs.abs() as u16);
                    }
                    
                    jumped = true;
                }
            },

            /* Call/ret instructions */
            Op::call => {
                let pc = self.pc;
                self.push(pc);
                self.pc = instr.imm.imm16();
            },
            Op::call_cond { ref cond } => {
                if self.jmp_cond_fulfilled(cond) {
                    let pc = self.pc;
                    self.push(pc);
                    self.pc = instr.imm.imm16();
                    jumped = true;
                }
            },
            Op::ret => {
                self.pc = self.pop();
            },
            Op::ret_cond { ref cond } => {
                if self.jmp_cond_fulfilled(cond) {
                    self.pc = self.pop();
                    jumped = true;
                }
            },
            Op::reti => {
                self.pc = self.pop();
                self.int_flag = true;
            },

            Op::rst { target } => {
                self.sp = self.sp.wrapping_sub(2);
                let addr = self.sp;
                let value = self.pc;
                self.write_word(addr, value);
                self.pc = target;
            }

            /* Misc instructions */
            Op::daa => {
                let mut value = self.read_reg8(Reg8::A);
                if !self.flags.contains(SUB) {
                    if self.flags.contains(CARRY) || value > 0x99 {
                        value = value.wrapping_add(0x60);
                        self.flags.insert(CARRY);
                    }
                    if self.flags.contains(HCARRY) || value & 0x0f > 0x09 {
                        value = value.wrapping_add(0x06);
                    }
                } else {
                    if self.flags.contains(CARRY) {
                        if self.flags.contains(HCARRY) {
                            value = value.wrapping_add(0x9a);
                        } else {
                            value = value.wrapping_add(0xa0);
                        }
                    } else if self.flags.contains(HCARRY) {
                        value += value.wrapping_add(0xfa);
                    }
                }
                self.flags.remove(HCARRY);
                self.flags = self.flags | ZERO.test(value == 0);
                self.write_reg8(Reg8::A, value);
            },

            Op::cpl => {
                let value = self.read_reg8(Reg8::A);
                self.flags.insert(SUB);
                self.flags.insert(HCARRY);
                self.write_reg8(Reg8::A, !value);
            },
            Op::ccf => {
                self.flags.remove(SUB);
                self.flags.remove(HCARRY);
                self.flags.toggle(CARRY);
            },
            Op::scf => {
                self.flags.remove(SUB);
                self.flags.remove(HCARRY);
                self.flags.insert(CARRY);
            },

            _ => panic!("Trying to execute non-implemented instruction {}.\n{}", 
                        instr, self)
        }

        if jumped {
            self.last_cycles = instructions::cycles_jmp(&instr.op, true);
        } else {
            self.last_cycles = instructions::cycles(&instr.op);
        }
        
        if toggle_ints {
            self.int_flag = !self.int_flag;
        }
    }

    fn read_reg8(&self, dest: Reg8) -> u8 {
        self.regs[dest as usize]
    }

    fn read_reg16(&self, dest: Reg16) -> u16 {
        let lo = self.regs[dest as usize] as u16;
        let hi = (self.regs[(dest as usize)-1] as u16) << 8;
        hi + lo
    }

    fn write_reg8(&mut self, dest: Reg8, value: u8) {
        self.regs[dest as usize] = value;
    }

    fn write_reg16(&mut self, dest: Reg16, value: u16) {
        self.regs[(dest as usize)-1] = (value >> 8) as u8;
        self.regs[dest as usize] = (value & 0x00ff) as u8;
    }

    fn extract_instr8(&mut self) -> u8 {
        let addr = self.pc;
        let value = self.bus.read(addr);
        self.pc = self.pc.wrapping_add(1);
        return value;
    }

    fn extract_instr16(&mut self) -> u16 {
        let addr = self.pc;
        let value = self.read_word(addr);
        self.pc = self.pc.wrapping_add(2);
        return value;
    }

    #[inline(always)]
    fn read_word(&mut self, addr: u16) -> u16 {
        let lo = self.bus.read(addr);
        let hi = self.bus.read(addr+1);
        ((hi as u16) << 8) + lo as u16
    }

    #[inline(always)]
    fn write_word(&mut self, addr: u16, value: u16) {
        let lo = (value & 0x00ff) as u8;
        let hi = (value >> 8) as u8;
        self.bus.write(addr, lo);
        self.bus.write(addr+1, hi);
    }

    pub fn tot_m_cycles(&self) -> u64 {
        self.total_cycles
    }

    pub fn tot_c_cycles(&self) -> u64 {
        self.total_cycles * 4
    }

    pub fn last_m_cycles(&self) -> u8 {
        self.last_cycles
    }

    pub fn last_c_cycles(&self) -> u8 {
        self.last_cycles * 4
    }

    fn alu_add_bytes(&mut self, a: u8, b: u8, with_carry: bool) -> u8 {
        let bc = b.wrapping_add(with_carry as u8);
        let res = a.wrapping_add(bc);

        let a_7 = get_bit!(a, 7);
        let bc_7 = get_bit!(bc, 7);
        let res_7 = get_bit!(res, 7);
        let a_3 = get_bit!(a, 3);
        let bc_3 = get_bit!(bc, 3);
        let res_3 = get_bit!(res, 3);

        self.flags = ZERO.test(res == 0) |
                     CARRY.test(((a_7 | bc_7) == 1 && res_7 == 0) 
                        || (a_7 & bc_7) == 1 && res_7 == 1) |
                     HCARRY.test(((a_3 | bc_3) == 1 && res_3 == 0) 
                        || (a_3 & bc_3) == 1 && res_3 == 1);
        return res;
    }

    fn alu_sub_bytes(&mut self, a: u8, b: u8, with_carry: bool) -> u8 {
        let res = self.alu_add_bytes(a, (!b).wrapping_add(1), with_carry);

        self.flags = self.flags |
                     SUB |
                     CARRY.test(!self.flags.contains(CARRY)) |
                     HCARRY.test(!self.flags.contains(HCARRY));
        return res;
    }

    fn alu_add_words(&mut self, a: u16, b: u16, with_carry: bool) -> u16 {
        let bc = b.wrapping_add(with_carry as u16);
        let res = a.wrapping_add(bc);

        let a_15 = get_bit!(a, 15);
        let bc_15 = get_bit!(bc, 15);
        let res_15 = get_bit!(res, 15);
        let a_11 = get_bit!(a, 11);
        let bc_11 = get_bit!(bc, 11);
        let res_11 = get_bit!(res, 11);

        self.flags = self.flags |
                     CARRY.test(((a_15 | bc_15) == 1 && res_15 == 0) 
                        || (a_15 & bc_15) == 1 && res_15 == 1) |
                     HCARRY.test(((a_11 | bc_11) == 1 && res_11 == 0) 
                        || (a_11 & bc_11) == 1 && res_11 == 1);
        self.flags.remove(SUB);
        return res;
    }

    fn alu_and_bytes(&mut self, a: u8, b: u8) -> u8 {
        let res = a & b;
        self.flags = ZERO.test(res == 0) |
                     HCARRY.test(true);
        return res;
    }

    fn alu_or_bytes(&mut self, a: u8, b: u8) -> u8 {
        let res = a | b;
        self.flags = ZERO.test(res == 0);
        return res;
    }

    fn alu_xor_bytes(&mut self, a: u8, b: u8) -> u8 {
        let res = a ^ b;
        self.flags = ZERO.test(res == 0);
        return res;
    }

    fn jmp_cond_fulfilled(&self, cond: &Condition) -> bool {
        match cond {
            &Condition::Z  => self.flags.contains(ZERO),
            &Condition::C  => self.flags.contains(CARRY),
            &Condition::NC => !self.flags.contains(CARRY),
            &Condition::NZ => !self.flags.contains(ZERO),
        }
    }
}

impl<B> fmt::Display for Cpu<B> where B: Bus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(writeln!(f, "PC: {:#06x}", self.pc));
        try!(writeln!(f, "SP: {:#06x}", self.sp));
        try!(writeln!(f, "Z: {}, S: {}, C: {}, HC: {}",
            self.flags.contains(ZERO) as u8, self.flags.contains(SUB) as u8,
            self.flags.contains(CARRY) as u8, self.flags.contains(HCARRY) as u8));
        try!(writeln!(f, "A | {:#04x} | {:#04x} | F", self.regs[0], self.regs[1]));
        try!(writeln!(f, "B | {:#04x} | {:#04x} | C", self.regs[2], self.regs[3]));
        try!(writeln!(f, "D | {:#04x} | {:#04x} | E", self.regs[4], self.regs[5]));
        write!(f, "H | {:#04x} | {:#04x} | L", self.regs[6], self.regs[7])
    }
}