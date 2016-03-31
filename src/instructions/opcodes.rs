/* THIS IS AN AUTOGENERATED FILE, DO NOT EDIT */

use super::registers::{Reg8, Reg16};
use super::instructions::Addr;
use super::instructions::Op;
use super::instructions::Op::*;
use super::instructions::Condition;
use super::instructions::Immediate;
use super::instructions::Immediate::*;

const IMM8: Immediate = Immediate::Imm8(0);
const IMM16: Immediate = Immediate::Imm16(0);

pub const OPCODES: [(Op, Immediate); 256] = [
    (nop, None), //0x00
    (ld16_imm { dest: Reg16::BC }, IMM16), //0x01
    (st8_ind { dest: Addr::BC, src: Reg8::A }, None), //0x02
    (inc16_reg { src: Reg16::BC }, None), //0x03
    (inc8_reg { src: Reg8::B }, None), //0x04
    (dec8_reg { src: Reg8::B }, None), //0x05
    (ld8_imm { dest: Reg8::B }, IMM8), //0x06
    (rlca, None), //0x07
    (st16_sp, IMM16), //0x08
    (add16_reg { src: Reg16::BC }, None), //0x09
    (ld8_ind { dest: Reg8::A, src: Addr::BC }, None), //0x0A
    (dec16_reg { src: Reg16::BC }, None), //0x0B
    (inc8_reg { src: Reg8::C }, None), //0x0C
    (dec8_reg { src: Reg8::C }, None), //0x0D
    (ld8_imm { dest: Reg8::C }, IMM8), //0x0E
    (rrca, None), //0x0F
    (stop, None), //0x10
    (ld16_imm { dest: Reg16::DE }, IMM16), //0x11
    (st8_ind { dest: Addr::DE, src: Reg8::A }, None), //0x12
    (inc16_reg { src: Reg16::DE }, None), //0x13
    (inc8_reg { src: Reg8::D }, None), //0x14
    (dec8_reg { src: Reg8::D }, None), //0x15
    (ld8_imm { dest: Reg8::D }, IMM8), //0x16
    (rla, None), //0x17
    (jp_rel, IMM8), //0x18
    (add16_reg { src: Reg16::DE }, None), //0x19
    (ld8_ind { dest: Reg8::A, src: Addr::DE }, None), //0x1A
    (dec16_reg { src: Reg16::DE }, None), //0x1B
    (inc8_reg { src: Reg8::E }, None), //0x1C
    (dec8_reg { src: Reg8::E }, None), //0x1D
    (ld8_imm { dest: Reg8::E }, IMM8), //0x1E
    (rra, None), //0x1F
    (jp_rel_cond { cond: Condition::NZ }, IMM8), //0x20
    (ld16_imm { dest: Reg16::HL }, IMM16), //0x21
    (st8_ind { dest: Addr::HLI, src: Reg8::A }, None), //0x22
    (inc16_reg { src: Reg16::HL }, None), //0x23
    (inc8_reg { src: Reg8::H }, None), //0x24
    (dec8_reg { src: Reg8::H }, None), //0x25
    (ld8_imm { dest: Reg8::H }, IMM8), //0x26
    (daa, None), //0x27
    (jp_rel_cond { cond: Condition::Z }, IMM8), //0x28
    (add16_reg { src: Reg16::HL }, None), //0x29
    (ld8_ind { dest: Reg8::A, src: Addr::HLI }, None), //0x2A
    (dec16_reg { src: Reg16::HL }, None), //0x2B
    (inc8_reg { src: Reg8::L }, None), //0x2C
    (dec8_reg { src: Reg8::L }, None), //0x2D
    (ld8_imm { dest: Reg8::L }, IMM8), //0x2E
    (cpl, None), //0x2F
    (jp_rel_cond { cond: Condition::NC }, IMM8), //0x30
    (ld16_imm { dest: Reg16::SP }, IMM16), //0x31
    (st8_ind { dest: Addr::HLD, src: Reg8::A }, None), //0x32
    (inc16_reg { src: Reg16::SP }, None), //0x33
    (inc8_ind, None), //0x34
    (dec8_ind, None), //0x35
    (st8_ind_imm, IMM8), //0x36
    (scf, None), //0x37
    (jp_rel_cond { cond: Condition::C }, IMM8), //0x38
    (add16_reg { src: Reg16::SP }, None), //0x39
    (ld8_ind { dest: Reg8::A, src: Addr::HLD }, None), //0x3A
    (dec16_reg { src: Reg16::SP }, None), //0x3B
    (inc8_reg { src: Reg8::A }, None), //0x3C
    (dec8_reg { src: Reg8::A }, None), //0x3D
    (ld8_imm { dest: Reg8::A }, IMM8), //0x3E
    (ccf, None), //0x3F
    (ld8_rr { dest: Reg8::B, src: Reg8::B }, None), //0x40
    (ld8_rr { dest: Reg8::B, src: Reg8::C }, None), //0x41
    (ld8_rr { dest: Reg8::B, src: Reg8::D }, None), //0x42
    (ld8_rr { dest: Reg8::B, src: Reg8::E }, None), //0x43
    (ld8_rr { dest: Reg8::B, src: Reg8::H }, None), //0x44
    (ld8_rr { dest: Reg8::B, src: Reg8::L }, None), //0x45
    (ld8_ind { dest: Reg8::B, src: Addr::HL }, None), //0x46
    (ld8_rr { dest: Reg8::B, src: Reg8::A }, None), //0x47
    (ld8_rr { dest: Reg8::C, src: Reg8::B }, None), //0x48
    (ld8_rr { dest: Reg8::C, src: Reg8::C }, None), //0x49
    (ld8_rr { dest: Reg8::C, src: Reg8::D }, None), //0x4A
    (ld8_rr { dest: Reg8::C, src: Reg8::E }, None), //0x4B
    (ld8_rr { dest: Reg8::C, src: Reg8::H }, None), //0x4C
    (ld8_rr { dest: Reg8::C, src: Reg8::L }, None), //0x4D
    (ld8_ind { dest: Reg8::C, src: Addr::HL }, None), //0x4E
    (ld8_rr { dest: Reg8::C, src: Reg8::A }, None), //0x4F
    (ld8_rr { dest: Reg8::D, src: Reg8::B }, None), //0x50
    (ld8_rr { dest: Reg8::D, src: Reg8::C }, None), //0x51
    (ld8_rr { dest: Reg8::D, src: Reg8::D }, None), //0x52
    (ld8_rr { dest: Reg8::D, src: Reg8::E }, None), //0x53
    (ld8_rr { dest: Reg8::D, src: Reg8::H }, None), //0x54
    (ld8_rr { dest: Reg8::D, src: Reg8::L }, None), //0x55
    (ld8_ind { dest: Reg8::D, src: Addr::HL }, None), //0x56
    (ld8_rr { dest: Reg8::D, src: Reg8::A }, None), //0x57
    (ld8_rr { dest: Reg8::E, src: Reg8::B }, None), //0x58
    (ld8_rr { dest: Reg8::E, src: Reg8::C }, None), //0x59
    (ld8_rr { dest: Reg8::E, src: Reg8::D }, None), //0x5A
    (ld8_rr { dest: Reg8::E, src: Reg8::E }, None), //0x5B
    (ld8_rr { dest: Reg8::E, src: Reg8::H }, None), //0x5C
    (ld8_rr { dest: Reg8::E, src: Reg8::L }, None), //0x5D
    (ld8_ind { dest: Reg8::E, src: Addr::HL }, None), //0x5E
    (ld8_rr { dest: Reg8::E, src: Reg8::A }, None), //0x5F
    (ld8_rr { dest: Reg8::H, src: Reg8::B }, None), //0x60
    (ld8_rr { dest: Reg8::H, src: Reg8::C }, None), //0x61
    (ld8_rr { dest: Reg8::H, src: Reg8::D }, None), //0x62
    (ld8_rr { dest: Reg8::H, src: Reg8::E }, None), //0x63
    (ld8_rr { dest: Reg8::H, src: Reg8::H }, None), //0x64
    (ld8_rr { dest: Reg8::H, src: Reg8::L }, None), //0x65
    (ld8_ind { dest: Reg8::H, src: Addr::HL }, None), //0x66
    (ld8_rr { dest: Reg8::H, src: Reg8::A }, None), //0x67
    (ld8_rr { dest: Reg8::L, src: Reg8::B }, None), //0x68
    (ld8_rr { dest: Reg8::L, src: Reg8::C }, None), //0x69
    (ld8_rr { dest: Reg8::L, src: Reg8::D }, None), //0x6A
    (ld8_rr { dest: Reg8::L, src: Reg8::E }, None), //0x6B
    (ld8_rr { dest: Reg8::L, src: Reg8::H }, None), //0x6C
    (ld8_rr { dest: Reg8::L, src: Reg8::L }, None), //0x6D
    (ld8_ind { dest: Reg8::L, src: Addr::HL }, None), //0x6E
    (ld8_rr { dest: Reg8::L, src: Reg8::A }, None), //0x6F
    (st8_ind { dest: Addr::HL, src: Reg8::B }, None), //0x70
    (st8_ind { dest: Addr::HL, src: Reg8::C }, None), //0x71
    (st8_ind { dest: Addr::HL, src: Reg8::D }, None), //0x72
    (st8_ind { dest: Addr::HL, src: Reg8::E }, None), //0x73
    (st8_ind { dest: Addr::HL, src: Reg8::H }, None), //0x74
    (st8_ind { dest: Addr::HL, src: Reg8::L }, None), //0x75
    (halt, None), //0x76
    (st8_ind { dest: Addr::HL, src: Reg8::A }, None), //0x77
    (ld8_rr { dest: Reg8::A, src: Reg8::B }, None), //0x78
    (ld8_rr { dest: Reg8::A, src: Reg8::C }, None), //0x79
    (ld8_rr { dest: Reg8::A, src: Reg8::D }, None), //0x7A
    (ld8_rr { dest: Reg8::A, src: Reg8::E }, None), //0x7B
    (ld8_rr { dest: Reg8::A, src: Reg8::H }, None), //0x7C
    (ld8_rr { dest: Reg8::A, src: Reg8::L }, None), //0x7D
    (ld8_ind { dest: Reg8::A, src: Addr::HL }, None), //0x7E
    (ld8_rr { dest: Reg8::A, src: Reg8::A }, None), //0x7F
    (add8_reg { src: Reg8::B }, None), //0x80
    (add8_reg { src: Reg8::C }, None), //0x81
    (add8_reg { src: Reg8::D }, None), //0x82
    (add8_reg { src: Reg8::E }, None), //0x83
    (add8_reg { src: Reg8::H }, None), //0x84
    (add8_reg { src: Reg8::L }, None), //0x85
    (add8_ind, None), //0x86
    (add8_reg { src: Reg8::A }, None), //0x87
    (adc8_reg { src: Reg8::B }, None), //0x88
    (adc8_reg { src: Reg8::C }, None), //0x89
    (adc8_reg { src: Reg8::D }, None), //0x8A
    (adc8_reg { src: Reg8::E }, None), //0x8B
    (adc8_reg { src: Reg8::H }, None), //0x8C
    (adc8_reg { src: Reg8::L }, None), //0x8D
    (adc8_ind, None), //0x8E
    (adc8_reg { src: Reg8::A }, None), //0x8F
    (sub8_reg { src: Reg8::B }, None), //0x90
    (sub8_reg { src: Reg8::C }, None), //0x91
    (sub8_reg { src: Reg8::D }, None), //0x92
    (sub8_reg { src: Reg8::E }, None), //0x93
    (sub8_reg { src: Reg8::H }, None), //0x94
    (sub8_reg { src: Reg8::L }, None), //0x95
    (sub8_ind, None), //0x96
    (sub8_reg { src: Reg8::A }, None), //0x97
    (sbc8_reg { src: Reg8::B }, None), //0x98
    (sbc8_reg { src: Reg8::C }, None), //0x99
    (sbc8_reg { src: Reg8::D }, None), //0x9A
    (sbc8_reg { src: Reg8::E }, None), //0x9B
    (sbc8_reg { src: Reg8::H }, None), //0x9C
    (sbc8_reg { src: Reg8::L }, None), //0x9D
    (sbc8_ind, None), //0x9E
    (sbc8_reg { src: Reg8::A }, None), //0x9F
    (and8_reg { src: Reg8::B }, None), //0xA0
    (and8_reg { src: Reg8::C }, None), //0xA1
    (and8_reg { src: Reg8::D }, None), //0xA2
    (and8_reg { src: Reg8::E }, None), //0xA3
    (and8_reg { src: Reg8::H }, None), //0xA4
    (and8_reg { src: Reg8::L }, None), //0xA5
    (and8_ind, None), //0xA6
    (and8_reg { src: Reg8::A }, None), //0xA7
    (xor8_reg { src: Reg8::B }, None), //0xA8
    (xor8_reg { src: Reg8::C }, None), //0xA9
    (xor8_reg { src: Reg8::D }, None), //0xAA
    (xor8_reg { src: Reg8::E }, None), //0xAB
    (xor8_reg { src: Reg8::H }, None), //0xAC
    (xor8_reg { src: Reg8::L }, None), //0xAD
    (xor8_ind, None), //0xAE
    (xor8_reg { src: Reg8::A }, None), //0xAF
    (or8_reg { src: Reg8::B }, None), //0xB0
    (or8_reg { src: Reg8::C }, None), //0xB1
    (or8_reg { src: Reg8::D }, None), //0xB2
    (or8_reg { src: Reg8::E }, None), //0xB3
    (or8_reg { src: Reg8::H }, None), //0xB4
    (or8_reg { src: Reg8::L }, None), //0xB5
    (or8_ind, None), //0xB6
    (or8_reg { src: Reg8::A }, None), //0xB7
    (cp8_reg { src: Reg8::B }, None), //0xB8
    (cp8_reg { src: Reg8::C }, None), //0xB9
    (cp8_reg { src: Reg8::D }, None), //0xBA
    (cp8_reg { src: Reg8::E }, None), //0xBB
    (cp8_reg { src: Reg8::H }, None), //0xBC
    (cp8_reg { src: Reg8::L }, None), //0xBD
    (cp8_ind, None), //0xBE
    (cp8_reg { src: Reg8::A }, None), //0xBF
    (ret_cond { cond: Condition::NZ }, None), //0xC0
    (pop16 { dest: Reg16::BC }, None), //0xC1
    (jp_cond { cond: Condition::NZ }, IMM16), //0xC2
    (jp, IMM16), //0xC3
    (call_cond { cond: Condition::NZ }, IMM16), //0xC4
    (push16 { src: Reg16::BC }, None), //0xC5
    (add8_imm, IMM8), //0xC6
    (rst { target: 0x00 }, None), //0xC7
    (ret_cond { cond: Condition::Z }, None), //0xC8
    (ret, None), //0xC9
    (jp_cond { cond: Condition::Z }, IMM16), //0xCA
    (ext, None), //0xCB
    (call_cond { cond: Condition::Z }, IMM16), //0xCC
    (call, IMM16), //0xCD
    (adc8_imm, IMM8), //0xCE
    (rst { target: 0x08 }, None), //0xCF
    (ret_cond { cond: Condition::NC }, None), //0xD0
    (pop16 { dest: Reg16::DE }, None), //0xD1
    (jp_cond { cond: Condition::NC }, IMM16), //0xD2
    (inv, None), //0xD3
    (call_cond { cond: Condition::NC }, IMM16), //0xD4
    (push16 { src: Reg16::DE }, None), //0xD5
    (sub8_imm, IMM8), //0xD6
    (rst { target: 0x10 }, None), //0xD7
    (ret_cond { cond: Condition::C }, None), //0xD8
    (reti, None), //0xD9
    (jp_cond { cond: Condition::C }, IMM16), //0xDA
    (inv, None), //0xDB
    (call_cond { cond: Condition::C }, IMM16), //0xDC
    (inv, None), //0xDD
    (sbc8_imm, IMM8), //0xDE
    (rst { target: 0x18 }, None), //0xDF
    (out8_imm, IMM8), //0xE0
    (pop16 { dest: Reg16::HL }, None), //0xE1
    (out8_reg, None), //0xE2
    (inv, None), //0xE3
    (inv, None), //0xE4
    (push16 { src: Reg16::HL }, None), //0xE5
    (and8_imm, IMM8), //0xE6
    (rst { target: 0x20 }, None), //0xE7
    (add8_sp_imm, IMM8), //0xE8
    (jp_ind, None), //0xE9
    (st8_ind { dest: Addr::Imm, src: Reg8::A }, IMM16), //0xEA
    (inv, None), //0xEB
    (inv, None), //0xEC
    (inv, None), //0xED
    (xor8_imm, IMM8), //0xEE
    (rst { target: 0x28 }, None), //0xEF
    (in8_imm, IMM8), //0xF0
    (pop16 { dest: Reg16::AF }, None), //0xF1
    (in8_reg, None), //0xF2
    (di, None), //0xF3
    (inv, None), //0xF4
    (push16 { src: Reg16::AF }, None), //0xF5
    (or8_imm, IMM8), //0xF6
    (rst { target: 0x30 }, None), //0xF7
    (ld16_lea, None), //0xF8
    (ld16_sp, None), //0xF9
    (ld8_ind { dest: Reg8::A, src: Addr::Imm }, IMM16), //0xFA
    (ei, None), //0xFB
    (inv, None), //0xFC
    (inv, None), //0xFD
    (cp8_imm, IMM8), //0xFE
    (rst { target: 0x38 }, None), //0xFF
];

