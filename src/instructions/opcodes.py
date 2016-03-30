#!/usr/bin/env python3

# Quick'n' dirty generator script for opcodes.rs

none = "None"
imm8 = "IMM8"
imm16 = "IMM16"

a = "A"
b = "B"
c = "C"
d = "D"
e = "E"
f = "F"
h = "H"
l = "L"
af = "AF"
bc = "BC"
de = "DE"
hl = "HL"
sp = "SP"

instr_cycles = {}
cycles_jmp_not_taken = {}
cycles_jmp_taken = {}

out = """/* THIS IS AN AUTOGENERATED FILE, DO NOT EDIT */

use super::registers::{Reg8, Reg16};
use super::instructions::Op;
use super::instructions::Op::*;
use super::instructions::Condition;
use super::instructions::Immediate;
use super::instructions::Immediate::*;

const IMM8: Immediate = Immediate::Imm8(0);
const IMM16: Immediate = Immediate::Imm16(0);

pub const OPCODES: [(Op, Immediate); 256] = [
"""

for code in range(0, 256):
    op = dest = src = cond = target = ""
    imm = none
    dbits = sbits = 8

    cycles = 0
    jcycles = 0 # jmp instructions cycles on branch

    lo = code & 0x0f
    hi = code >> 4

    if lo == 0 and 0 <= hi <= 1:
        op = ["nop", "stop"][hi]
        cycles = 1
    elif lo == 0 and 2 <= hi <= 3:
        op = "jp_rel_cond"
        cond = ["NZ", "NC"][hi-2]
        imm = imm8
        cycles = 2
        jcycles = 3

    elif lo == 1 and 0 <= hi <= 2:
        op = "ld16_imm"
        dest = [bc, de, hl][hi]
        dbits = 16
        imm = imm16
        cycles = 3
    elif lo == 1 and hi == 3:
        op = "ld16_sp_imm"
        imm = imm16
        cycles = 3

    elif lo == 2 and 0 <= hi <= 1:
        op = "st8_ind_r16"
        dest = [bc, de][hi]
        dbits = 16
        cycles = 2
    elif lo == 2 and hi == 2:
        op = "st8_ind_inc"
        cycles = 2
    elif lo == 2 and hi == 3:
        op = "st8_ind_dec"
        cycles = 2

    elif lo == 3 and 0 <= hi <= 2:
        op = "inc16_reg"
        src = [bc, de, hl][hi]
        sbits = 16
        cycles = 2
    elif lo == 3 and hi == 3:
        op = "inc16_sp"
        cycles = 2

    elif lo == 4 and 0 <= hi <= 2:
        op = "inc8_reg"
        src = [b, d, h][hi]
        cycles = 1
    elif lo == 4 and hi == 3:
        op = "inc8_ind"
        cycles = 1

    elif lo == 5 and 0 <= hi <= 2:
        op = "dec8_reg"
        src = [b, d, h][hi]
        cycles = 1
    elif lo == 5 and hi == 3:
        op = "dec8_ind"
        cycles = 3

    elif lo == 6 and 0 <= hi <= 2:
        op = "ld8_imm"
        dest = [b, d, h][hi]
        imm = imm8
        cycles = 2
    elif lo == 6 and hi == 3:
        op = "st8_ind_imm"
        imm = imm8
        cycles = 3

    elif lo == 7 and 0 <= hi <= 3:
        op = ["rlca", "rla", "daa", "scf"][hi]
        cycles = 1

    elif lo == 8 and hi == 0:
        op = "st16_sp"
        imm = imm16
        cycles = 5
    elif lo == 8 and hi == 1:
        op = "jp_rel"
        imm = imm8
        cycles = jcycles = 3
    elif lo == 8 and 2 <= hi <= 3:
        op = "jp_rel_cond"
        cond = ["Z", "C"][hi-2]
        imm = imm8
        cycles = 2
        jcycles = 3

    elif lo == 9 and 0 <= hi <= 2:
        op = "add16_reg"
        src = [bc, de, hl][hi]
        sbits = 16
        cycles = 2
    elif lo == 9 and hi == 3:
        op = "add16_sp"
        cycles = 2

    elif lo == 0xA and 0 <= hi <= 1:
        op = "ld8_ind_r16"
        src = [bc, de][hi]
        sbits = 16
        cycles = 2
    elif lo == 0xA and 2 <= hi <= 3:
        op = ["ld8_ind_inc", "ld8_ind_dec"][hi-2]
        cycles = 2

    elif lo == 0xB and 0 <= hi <= 2:
        op = "dec16_reg"
        src = [bc, de, hl][hi]
        sbits = 16
        cycles = 2
    elif lo == 0xB and hi == 3:
        op = "dec16_sp"
        cycles = 2

    elif lo == 0xC and 0 <= hi <= 3:
        op = "inc8_reg"
        src = [c, e, l, a][hi]
        cycles = 1

    elif lo == 0xD and 0 <= hi <= 3:
        op = "dec8_reg"
        src = [c, e, l, a][hi]
        cycles = 1

    elif lo == 0xE and 0 <= hi <= 3:
        op = "ld8_imm"
        dest = [c, e, l, a][hi]
        imm = imm8
        cycles = 2

    elif lo == 0xF and 0 <= hi <= 3:
        op = ["rrca", "rra", "cpl", "ccf"][hi]
        cycles = 1

    elif 4 <= hi <= 7:
        if lo == 6 or lo == 0xE:
            op = "ld8_ind_r16"
            src = "HL"
            sbits = 16
            cycles = 2
        else:
            op = "ld8_rr"
            src = (2*[b, c, d, e, h, l, hl, a])[lo]
            cycles = 1

        dest = [[b, c], [d, e], [h, l], [hl, a]][hi-4][0 if 0 <= lo <= 6 else 1]
        if hi == 7 and 0 <= lo <= 5:
            op = "st8_ind_r16"
            dbits = 16
            src = ""
            cycles = 2
        if lo == 6 or lo == 0xE:
            dest = ""
    
    elif hi == 7 and lo == 6:
        op = "halt"
        cycles = 1

    elif 8 <= hi <= 0xB:
        op = [["add8_", "adc8_"], ["sub8_", "sbc8_"], 
            ["and8_", "xor8_"], ["or8_", "cp8_"]][hi-8][0 if 0 <= lo <= 7 else 1]

        if lo == 6 or lo == 0xE:
            op += "ind"
            cycles = 2
        else:
            op += "reg"
            src = (2*[b, c, d, e, h, l, hl, a])[lo]
            cycles = 1

    elif lo == 0 and 0xC <= hi <= 0xD:
        op = "ret_cond"
        cond = ["NZ", "NC"][hi-0xC]
        cycles = 2
        jcycles = 5

    elif lo == 0 and 0xE <= hi <= 0xF:
        op = ["out8_imm", "in8_imm"][hi-0xE]
        imm = imm8
        cycles = 3

    elif lo == 1 and 0xC <= hi <= 0xF:
        op = "pop16"
        dest = [bc, de, hl, af][hi-0xC]
        dbits = 16
        cycles = 2

    elif lo == 2 and 0xC <= hi <= 0xD:
        op = "jp_cond"
        cond = ["NZ", "NC"][hi-0xC]
        imm = imm16
        cycles = 3
        jcycles = 4

    elif lo == 2 and 0xE <= hi <= 0xE:
        op = ["out8_ofs", "in8_ofs"][hi-0xE]
        cycles = 2

    elif lo == 3 and hi == 0xC:
        op = "jp"
        imm = imm16
        cycles = jcycles = 4

    elif lo == 3 and hi == 0xF:
        op = "di"
        cycles = 1

    elif lo == 4 and 0xC <= hi <= 0xD:
        op = "call_cond"
        cond = ["NZ", "NC"][hi-0xC]
        imm = imm16
        cycles = 3
        jcycles = 6

    elif lo == 5 and 0xC <= hi <= 0xF:
        op = "push16"
        src = [bc, de, hl, af][hi-0xC]
        sbits = 16
        cycles = 4

    elif lo == 6 and 0xC <= hi <= 0xF:
        op = ["add8_imm", "sub8_imm", "and8_imm", "or8_imm"][hi-0xC]
        imm = imm8
        cycles = 2

    elif lo == 7 and 0xC <= hi <= 0xF:
        op = "rst"
        target = 16 * (hi-0xC)
        cycles = 4

    elif lo == 8 and 0xC <= hi <= 0xD:
        op = "ret_cond"
        cond = ["Z", "C"][hi-0xC]
        cycles = 2
    elif lo == 8 and hi == 0xE:
        op = "add8_sp_imm"
        imm = imm8
        cycles = 4

    elif lo == 9 and 0xC <= hi <= 0xE:
        op = ["ret", "reti", "jp_ind"][hi-0xC]
        cycles = jcycles = [4, 4, 1][hi-0xC]

    elif 8 <= lo <= 9 and hi == 0xF:
        op = ["ld16_lea", "ld16_sp"][lo-8]
        cycles = [3, 2][lo-8]

    elif lo == 0xA and 0xC <= hi <= 0xD:
        op = "jp_cond"
        cond = ["Z", "C"][hi-0xC]
        imm = imm16
        cycles = 3
        jcycles = 4

    elif lo == 0xA and 0xE <= hi <= 0xF:
        op = ["st8_ind_imm16", "ld8_ind_imm16"][hi-0xE]
        imm = imm16
        cycles = 4

    elif lo == 0xB and hi == 0xC:
        op = "ext"

    elif lo == 0xB and hi == 0xF:
        op = "ei"
        cycles = 1

    elif lo == 0xC and 0xC <= hi <= 0xD:
        op = "call_cond"
        cond = ["Z", "C"][hi-0xC]
        imm = imm16
        cycles = 3
        jcycles = 6

    elif lo == 0xD and hi == 0xC:
        op = "call"
        imm = imm16
        cycles = jcycles = 6

    elif lo == 0xE and 0xC <= hi <= 0xF:
        op = ["adc8_imm", "sbc8_imm", "xor8_imm", "cp8_imm"][hi-0xC]
        imm = imm8
        cycles = 2

    elif lo == 0xF and 0xC <= hi <= 0xF:
        op = "rst"
        target = 16 * (hi-0xC) + 0x8
        cycles = 4

    else:
        op = "inv"

    if op != "":
        out += "    ({0}".format(op)

        if dest != "" or src != "":
            out += " {"
            if dest != "":
                out += " dest: Reg{0}::{1}".format(dbits, dest)
                if src != "":
                    out += ", src: Reg{0}::{1}".format(sbits, src)
            
            elif src != "":
                out += " src: Reg{0}::{1}".format(sbits, src)

            out += " }"
        elif cond != "":
            out += " {{ cond: Condition::{0} }}".format(cond)
        elif target != "":
            out += " {{ target: 0x{0:02X} }}".format(target)

        out += ", {0}), //0x{1:02X}\n".format(imm, code)

    if op != "":
        has_operands = False
        if dest != "" or src != "" or cond != "" or target != "":
            has_operands = True

        if jcycles == 0:
            if cycles > 0:
                instr_list = instr_cycles.setdefault(cycles, [])
                if not (op, has_operands) in instr_list: instr_list.append((op, has_operands))
        else:
            if cycles > 0:
                instr_list = cycles_jmp_not_taken.setdefault(cycles, [])
                if not (op, has_operands) in instr_list: instr_list.append((op, has_operands))
                instr_list= cycles_jmp_taken.setdefault(jcycles, [])
                if not (op, has_operands) in instr_list: instr_list.append((op, has_operands))

