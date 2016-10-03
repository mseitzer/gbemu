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
use cpu;
use hardware;
use gpu::{self, Color, SCREEN_WIDTH};
use instructions::Instr;
use cpu::debug::DebugInfo;

use std::io::{self, Write};

fn print_help() {
    println!("Commands:");
    println!("help: Print this help");
    println!("s|step: Single step");
    println!("c|continue: Continue to next breakpoint");
    println!("break <addr>: Set breakpoint to address <addr>");
    println!("rm|remove <addr>: Remove breakpoint from address <addr>");
    println!("[print ]instr: Print current instruction");
    println!("[print ]cpu: Print current CPU state");
    println!("read <addr>: Read memory address <addr>");
    println!("auto <instr|cpu>: Automatically print item after instructions");
}

fn print_instr(addr: u16, instr: &Instr) {
    println!("{:#06x}: {}", addr, instr);
}

pub fn print_framebuffer(framebuffer: &gpu::Framebuffer) {
    for i in 0..144 {
        for j in 0..160 {
            let ch = match framebuffer[i*SCREEN_WIDTH+j] {
                Color::Black => '■',
                Color::DarkGray => '▩',
                Color::LightGray => '▥',
                Color::White => ' ',
            };
            print!("{}", ch);
        }
        print!("\n");
    }
    print!("\n");
}

pub fn start(bios: Box<[u8]>, rom: Box<[u8]>) {
    let hardware = hardware::Hardware::new(bios, rom);
    let mut cpu = cpu::Cpu::new(hardware);
    let mut db = DebugInfo::new();

    let mut last_input = String::new();
    let mut cur_pc = cpu.get_pc();
    let mut hit_breakpoint = false;
    let mut print_instr_flag = false;
    let mut print_cpu_flag = false;

    loop {
        print!("{:#06x}> ", cur_pc);
        io::stdout().flush().ok().expect("Could not flush stdout");

        let mut print_items = false;
        let mut input = String::new();

        if let Ok(size) = io::stdin().read_line(&mut input) {
            if size == 0 { // EOF
                break;
            }

            input = String::from(input.trim());
            if input.len() == 0 {
                input = last_input.clone();
                if input.len() == 0 {
                    continue;
                }
                println!("{}", input);
            }

            if input.starts_with("help") {
                print_help();
            } else if input == "s" || input.starts_with("step") {
                cur_pc = cpu.get_pc();
                cpu.single_step(&mut db);
                print_items = true;
            } else if input == "c" || input.starts_with("continue") {
                if hit_breakpoint {
                    // Single step over the breakpoint
                    cpu.single_step(&mut db);
                    hit_breakpoint = false;
                }

                println!("Continuing.");
                cpu.continue_exec(&mut db);
                if db.contains_breakpoint(cpu.get_pc()) {
                    println!("Hit breakpoint at {:#06x}", cpu.get_pc());
                    hit_breakpoint = true;
                } else {
                    println!("Stopped continuing for unknown reasons");
                }
                cur_pc = cpu.get_pc();
                print_items = true;
            } else if input.starts_with("break ") {
                if let Some(num) = input.split_whitespace().nth(1) {
                    match u16::from_str_radix(num.trim_left_matches("0x"),
                                              16) {
                        Ok(addr) => {
                            db.add_breakpoint(addr);
                            println!("Added breakpoint at {:#06x}", addr);
                        },
                        Err(f) => {
                            println!("Could not parse breakpoint address: {}", f);
                        }
                    }
                }
            } else if input.starts_with("rm ") || input.starts_with("remove ") {
                if let Some(num) = input.split_whitespace().nth(1) {
                    match u16::from_str_radix(num.trim_left_matches("0x"),
                                              16) {
                        Ok(addr) => {
                            if db.remove_breakpoint(addr) {
                                println!("Removed breakpoint at {:#06x}", addr);
                            } else {
                                println!("No breakpoint at {:#06x}", addr);
                            }
                        },
                        Err(f) => {
                            println!("Could not parse breakpoint address: {}", f);
                        }
                    }
                }
            } else if input.starts_with("instr") {
                print_instr(cpu.get_pc(), &db.instr());
            } else if input.starts_with("cpu") {
                println!("{}", cpu);
            } else if input.starts_with("print ") {
                if let Some(item) = input.split_whitespace().nth(1) {
                    match item {
                        "instr" => print_instr(cpu.get_pc(), &db.instr()),
                        "cpu" => println!("{}", cpu),
                        _ => println!("Unsupported print item {}", item)
                    }
                }
            } else if input.starts_with("read ") {
                if let Some(num) = input.split_whitespace().nth(1) {
                    match u16::from_str_radix(num.trim().trim_left_matches("0x"),
                                              16) {
                        Ok(addr) => {
                            println!("{:#04x}", cpu.read_mem(addr));
                        },
                        Err(f) => {
                            println!("Could not parse breakpoint address: {}", f);
                        }
                    }
                }
            } else if input.starts_with("auto ") {
                if let Some(item) = input.split_whitespace().nth(1) {
                    match item {
                        "instr" => print_instr_flag ^= true,
                        "cpu" => print_cpu_flag ^= true,
                        _ => println!("Unsupported auto item {}", item)
                    }
                }
            } else if input.starts_with("screen") {
                let framebuffer = cpu.hardware().framebuffer();
                print_framebuffer(framebuffer);
            } else {
                println!("Unknown command '{}'. \
                         Try help for an overview of available commands.", 
                         input.trim());
            }

            if print_items {
                if print_instr_flag {
                    print_instr(cur_pc, &db.instr());
                }
                if print_cpu_flag {
                    println!("{}", cpu);
                }
            }

            last_input = input;
        } else {
            break;
        }
    }
}