pub const EXT_OPCODES: [Op; 256] = [
    rlc { src: Reg8::B }, //0x00
    rlc { src: Reg8::C }, //0x01
    rlc { src: Reg8::D }, //0x02
    rlc { src: Reg8::E }, //0x03
    rlc { src: Reg8::H }, //0x04
    rlc { src: Reg8::L }, //0x05
    rlc_ind, //0x06
    rlc { src: Reg8::A }, //0x07
    rrc { src: Reg8::B }, //0x08
    rrc { src: Reg8::C }, //0x09
    rrc { src: Reg8::D }, //0x0A
    rrc { src: Reg8::E }, //0x0B
    rrc { src: Reg8::H }, //0x0C
    rrc { src: Reg8::L }, //0x0D
    rrc_ind, //0x0E
    rrc { src: Reg8::A }, //0x0F
    rl { src: Reg8::B }, //0x10
    rl { src: Reg8::C }, //0x11
    rl { src: Reg8::D }, //0x12
    rl { src: Reg8::E }, //0x13
    rl { src: Reg8::H }, //0x14
    rl { src: Reg8::L }, //0x15
    rl_ind, //0x16
    rl { src: Reg8::A }, //0x17
    rr { src: Reg8::B }, //0x18
    rr { src: Reg8::C }, //0x19
    rr { src: Reg8::D }, //0x1A
    rr { src: Reg8::E }, //0x1B
    rr { src: Reg8::H }, //0x1C
    rr { src: Reg8::L }, //0x1D
    rr_ind, //0x1E
    rr { src: Reg8::A }, //0x1F
    sla { src: Reg8::B }, //0x20
    sla { src: Reg8::C }, //0x21
    sla { src: Reg8::D }, //0x22
    sla { src: Reg8::E }, //0x23
    sla { src: Reg8::H }, //0x24
    sla { src: Reg8::L }, //0x25
    sla_ind, //0x26
    sla { src: Reg8::A }, //0x27
    sra { src: Reg8::B }, //0x28
    sra { src: Reg8::C }, //0x29
    sra { src: Reg8::D }, //0x2A
    sra { src: Reg8::E }, //0x2B
    sra { src: Reg8::H }, //0x2C
    sra { src: Reg8::L }, //0x2D
    sra_ind, //0x2E
    sra { src: Reg8::A }, //0x2F
    swap { src: Reg8::B }, //0x30
    swap { src: Reg8::C }, //0x31
    swap { src: Reg8::D }, //0x32
    swap { src: Reg8::E }, //0x33
    swap { src: Reg8::H }, //0x34
    swap { src: Reg8::L }, //0x35
    swap_ind, //0x36
    swap { src: Reg8::A }, //0x37
    srl { src: Reg8::B }, //0x38
    srl { src: Reg8::C }, //0x39
    srl { src: Reg8::D }, //0x3A
    srl { src: Reg8::E }, //0x3B
    srl { src: Reg8::H }, //0x3C
    srl { src: Reg8::L }, //0x3D
    srl_ind, //0x3E
    srl { src: Reg8::A }, //0x3F
    bit { src: Reg8::B, bit: 0 }, //0x40
    bit { src: Reg8::C, bit: 0 }, //0x41
    bit { src: Reg8::D, bit: 0 }, //0x42
    bit { src: Reg8::E, bit: 0 }, //0x43
    bit { src: Reg8::H, bit: 0 }, //0x44
    bit { src: Reg8::L, bit: 0 }, //0x45
    bit_ind { bit: 0 }, //0x46
    bit { src: Reg8::A, bit: 0 }, //0x47
    bit { src: Reg8::B, bit: 1 }, //0x48
    bit { src: Reg8::C, bit: 1 }, //0x49
    bit { src: Reg8::D, bit: 1 }, //0x4A
    bit { src: Reg8::E, bit: 1 }, //0x4B
    bit { src: Reg8::H, bit: 1 }, //0x4C
    bit { src: Reg8::L, bit: 1 }, //0x4D
    bit_ind { bit: 1 }, //0x4E
    bit { src: Reg8::A, bit: 1 }, //0x4F
    bit { src: Reg8::B, bit: 2 }, //0x50
    bit { src: Reg8::C, bit: 2 }, //0x51
    bit { src: Reg8::D, bit: 2 }, //0x52
    bit { src: Reg8::E, bit: 2 }, //0x53
    bit { src: Reg8::H, bit: 2 }, //0x54
    bit { src: Reg8::L, bit: 2 }, //0x55
    bit_ind { bit: 2 }, //0x56
    bit { src: Reg8::A, bit: 2 }, //0x57
    bit { src: Reg8::B, bit: 3 }, //0x58
    bit { src: Reg8::C, bit: 3 }, //0x59
    bit { src: Reg8::D, bit: 3 }, //0x5A
    bit { src: Reg8::E, bit: 3 }, //0x5B
    bit { src: Reg8::H, bit: 3 }, //0x5C
    bit { src: Reg8::L, bit: 3 }, //0x5D
    bit_ind { bit: 3 }, //0x5E
    bit { src: Reg8::A, bit: 3 }, //0x5F
    bit { src: Reg8::B, bit: 4 }, //0x60
    bit { src: Reg8::C, bit: 4 }, //0x61
    bit { src: Reg8::D, bit: 4 }, //0x62
    bit { src: Reg8::E, bit: 4 }, //0x63
    bit { src: Reg8::H, bit: 4 }, //0x64
    bit { src: Reg8::L, bit: 4 }, //0x65
    bit_ind { bit: 4 }, //0x66
    bit { src: Reg8::A, bit: 4 }, //0x67
    bit { src: Reg8::B, bit: 5 }, //0x68
    bit { src: Reg8::C, bit: 5 }, //0x69
    bit { src: Reg8::D, bit: 5 }, //0x6A
    bit { src: Reg8::E, bit: 5 }, //0x6B
    bit { src: Reg8::H, bit: 5 }, //0x6C
    bit { src: Reg8::L, bit: 5 }, //0x6D
    bit_ind { bit: 5 }, //0x6E
    bit { src: Reg8::A, bit: 5 }, //0x6F
    bit { src: Reg8::B, bit: 6 }, //0x70
    bit { src: Reg8::C, bit: 6 }, //0x71
    bit { src: Reg8::D, bit: 6 }, //0x72
    bit { src: Reg8::E, bit: 6 }, //0x73
    bit { src: Reg8::H, bit: 6 }, //0x74
    bit { src: Reg8::L, bit: 6 }, //0x75
    bit_ind { bit: 6 }, //0x76
    bit { src: Reg8::A, bit: 6 }, //0x77
    bit { src: Reg8::B, bit: 7 }, //0x78
    bit { src: Reg8::C, bit: 7 }, //0x79
    bit { src: Reg8::D, bit: 7 }, //0x7A
    bit { src: Reg8::E, bit: 7 }, //0x7B
    bit { src: Reg8::H, bit: 7 }, //0x7C
    bit { src: Reg8::L, bit: 7 }, //0x7D
    bit_ind { bit: 7 }, //0x7E
    bit { src: Reg8::A, bit: 7 }, //0x7F
    res { src: Reg8::B, bit: 0 }, //0x80
    res { src: Reg8::C, bit: 0 }, //0x81
    res { src: Reg8::D, bit: 0 }, //0x82
    res { src: Reg8::E, bit: 0 }, //0x83
    res { src: Reg8::H, bit: 0 }, //0x84
    res { src: Reg8::L, bit: 0 }, //0x85
    res_ind { bit: 0 }, //0x86
    res { src: Reg8::A, bit: 0 }, //0x87
    res { src: Reg8::B, bit: 1 }, //0x88
    res { src: Reg8::C, bit: 1 }, //0x89
    res { src: Reg8::D, bit: 1 }, //0x8A
    res { src: Reg8::E, bit: 1 }, //0x8B
    res { src: Reg8::H, bit: 1 }, //0x8C
    res { src: Reg8::L, bit: 1 }, //0x8D
    res_ind { bit: 1 }, //0x8E
    res { src: Reg8::A, bit: 1 }, //0x8F
    res { src: Reg8::B, bit: 2 }, //0x90
    res { src: Reg8::C, bit: 2 }, //0x91
    res { src: Reg8::D, bit: 2 }, //0x92
    res { src: Reg8::E, bit: 2 }, //0x93
    res { src: Reg8::H, bit: 2 }, //0x94
    res { src: Reg8::L, bit: 2 }, //0x95
    res_ind { bit: 2 }, //0x96
    res { src: Reg8::A, bit: 2 }, //0x97
    res { src: Reg8::B, bit: 3 }, //0x98
    res { src: Reg8::C, bit: 3 }, //0x99
    res { src: Reg8::D, bit: 3 }, //0x9A
    res { src: Reg8::E, bit: 3 }, //0x9B
    res { src: Reg8::H, bit: 3 }, //0x9C
    res { src: Reg8::L, bit: 3 }, //0x9D
    res_ind { bit: 3 }, //0x9E
    res { src: Reg8::A, bit: 3 }, //0x9F
    res { src: Reg8::B, bit: 4 }, //0xA0
    res { src: Reg8::C, bit: 4 }, //0xA1
    res { src: Reg8::D, bit: 4 }, //0xA2
    res { src: Reg8::E, bit: 4 }, //0xA3
    res { src: Reg8::H, bit: 4 }, //0xA4
    res { src: Reg8::L, bit: 4 }, //0xA5
    res_ind { bit: 4 }, //0xA6
    res { src: Reg8::A, bit: 4 }, //0xA7
    res { src: Reg8::B, bit: 5 }, //0xA8
    res { src: Reg8::C, bit: 5 }, //0xA9
    res { src: Reg8::D, bit: 5 }, //0xAA
    res { src: Reg8::E, bit: 5 }, //0xAB
    res { src: Reg8::H, bit: 5 }, //0xAC
    res { src: Reg8::L, bit: 5 }, //0xAD
    res_ind { bit: 5 }, //0xAE
    res { src: Reg8::A, bit: 5 }, //0xAF
    res { src: Reg8::B, bit: 6 }, //0xB0
    res { src: Reg8::C, bit: 6 }, //0xB1
    res { src: Reg8::D, bit: 6 }, //0xB2
    res { src: Reg8::E, bit: 6 }, //0xB3
    res { src: Reg8::H, bit: 6 }, //0xB4
    res { src: Reg8::L, bit: 6 }, //0xB5
    res_ind { bit: 6 }, //0xB6
    res { src: Reg8::A, bit: 6 }, //0xB7
    res { src: Reg8::B, bit: 7 }, //0xB8
    res { src: Reg8::C, bit: 7 }, //0xB9
    res { src: Reg8::D, bit: 7 }, //0xBA
    res { src: Reg8::E, bit: 7 }, //0xBB
    res { src: Reg8::H, bit: 7 }, //0xBC
    res { src: Reg8::L, bit: 7 }, //0xBD
    res_ind { bit: 7 }, //0xBE
    res { src: Reg8::A, bit: 7 }, //0xBF
    set { src: Reg8::B, bit: 0 }, //0xC0
    set { src: Reg8::C, bit: 0 }, //0xC1
    set { src: Reg8::D, bit: 0 }, //0xC2
    set { src: Reg8::E, bit: 0 }, //0xC3
    set { src: Reg8::H, bit: 0 }, //0xC4
    set { src: Reg8::L, bit: 0 }, //0xC5
    set_ind { bit: 0 }, //0xC6
    set { src: Reg8::A, bit: 0 }, //0xC7
    set { src: Reg8::B, bit: 1 }, //0xC8
    set { src: Reg8::C, bit: 1 }, //0xC9
    set { src: Reg8::D, bit: 1 }, //0xCA
    set { src: Reg8::E, bit: 1 }, //0xCB
    set { src: Reg8::H, bit: 1 }, //0xCC
    set { src: Reg8::L, bit: 1 }, //0xCD
    set_ind { bit: 1 }, //0xCE
    set { src: Reg8::A, bit: 1 }, //0xCF
    set { src: Reg8::B, bit: 2 }, //0xD0
    set { src: Reg8::C, bit: 2 }, //0xD1
    set { src: Reg8::D, bit: 2 }, //0xD2
    set { src: Reg8::E, bit: 2 }, //0xD3
    set { src: Reg8::H, bit: 2 }, //0xD4
    set { src: Reg8::L, bit: 2 }, //0xD5
    set_ind { bit: 2 }, //0xD6
    set { src: Reg8::A, bit: 2 }, //0xD7
    set { src: Reg8::B, bit: 3 }, //0xD8
    set { src: Reg8::C, bit: 3 }, //0xD9
    set { src: Reg8::D, bit: 3 }, //0xDA
    set { src: Reg8::E, bit: 3 }, //0xDB
    set { src: Reg8::H, bit: 3 }, //0xDC
    set { src: Reg8::L, bit: 3 }, //0xDD
    set_ind { bit: 3 }, //0xDE
    set { src: Reg8::A, bit: 3 }, //0xDF
    set { src: Reg8::B, bit: 4 }, //0xE0
    set { src: Reg8::C, bit: 4 }, //0xE1
    set { src: Reg8::D, bit: 4 }, //0xE2
    set { src: Reg8::E, bit: 4 }, //0xE3
    set { src: Reg8::H, bit: 4 }, //0xE4
    set { src: Reg8::L, bit: 4 }, //0xE5
    set_ind { bit: 4 }, //0xE6
    set { src: Reg8::A, bit: 4 }, //0xE7
    set { src: Reg8::B, bit: 5 }, //0xE8
    set { src: Reg8::C, bit: 5 }, //0xE9
    set { src: Reg8::D, bit: 5 }, //0xEA
    set { src: Reg8::E, bit: 5 }, //0xEB
    set { src: Reg8::H, bit: 5 }, //0xEC
    set { src: Reg8::L, bit: 5 }, //0xED
    set_ind { bit: 5 }, //0xEE
    set { src: Reg8::A, bit: 5 }, //0xEF
    set { src: Reg8::B, bit: 6 }, //0xF0
    set { src: Reg8::C, bit: 6 }, //0xF1
    set { src: Reg8::D, bit: 6 }, //0xF2
    set { src: Reg8::E, bit: 6 }, //0xF3
    set { src: Reg8::H, bit: 6 }, //0xF4
    set { src: Reg8::L, bit: 6 }, //0xF5
    set_ind { bit: 6 }, //0xF6
    set { src: Reg8::A, bit: 6 }, //0xF7
    set { src: Reg8::B, bit: 7 }, //0xF8
    set { src: Reg8::C, bit: 7 }, //0xF9
    set { src: Reg8::D, bit: 7 }, //0xFA
    set { src: Reg8::E, bit: 7 }, //0xFB
    set { src: Reg8::H, bit: 7 }, //0xFC
    set { src: Reg8::L, bit: 7 }, //0xFD
    set_ind { bit: 7 }, //0xFE
    set { src: Reg8::A, bit: 7 }, //0xFF
];