out += """];

pub const EXT_OPCODES: [Op; 256] = [
"""

for code in range(0, 256):
    src = bit = ""
    cycles = 2

    lo = code & 0x0f
    hi = code >> 4

    op = [
        ["rlc", "rrc"], ["rl", "rr"], ["sla", "sra"], ["swap", "srl"],
        ["bit", "bit"], ["bit", "bit"], ["bit", "bit"], ["bit", "bit"],
        ["res", "res"], ["res", "res"], ["res", "res"], ["res", "res"],
        ["set", "set"], ["set", "set"], ["set", "set"], ["set", "set"]
    ][hi][0 if 0 <= lo <= 7 else 1]

    src = (2*[b, c, d, e, h, l, hl, a])[lo]
    if lo == 6 or lo == 0xE:
        op += "_ind"
        src = ""
        cycles = 4

    if 0x4 <= hi <= 0xF:
        bit = (code & 0b00111000) >> 3

    out += "    {0}".format(op)
    if src != "":
        out += " {"
        out += " src: Reg8::{0}".format(src)
        if bit != "":
            out += ", bit: {}".format(bit)
        out += " }"
    elif bit != "":
        out += " {{ bit: {} }}".format(bit)

    out += ", //0x{0:02X}\n".format(code)

    if op != "":
        has_operands = False
        if bit != "" or src != "":
            has_operands = True

        if cycles > 0:
            instr_list = instr_cycles.setdefault(cycles, [])
            if not (op, has_operands) in instr_list: instr_list.append((op, has_operands))

