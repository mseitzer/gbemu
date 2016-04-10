use std::fmt;

use hardware::{self, Bus};
use int_controller::Interrupt;
use mem_map;
use events;
use instructions;
use instructions::{Instr, Immediate, Op, Condition, Addr, Reg8, Reg16};

mod registers;
mod instr_impl;
pub mod debug;
#[cfg(test)]
mod test;

use self::registers::{Registers, Flags, ZERO, SUB, CARRY, HCARRY};

pub struct Cpu<B: Bus> {
    regs: Registers,

    int_flag: bool,
    int_toggle_pending: bool,

    total_cycles: u64,
    last_cycles: u8, // Cycles of last instruction

    pub bus: B,
}

impl<B> Cpu<B> where B: Bus {
    pub fn new(bus: B) -> Cpu<B> {
        Cpu {
            regs: Registers::new(),

            int_flag: false,
            int_toggle_pending: false,

            total_cycles: 0,
            last_cycles: 0,

            bus: bus
        }
    }

    pub fn step(&mut self) -> Option<events::Events> {
        let instr = self.fetch_instr();

        self.execute_instr(instr);
        
        let mut events = self.handle_updates();

        self.handle_interrupts();
        
        let mut events2 = self.handle_updates();

        match events {
            Some(e1) => if let Some(e2) = events {
                Some(e1 | e2)
            } else {
                Some(e1)
            },
            None => events2
        }
    }

    fn handle_updates(&mut self) -> Option<events::Events> {
        let mut events = None;
        self.total_cycles += self.last_cycles as u64;
        if self.last_cycles != 0 {
            events = self.bus.update(self.last_cycles);
        }
        self.last_cycles = 0;
        events
    }

