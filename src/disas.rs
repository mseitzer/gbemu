mod instructions;

use std::env;
use std::io::prelude::*;
use std::error::Error;
use std::fs::File;

fn disassemble(binary: Vec<u8>) -> Vec<(usize, instructions::Instr)> {
    use instructions::Instr;
    use instructions::Op;
    use instructions::Immediate::*;

    let mut instr_buf = Vec::new();

    let mut iter = binary.iter().enumerate();

    while let Some((addr, &opcode)) = iter.next() {
        let instr = match instructions::from_opcode(opcode) {
            (Op::ext, None) => {
                let &ext_opcode = iter.next().unwrap().1;
                instructions::from_ext_opcode(ext_opcode)
            }
            (op, imm @ None) => Instr { op: op, imm: imm },
            (op, Imm8(_)) => {
                let &imm = iter.next().unwrap().1;
                Instr { op: op, imm: Imm8(imm) }
            },
            (op, Imm16(_)) => {
                let lo = *iter.next().unwrap().1 as u16;
                let hi = *iter.next().unwrap().1 as u16;
                Instr { op: op, imm: Imm16((hi << 8) + lo) }
            }
        };

        instr_buf.push((addr, instr));
    }

    instr_buf
}

fn main() {
    let rom_path = env::args().nth(1).unwrap();

    let mut rom_fd = match File::open(&rom_path) {
        Err(why) => panic!("Can't open file '{}': {}", rom_path, why.description()),
        Ok(f) => f
    };

    let mut rom_buf = Vec::new();
    if let Err(why) = rom_fd.read_to_end(&mut rom_buf) {
        panic!("Can't read file '{}': {}", rom_path, why.description());
    }
    
    let instructions = disassemble(rom_buf);

    for (addr, instr) in instructions {
        println!("{:#06x}: {}", addr, instr);
    }
}