out += "];\n"


def generate_match_arms(cycles_dict, indent_lvl):
    res = ""
    for (cycles, instr_list) in sorted(cycles_dict.items()):
        line = "\n" + "    " * (indent_lvl + 1)
        first = True
        for (instr, has_operands) in instr_list:
            if len(line) > 65:
                res += line + "\n"
                line = "    " * (indent_lvl + 1)
            if not first:
                line += "| "
            else:
                first = False
            line += instr + " "
            if has_operands: line += "{..} "
        res += line + "=> {0},".format(cycles)
    return res

out += '''
pub fn cycles(opcode: &Op) -> u8 {
    match *opcode {'''
out += generate_match_arms(instr_cycles, 1)
out += '''
        _ => cycles_jmp(opcode, false)
    }
}
'''

out += '''
pub fn cycles_jmp(opcode: &Op, jumped: bool) -> u8 {
    if jumped {
        match *opcode {'''
out += generate_match_arms(cycles_jmp_taken, 2)
out += '''
            _ => panic!("Trying to get cycles of unknown opcode {:?}", opcode)
        }
    } else {
        match *opcode {'''
out += generate_match_arms(cycles_jmp_not_taken, 2)
out += '''
            _ => panic!("Trying to get cycles of unknown opcode {:?}", opcode)
        }
    }
}
'''

print(out)