    fn handle_interrupts(&mut self) {
        use super::int_controller::Interrupt::*;

        if self.int_flag && self.bus.has_irq(){
            if let Some(int) = self.bus.ack_irq() {
                println!("Interrupt {:?} occured at cycle {}", 
                    int, self.total_cycles);

                self.int_flag = false;

                let pc = self.regs.pc;
                self.push(pc);
                self.regs.pc = int.isr_addr();

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
                self.regs.write8(dest, instr.imm.imm8());
            },
            Op::ld8_rr { dest, src } => {
                let value = self.regs.read8(src);
                self.regs.write8(dest, value);
            },
            Op::ld8_ind { dest, src } => {;
                let value = self.read_addr(src, &instr);
                self.regs.write8(dest, value);
            }

            Op::ld16_sp => {
                self.regs.sp = self.regs.read16(Reg16::HL);
            },
            Op::ld16_imm { dest } => {
                self.regs.write16(dest, instr.imm.imm16());
            },
            Op::ld16_lea => {
                let addr = self.regs.sp + instr.imm.imm8() as u16;
                self.regs.write16(Reg16::HL, addr);
            },

            Op::st8_ind_imm => {
                let addr = self.regs.read16(Reg16::HL);
                self.bus.write(addr, instr.imm.imm8());
            },
            Op::st8_ind { dest, src } => {
                let value = self.regs.read8(src);
                self.write_addr(dest, &instr, value);
            },
            Op::st16_sp => {
                let value = self.regs.sp;
                self.write_word(instr.imm.imm16(), value);
            },

            Op::push16 { src } => {
                let value = self.regs.read16(src);
                self.push(value);
            },
            Op::pop16 { dest } => {
                let value = self.pop();
                self.regs.write16(dest, value);
            },

            /* I/O instructions */
            Op::in8_reg => {
                let ofs = self.regs.read8(Reg8::C) as u16;
                let value = self.bus.read(mem_map::IO_LO + ofs);
                self.regs.write8(Reg8::A, value);
            },
            Op::in8_imm => {
                let ofs = instr.imm.imm8() as u16;
                let value = self.bus.read(mem_map::IO_LO + ofs);
                self.regs.write8(Reg8::A, value);
            },
            Op::out8_reg => {
                let value = self.regs.read8(Reg8::A);
                let ofs = self.regs.read8(Reg8::C) as u16;
                self.bus.write(mem_map::IO_LO + ofs, value)
            },
            Op::out8_imm => {
                let value = self.regs.read8(Reg8::A);
                let ofs = instr.imm.imm8() as u16;
                self.bus.write(mem_map::IO_LO + ofs, value)
            },

            /* ALU instructions */
            Op::add8_reg { src } => {
                let a = self.regs.read8(Reg8::A);
                let b = self.regs.read8(src);
                let value = self.alu_add_bytes(a, b, false);
                self.regs.write8(Reg8::A, value);
            },
            Op::add8_ind => {
                let addr = self.regs.read16(Reg16::HL);
                let a = self.regs.read8(Reg8::A);
                let b = self.bus.read(addr);
                let value = self.alu_add_bytes(a, b, false);
                self.regs.write8(Reg8::A, value);
            },
            Op::add8_imm => {
                let a = self.regs.read8(Reg8::A);
                let b = instr.imm.imm8();
                let value = self.alu_add_bytes(a, b, false);
                self.regs.write8(Reg8::A, value);
            },
            Op::add8_sp_imm => {
                let a = self.regs.sp;
                let b = instr.imm.imm8() as u16;
                self.regs.sp = self.alu_add_words(a, b);
                self.regs.f.remove(ZERO);
            },
            Op::add16_reg { src } => {
                let a = self.regs.read16(Reg16::HL);
                let b = self.regs.read16(src);
                let value = self.alu_add_words(a, b);
                self.regs.write16(Reg16::HL, value);
            },

            Op::adc8_reg { src } => {
                let a = self.regs.read8(Reg8::A);
                let b = self.regs.read8(src);
                let value = self.alu_add_bytes(a, b, true);
                self.regs.write8(Reg8::A, value);
            },
            Op::adc8_ind => {
                let addr = self.regs.read16(Reg16::HL);
                let a = self.regs.read8(Reg8::A);
                let b = self.bus.read(addr);
                let value = self.alu_add_bytes(a, b, true);
                self.regs.write8(Reg8::A, value);
            },
            Op::adc8_imm => {
                let a = self.regs.read8(Reg8::A);
                let b = instr.imm.imm8();
                let value = self.alu_add_bytes(a, b, true);
                self.regs.write8(Reg8::A, value);
            },

            Op::sub8_reg { src } => {
                let a = self.regs.read8(Reg8::A);
                let b = self.regs.read8(src);
                let value = self.alu_sub_bytes(a, b, false);
                self.regs.write8(Reg8::A, value);
            },
            Op::sub8_ind => {
                let addr = self.regs.read16(Reg16::HL);
                let a = self.regs.read8(Reg8::A);
                let b = self.bus.read(addr);
                let value = self.alu_sub_bytes(a, b, false);
                self.regs.write8(Reg8::A, value);
            },
            Op::sub8_imm => {
                let a = self.regs.read8(Reg8::A);
                let b = instr.imm.imm8();
                let value = self.alu_sub_bytes(a, b, false);
                self.regs.write8(Reg8::A, value);
            },

            Op::sbc8_reg { src } => {
                let a = self.regs.read8(Reg8::A);
                let b = self.regs.read8(src);
                let value = self.alu_sub_bytes(a, b, true);
                self.regs.write8(Reg8::A, value);
            },
            Op::sbc8_ind => {
                let addr = self.regs.read16(Reg16::HL);
                let a = self.regs.read8(Reg8::A);
                let b = self.bus.read(addr);
                let value = self.alu_sub_bytes(a, b, true);
                self.regs.write8(Reg8::A, value);
            },
            Op::sbc8_imm => {
                let a = self.regs.read8(Reg8::A);
                let b = instr.imm.imm8();
                let value = self.alu_sub_bytes(a, b, true);
                self.regs.write8(Reg8::A, value);
            },

            Op::and8_reg { src } => {
                let a = self.regs.read8(Reg8::A);
                let b = self.regs.read8(src);
                let value = self.alu_and_bytes(a, b);
                self.regs.write8(Reg8::A, value);
            },
            Op::and8_ind => {
                let addr = self.regs.read16(Reg16::HL);
                let a = self.regs.read8(Reg8::A);
                let b = self.bus.read(addr);
                let value = self.alu_and_bytes(a, b);
                self.regs.write8(Reg8::A, value);
            },
            Op::and8_imm => {
                let a = self.regs.read8(Reg8::A);
                let b = instr.imm.imm8();
                let value = self.alu_and_bytes(a, b);
                self.regs.write8(Reg8::A, value);
            },

            Op::or8_reg { src } => {
                let a = self.regs.read8(Reg8::A);
                let b = self.regs.read8(src);
                let value = self.alu_or_bytes(a, b);
                self.regs.write8(Reg8::A, value);
            },
            Op::or8_ind => {
                let addr = self.regs.read16(Reg16::HL);
                let a = self.regs.read8(Reg8::A);
                let b = self.bus.read(addr);
                let value = self.alu_or_bytes(a, b);
                self.regs.write8(Reg8::A, value);
            },
            Op::or8_imm => {
                let a = self.regs.read8(Reg8::A);
                let b = instr.imm.imm8();
                let value = self.alu_or_bytes(a, b);
                self.regs.write8(Reg8::A, value);
            },

            Op::xor8_reg { src } => {
                let a = self.regs.read8(Reg8::A);
                let b = self.regs.read8(src);
                let value = self.alu_xor_bytes(a, b);
                self.regs.write8(Reg8::A, value);
            },
            Op::xor8_ind => {
                let addr = self.regs.read16(Reg16::HL);
                let a = self.regs.read8(Reg8::A);
                let b = self.bus.read(addr);
                let value = self.alu_xor_bytes(a, b);
                self.regs.write8(Reg8::A, value);
            },
            Op::xor8_imm => {
                let a = self.regs.read8(Reg8::A);
                let b = instr.imm.imm8();
                let value = self.alu_xor_bytes(a, b);
                self.regs.write8(Reg8::A, value);
            },

            Op::inc8_reg { src } => {
                let carry = self.regs.f.contains(CARRY);
                let a = self.regs.read8(src);
                let value = self.alu_add_bytes(a, 1, false);
                self.regs.f.force(CARRY, carry);
                self.regs.write8(src, value);
            },
            Op::inc8_ind => {
                let carry = self.regs.f.contains(CARRY);
                let addr = self.regs.read16(Reg16::HL);
                let a = self.bus.read(addr);
                let value = self.alu_add_bytes(a, 1, false);
                self.regs.f.force(CARRY, carry);
                self.bus.write(addr, value);
            },
            Op::inc16_reg { src } => {
                let value = self.regs.read16(src).wrapping_add(1);
                self.regs.write16(src, value);
            },

            Op::dec8_reg { src } => {
                let carry = self.regs.f.contains(CARRY);
                let a = self.regs.read8(src);
                let value = self.alu_sub_bytes(a, 1, false);
                self.regs.f.force(CARRY, carry);
                self.regs.write8(src, value);
            },
            Op::dec8_ind => {
                let carry = self.regs.f.contains(CARRY);
                let addr = self.regs.read16(Reg16::HL);
                let a = self.bus.read(addr);
                let value = self.alu_sub_bytes(a, 1, false);
                self.regs.f.force(CARRY, carry);
                self.bus.write(addr, value);
            },
            Op::dec16_reg { src } => {
                let value = self.regs.read16(src).wrapping_sub(1);
                self.regs.write16(src, value);
            },

            Op::cp8_reg { src } => {
                let a = self.regs.read8(Reg8::A);
                let b = self.regs.read8(src);
                let _ = self.alu_sub_bytes(a, b, false);
            },
            Op::cp8_ind => {
                let addr = self.regs.read16(Reg16::HL);
                let a = self.regs.read8(Reg8::A);
                let b = self.bus.read(addr);
                let _ = self.alu_sub_bytes(a, b, false);
            },
            Op::cp8_imm => {
                let a = self.regs.read8(Reg8::A);
                let b = instr.imm.imm8();
                let _ = self.alu_sub_bytes(a, b, false);
            },

            Op::swap { src } => {
                let value = self.regs.read8(src);
                self.regs.f = ZERO.test(value == 0);
                self.regs.write8(src, (value & 0xf) << 4 | (value >> 4));
            },
            Op::swap_ind => {
                let addr = self.regs.read16(Reg16::HL);
                let value = self.bus.read(addr);
                self.regs.f = ZERO.test(value == 0);
                self.bus.write(addr, (value & 0xf) << 4 | (value >> 4));
            },

            /* Rotate & shift instructions */
            Op::rla => {
                let mut value = self.regs.read8(Reg8::A);
                value = self.rotate_left(value);
                self.regs.f.remove(ZERO);
                self.regs.write8(Reg8::A, value);
            },
            Op::rl { src } => {
                let mut value = self.regs.read8(src);
                value = self.rotate_left(value);
                self.regs.write8(src, value);
            },
            Op::rl_ind => {
                let addr = self.regs.read16(Reg16::HL);
                let mut value = self.bus.read(addr);
                value = self.rotate_left(value);
                self.bus.write(addr, value);
            },

            Op::rlca => {
                let mut value = self.regs.read8(Reg8::A);
                value = self.rotate_left_carry(value);
                self.regs.f.remove(ZERO);
                self.regs.write8(Reg8::A, value);
            },
            Op::rlc { src } => {
                let mut value = self.regs.read8(src);
                value = self.rotate_left_carry(value);
                self.regs.write8(src, value);
            },
            Op::rlc_ind => {
                let addr = self.regs.read16(Reg16::HL);
                let mut value = self.bus.read(addr);
                value = self.rotate_left_carry(value);
                self.bus.write(addr, value);
            }

            Op::rra => {
                let mut value = self.regs.read8(Reg8::A);
                value = self.rotate_right(value);
                self.regs.f.remove(ZERO);
                self.regs.write8(Reg8::A, value);
            },
            Op::rr { src } => {
                let mut value = self.regs.read8(src);
                value = self.rotate_right(value);
                self.regs.write8(src, value);
            },
            Op::rr_ind => {
                let addr = self.regs.read16(Reg16::HL);
                let mut value = self.bus.read(addr);
                value = self.rotate_right(value);
                self.bus.write(addr, value);
            },

            Op::rrca => {
                let mut value = self.regs.read8(Reg8::A);
                value = self.rotate_right_carry(value);
                self.regs.f.remove(ZERO);
                self.regs.write8(Reg8::A, value);
            },
            Op::rrc { src } => {
                let mut value = self.regs.read8(src);
                value = self.rotate_right_carry(value);
                self.regs.write8(src, value);
            },
            Op::rrc_ind => {
                let addr = self.regs.read16(Reg16::HL);
                let mut value = self.bus.read(addr);
                value = self.rotate_right_carry(value);
                self.bus.write(addr, value);
            },

            Op::sla { src } => {
                let mut value = self.regs.read8(src);
                value = self.shift_left_arithmetic(value);
                self.regs.write8(src, value);
            },
            Op::sla_ind => {
                let addr = self.regs.read16(Reg16::HL);
                let mut value = self.bus.read(addr);
                value = self.shift_left_arithmetic(value);
                self.bus.write(addr, value);
            },
            Op::sra { src } => {
                let mut value = self.regs.read8(src);
                value = self.shift_right_arithmetic(value);
                self.regs.write8(src, value);
            },
            Op::sra_ind => {
                let addr = self.regs.read16(Reg16::HL);
                let mut value = self.bus.read(addr);
                value = self.shift_right_arithmetic(value);
                self.bus.write(addr, value);
            },
            Op::srl { src } => {
                let mut value = self.regs.read8(src);
                value = self.shift_right_logical(value);
                self.regs.write8(src, value);
            },
            Op::srl_ind => {
                let addr = self.regs.read16(Reg16::HL);
                let mut value = self.bus.read(addr);
                value = self.shift_right_logical(value);
                self.bus.write(addr, value);
            },

            /* Bit operation instructions */
            Op::bit { src, bit } => {
                let value = self.regs.read8(src);
                self.regs.f = self.regs.f | HCARRY |
                             ZERO.test(get_bit!(value, bit) == 0);
                self.regs.f.remove(SUB);
            },
            Op::bit_ind { bit } => {
                let addr = self.regs.read16(Reg16::HL);
                let value = self.bus.read(addr);
                self.regs.f = self.regs.f | HCARRY |
                             ZERO.test(get_bit!(value, bit) == 0);
                self.regs.f.remove(SUB);
            },
            Op::set { src, bit } => {
                let value = self.regs.read8(src);
                self.regs.write8(src, set_bit!(value, bit));
            },
            Op::set_ind { bit } => {
                let addr = self.regs.read16(Reg16::HL);
                let value = self.bus.read(addr);
                self.bus.write(addr, set_bit!(value, bit));
            },
            Op::res { src, bit } => {
                let value = self.regs.read8(src);
                self.regs.write8(src, reset_bit!(value, bit));
            },
            Op::res_ind { bit } => {
                let addr = self.regs.read16(Reg16::HL);
                let value = self.bus.read(addr);
                self.bus.write(addr, reset_bit!(value, bit));
            },

            /* Jump instructions */
            Op::jp => {
                self.regs.pc = instr.imm.imm16();
            },
            Op::jp_cond { ref cond } => {
                if self.jmp_cond_fulfilled(cond) {
                    self.regs.pc = instr.imm.imm16();
                    jumped = true;
                }
            },
            Op::jp_ind => {
                self.regs.pc = self.regs.read16(Reg16::HL);
            },
            Op::jp_rel => {
                let ofs = instr.imm.imm8() as i8;
                if ofs >= 0 {
                    self.regs.pc = self.regs.pc.wrapping_add(ofs as u16);
                } else {
                    self.regs.pc = self.regs.pc.wrapping_sub(ofs.abs() as u16);
                }
            },
            Op::jp_rel_cond { ref cond } => {
                if self.jmp_cond_fulfilled(cond) {
                    let ofs = instr.imm.imm8() as i8;
                    if ofs >= 0 {
                        self.regs.pc = self.regs.pc.wrapping_add(ofs as u16);
                    } else {
                        self.regs.pc = self.regs.pc.wrapping_sub(ofs.abs() as u16);
                    }
                    
                    jumped = true;
                }
            },

            /* Call/ret instructions */
            Op::call => {
                let pc = self.regs.pc;
                self.push(pc);
                self.regs.pc = instr.imm.imm16();
            },
            Op::call_cond { ref cond } => {
                if self.jmp_cond_fulfilled(cond) {
                    let pc = self.regs.pc;
                    self.push(pc);
                    self.regs.pc = instr.imm.imm16();
                    jumped = true;
                }
            },
            Op::ret => {
                self.regs.pc = self.pop();
            },
            Op::ret_cond { ref cond } => {
                if self.jmp_cond_fulfilled(cond) {
                    self.regs.pc = self.pop();
                    jumped = true;
                }
            },
            Op::reti => {
                self.regs.pc = self.pop();
                self.int_flag = true;
            },

            Op::rst { target } => {
                self.regs.sp = self.regs.sp.wrapping_sub(2);
                let addr = self.regs.sp;
                let value = self.regs.pc;
                self.write_word(addr, value);
                self.regs.pc = target;
            }

            /* Misc instructions */
            Op::daa => {
                let mut value = self.regs.read8(Reg8::A);
                if !self.regs.f.contains(SUB) {
                    if self.regs.f.contains(CARRY) || value > 0x99 {
                        value = value.wrapping_add(0x60);
                        self.regs.f.insert(CARRY);
                    }
                    if self.regs.f.contains(HCARRY) || value & 0x0f > 0x09 {
                        value = value.wrapping_add(0x06);
                    }
                } else {
                    if self.regs.f.contains(CARRY) {
                        if self.regs.f.contains(HCARRY) {
                            value = value.wrapping_add(0x9a);
                        } else {
                            value = value.wrapping_add(0xa0);
                        }
                    } else if self.regs.f.contains(HCARRY) {
                        value += value.wrapping_add(0xfa);
                    }
                }
                self.regs.f.remove(HCARRY);
                self.regs.f = self.regs.f | ZERO.test(value == 0);
                self.regs.write8(Reg8::A, value);
            },

            Op::cpl => {
                let value = self.regs.read8(Reg8::A);
                self.regs.f.insert(SUB);
                self.regs.f.insert(HCARRY);
                self.regs.write8(Reg8::A, !value);
            },
            Op::ccf => {
                self.regs.f.remove(SUB);
                self.regs.f.remove(HCARRY);
                self.regs.f.toggle(CARRY);
            },
            Op::scf => {
                self.regs.f.remove(SUB);
                self.regs.f.remove(HCARRY);
                self.regs.f.insert(CARRY);
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

    fn extract_instr8(&mut self) -> u8 {
        let addr = self.regs.pc;
        let value = self.bus.read(addr);
        self.regs.pc = self.regs.pc.wrapping_add(1);
        return value;
    }

    fn extract_instr16(&mut self) -> u16 {
        let addr = self.regs.pc;
        let value = self.read_word(addr);
        self.regs.pc = self.regs.pc.wrapping_add(2);
        return value;
    }

    fn resolve_addr(&mut self, addr: Addr, instr: &Instr) -> u16 {
        match addr {
            Addr::BC => self.regs.read16(Reg16::BC),
            Addr::DE => self.regs.read16(Reg16::DE),
            Addr::HL => self.regs.read16(Reg16::HL),
            Addr::HLI => {
                let value = self.regs.read16(Reg16::HL);
                self.regs.write16(Reg16::HL, value.wrapping_add(1));
                value
            },
            Addr::HLD => {
                let value = self.regs.read16(Reg16::HL);
                self.regs.write16(Reg16::HL, value.wrapping_sub(1));
                value
            },
            Addr::Imm => instr.imm.imm16(),
            Addr::IO => mem_map::IO_LO + instr.imm.imm8() as u16,
            Addr::IOC => mem_map::IO_LO + self.regs.read8(Reg8::C) as u16
        }
    }

    fn read_addr(&mut self, addr: Addr, instr: &Instr) -> u8 {
        let addr_value = self.resolve_addr(addr, instr);
        self.bus.read(addr_value)
    }

    fn write_addr(&mut self, addr: Addr, instr: &Instr, value: u8) {
        let addr_value = self.resolve_addr(addr, instr);
        self.bus.write(addr_value, value)
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
        let carry = if self.regs.f.contains(CARRY) {with_carry as u8} else {0};
        let bc = b.wrapping_add(carry);
        let res = a.wrapping_add(bc);

        let a_7 = get_bit!(a, 7);
        let bc_7 = get_bit!(bc, 7);
        let res_7 = get_bit!(res, 7);
        let a_3 = get_bit!(a, 3);
        let bc_3 = get_bit!(bc, 3);
        let res_3 = get_bit!(res, 3);

        self.regs.f = ZERO.test(res == 0) |
                      CARRY.test(((a_7 | bc_7) == 1 && res_7 == 0) 
                         || ((a_7 & bc_7) == 1 && res_7 == 1)) |
                      HCARRY.test(((a_3 | bc_3) == 1 && res_3 == 0) 
                         || ((a_3 & bc_3) == 1 && res_3 == 1));
        return res;
    }

    fn alu_sub_bytes(&mut self, a: u8, b: u8, with_carry: bool) -> u8 {
        let res = self.alu_add_bytes(a, (!b).wrapping_add(1), with_carry);

        self.regs.f = ZERO.test(self.regs.f.contains(ZERO)) |
                      SUB |
                      CARRY.test(!self.regs.f.contains(CARRY)) |
                      HCARRY.test(!self.regs.f.contains(HCARRY));
        return res;
    }

    fn alu_add_words(&mut self, a: u16, b: u16) -> u16 {
        let res = a.wrapping_add(b);

        let a_15 = get_bit!(a, 15);
        let b_15 = get_bit!(b, 15);
        let res_15 = get_bit!(res, 15);
        let a_11 = get_bit!(a, 11);
        let b_11 = get_bit!(b, 11);
        let res_11 = get_bit!(res, 11);

        self.regs.f = ZERO.test(self.regs.f.contains(ZERO)) |
                      CARRY.test(((a_15 | b_15) == 1 && res_15 == 0) 
                         || (a_15 & b_15) == 1 && res_15 == 1) |
                      HCARRY.test(((a_11 | b_11) == 1 && res_11 == 0) 
                         || (a_11 & b_11) == 1 && res_11 == 1);
        return res;
    }

    fn alu_and_bytes(&mut self, a: u8, b: u8) -> u8 {
        let res = a & b;
        self.regs.f = ZERO.test(res == 0) |
                      HCARRY.test(true);
        return res;
    }

    fn alu_or_bytes(&mut self, a: u8, b: u8) -> u8 {
        let res = a | b;
        self.regs.f = ZERO.test(res == 0);
        return res;
    }

    fn alu_xor_bytes(&mut self, a: u8, b: u8) -> u8 {
        let res = a ^ b;
        self.regs.f = ZERO.test(res == 0);
        return res;
    }

    fn jmp_cond_fulfilled(&self, cond: &Condition) -> bool {
        match cond {
            &Condition::Z  => self.regs.f.contains(ZERO),
            &Condition::C  => self.regs.f.contains(CARRY),
            &Condition::NC => !self.regs.f.contains(CARRY),
            &Condition::NZ => !self.regs.f.contains(ZERO),
        }
    }
}

impl<B> fmt::Display for Cpu<B> where B: Bus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(writeln!(f, "PC: {:#06x}", self.regs.pc));
        try!(writeln!(f, "SP: {:#06x}", self.regs.sp));
        try!(writeln!(f, "Z: {}, S: {}, C: {}, HC: {}",
            self.regs.f.contains(ZERO) as u8, self.regs.f.contains(SUB) as u8,
            self.regs.f.contains(CARRY) as u8, self.regs.f.contains(HCARRY) as u8));
        try!(writeln!(f, "A | {:#04x} | {:#04x} | F", self.regs.a, self.regs.f.bits()));
        try!(writeln!(f, "B | {:#04x} | {:#04x} | C", self.regs.b, self.regs.c));
        try!(writeln!(f, "D | {:#04x} | {:#04x} | E", self.regs.d, self.regs.e));
        write!(f, "H | {:#04x} | {:#04x} | L", self.regs.h, self.regs.l)
    }
}