pub fn cycles(opcode: &Op) -> u8 {
    match *opcode {
        nop | inc8_reg {..} | dec8_reg {..} | rlca | rrca | stop 
        | rla | rra | daa | cpl | inc8_ind | scf | ccf | ld8_rr {..} 
        | halt | add8_reg {..} | adc8_reg {..} | sub8_reg {..} | sbc8_reg {..} 
        | and8_reg {..} | xor8_reg {..} | or8_reg {..} | cp8_reg {..} 
        | di | ei => 1,
        st8_ind { dest: Addr::BC, src: _ } | inc16_reg {..} | ld8_imm {..} 
        | add16_reg {..} | ld8_ind { dest: _, src: Addr::BC } | dec16_reg {..} 
        | st8_ind { dest: Addr::DE, src: _ } | ld8_ind { dest: _, src: Addr::DE } 
        | st8_ind { dest: Addr::HLI, src: _ } | ld8_ind { dest: _, src: Addr::HLI } 
        | st8_ind { dest: Addr::HLD, src: _ } | ld8_ind { dest: _, src: Addr::HLD } 
        | ld8_ind { dest: _, src: Addr::HL } | st8_ind { dest: Addr::HL, src: _ } 
        | add8_ind | adc8_ind | sub8_ind | sbc8_ind | and8_ind | xor8_ind 
        | or8_ind | cp8_ind | pop16 {..} | add8_imm | ret_cond {..} 
        | adc8_imm | sub8_imm | sbc8_imm | out8_reg | and8_imm | xor8_imm 
        | in8_reg | or8_imm | ld16_sp | cp8_imm | rlc {..} | rrc {..} 
        | rl {..} | rr {..} | sla {..} | sra {..} | swap {..} | srl {..} 
        | bit {..} | res {..} | set {..} => 2,
        ld16_imm {..} | dec8_ind | st8_ind_imm | out8_imm | in8_imm 
        | ld16_lea => 3,
        push16 {..} | rst {..} | add8_sp_imm | st8_ind { dest: Addr::Imm, src: _ } 
        | ld8_ind { dest: _, src: Addr::Imm } | rlc_ind | rrc_ind 
        | rl_ind | rr_ind | sla_ind | sra_ind | swap_ind | srl_ind 
        | bit_ind {..} | res_ind {..} | set_ind {..} => 4,
        st16_sp => 5,
        _ => cycles_jmp(opcode, false)
    }
}

pub fn cycles_jmp(opcode: &Op, jumped: bool) -> u8 {
    if jumped {
        match *opcode {
            jp_ind => 1,
            jp_rel | jp_rel_cond {..} => 3,
            jp_cond {..} | jp | ret | reti => 4,
            ret_cond {..} => 5,
            call_cond {..} | call => 6,
            _ => panic!("Trying to get cycles of unknown opcode {:?}", opcode)
        }
    } else {
        match *opcode {
            jp_ind => 1,
            jp_rel_cond {..} | ret_cond {..} => 2,
            jp_rel | jp_cond {..} | call_cond {..} => 3,
            jp | ret | reti => 4,
            call => 6,
            _ => panic!("Trying to get cycles of unknown opcode {:?}", opcode)
        }
    }
}

