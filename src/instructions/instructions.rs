// This file is part of GBEmu.
// Copyright (C) 2016 Max Seitzer <contact@max-seitzer.de>
//
// GBEmu is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// GBEmu is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with GBEmu.  If not, see <http://www.gnu.org/licenses/>.
use std::fmt;

use super::registers::{Reg8, Reg16};
use super::opcodes::{OPCODES, EXT_OPCODES};

#[derive(Copy, Clone, Debug)]
pub struct Instr {
    pub op: Op,
    pub imm: Immediate,
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(test, derive(Eq, PartialEq))]
pub enum Condition {
    NZ,
    Z,
    NC,
    C
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(test, derive(Eq, PartialEq))]
pub enum Immediate {
    None,
    Imm8(u8),
    Imm16(u16)
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(test, derive(Eq, PartialEq))]
pub enum Addr {
    BC,
    DE,
    HL,
    HLI,
    HLD,
    Imm,
    #[allow(dead_code)]
    IO,
    #[allow(dead_code)]
    IOC
}

#[derive(Copy, Clone, Debug)]
#[allow(non_camel_case_types)]
#[cfg_attr(test, derive(Eq, PartialEq))]
pub enum Op {
    inv,
    ext,

    nop,
    stop,
    halt,
    di,
    ei,

    ld8_imm { dest: Reg8 },         // dest = imm8
    ld8_rr { dest: Reg8, src: Reg8 },  // dest = src
    ld8_ind { dest: Reg8, src: Addr }, // dest = (src)

    ld16_sp,                        // SP = HL
    ld16_imm { dest: Reg16 },       // BC/DE/HL/SP = imm16
    ld16_lea,                       // HL = SP+imm8
 
    st8_ind_imm,                    // (HL) = imm8
    st8_ind { dest: Addr, src: Reg8 }, // (src) = reg

    st16_sp,                        // (imm16) = SP

    push16 { src: Reg16 },          // (SP) = AF/BC/DE/HL; SP-=2
    pop16 { dest: Reg16 },          // AF/BC/DE/HL = (SP); SP+=2

    in8_reg,                        // A = ($FF00+C)
    in8_imm,                        // A = ($FF00+imm8)
    out8_reg,                       // ($FF00+C) = A
    out8_imm,                       // ($FF00+imm8) = A

    add8_reg { src: Reg8 },         // A += src
    add8_ind,                       // A += (HL)
    add8_imm,                       // A += imm8
    add8_sp_imm,                    // SP += imm8
    add16_reg { src: Reg16 },       // HL += BC/DE/HL/SP

    adc8_reg { src: Reg8 },         // A += src + CF
    adc8_ind,                       // A += (HL) + CF
    adc8_imm,                       // A += imm8 + CF

    sub8_reg { src: Reg8 },         // A -= src
    sub8_ind,                       // A -= (HL)
    sub8_imm,                       // A -= imm8

    sbc8_reg { src: Reg8 },         // A -= src + CF
    sbc8_ind,                       // A -= (HL) + CF
    sbc8_imm,                       // A -= imm8 + CF

    and8_reg { src: Reg8 },         // A &= src
    and8_ind,                       // A &= (HL)
    and8_imm,                       // A &= imm8

    or8_reg { src: Reg8 },          // A |= src
    or8_ind,                        // A |= (HL)
    or8_imm,                        // A |= imm8

    xor8_reg { src: Reg8 },         // A ^= src
    xor8_ind,                       // A ^= (HL)
    xor8_imm,                       // A ^= imm8

    inc8_reg { src: Reg8 },         // src += 1
    inc8_ind,                       // (HL) += 1
    inc16_reg { src: Reg16 },       // BC/DE/HL/SP += 1

    dec8_reg { src: Reg8 },         // src -= 1
    dec8_ind,                       // (HL) -= 1
    dec16_reg { src: Reg16 },       // BC/DE/HL/SP -= 1

    cp8_reg { src: Reg8 },          // cmp A, src
    cp8_ind,                        // cmp A, (HL)
    cp8_imm,                        // cmp A, imm8

    swap { src: Reg8 },             // src = (src >> 4) | (src & 0xf) << 4
    swap_ind,                       // (HL) = ((HL) >> 4) | ((HL) & 0xf) << 4

    rla,                            // A = rotate_left_t_carry(A, 1)
    rl { src: Reg8 },               // src = rotate_left_t_carry(src, 1)
    rl_ind,                         // (HL) = rotate_left_t_carry((HL), 1)
    rlca,                           // A = rotate_left(A, 1)
    rlc { src: Reg8 },              // src = rotate_left(src, 1)
    rlc_ind,                        // (HL) = rotate_left((HL), 1)
    rra,                            // A = rotate_right_t_carry(A, 1)
    rr { src: Reg8 },               // src = rotate_right_t_carry(src, 1)
    rr_ind,                         // (HL) = rotate_right_t_carry((HL), 1)
    rrca,                           // A = rotate_right(A 1)
    rrc { src: Reg8 },              // src = rotate_right(src 1)
    rrc_ind,                        // (HL) = rotate_right((HL) 1)

    sla { src: Reg8 },              // src = shift_left_arith(src, 1)
    sla_ind,                        // (HL) = shift_left_arith((HL), 1)
    sra { src: Reg8 },              // src = shift_right_arith(src, 1)
    sra_ind,                        // (HL) = shift_right_arith((HL), 1)
    srl { src: Reg8 },              // src = shift_right_logical(src, 1)
    srl_ind,                        // (HL) = shift_right_logical((HL), 1)

    bit { src: Reg8, bit: u8 },     // ZF = (src & bit) != 0
    bit_ind { bit: u8 },            // ZF = ((HL) & bit) != 0
    set { src: Reg8, bit: u8 },     // src |= bit
    set_ind { bit: u8 },            // (HL) |= bit
    res { src: Reg8, bit: u8 },     // src &= !bit
    res_ind { bit: u8 },            // (HL) &= !bit

    daa,                            // A = decimal_adjust(A)
    cpl,                            // A = complement(A)
    ccf,                            // CF ^= 1 [complement carry flag] 
    scf,                            // CF = 1

    jp,
    jp_cond { cond: Condition },
    jp_ind,
    jp_rel,
    jp_rel_cond { cond: Condition },

    call,
    call_cond { cond: Condition }, 
    ret,
    ret_cond { cond: Condition },
    reti,

    rst { target: u16 },
}

impl fmt::Display for Instr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let res = write!(f, "{:?}", self.op);

        match self.imm {
            Immediate::Imm8(value) => { 
                if let Op::jp_rel_cond { cond: _ } = self.op {
                    write!(f, " {:02}", value as i8)
                } else {
                    write!(f, " {:#02x}", value)
                }
            },
            Immediate::Imm16(value) => { write!(f, " {:#06x}", value) },
            Immediate::None => { res }
        }
    }
}

impl Immediate {
    pub fn imm8(&self) -> u8 {
        if let &Immediate::Imm8(value) = self {
            return value;
        }
        panic!("Tried extracting Imm8 from non-Imm8 Immediate");
    }

    pub fn imm16(&self) -> u16 {
        if let &Immediate::Imm16(value) = self {
            return value;
        }
        panic!("Tried extracting Imm8 from non-Imm8 Immediate");
    }
}

pub fn from_opcode(opcode: u8) -> (Op, Immediate) {
    OPCODES[opcode as usize].clone()
}

pub fn from_ext_opcode(opcode: u8) -> Instr {
    Instr { op: EXT_OPCODES[opcode as usize].clone(), imm: Immediate::None }
}
