use cpu;
use hardware;
use instructions::Instr;
use cpu::debug::DebugInfo;

use std::io::{self};

fn print_instr(addr: u16, instr: &Instr) {
	println!("{:#06x}: {}", addr, instr);
}

pub fn start(bios: Box<[u8]>, rom: Box<[u8]>) {
	let hardware = hardware::Hardware::new(bios, rom);
	let mut cpu = cpu::Cpu::new(hardware);
	let mut db = DebugInfo::new();

	let mut hit_breakpoint = false;

	loop {
        let mut input = String::new();

		if let Ok(_) = io::stdin().read_line(&mut input) {
			if input == "s\n" || input.starts_with("step") {
				let pc = cpu.get_pc();
				cpu.single_step(&mut db);
				print_instr(pc, &db.instr());
			} else if input == "c\n" || input.starts_with("continue") {
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
			} else if input.starts_with("break ") {
				if let Some(num) = input.split_whitespace().nth(1) {
					match u16::from_str_radix(num.trim().trim_left_matches("0x"),
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
			} else if input.starts_with("rm ") {
				if let Some(num) = input.split_whitespace().nth(1) {
					match u16::from_str_radix(num.trim().trim_left_matches("0x"),
											  16) {
						Ok(addr) => {
							db.remove_breakpoint(addr);
							println!("Removed breakpoint at {:#06x}", addr);
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
			}
	    } else {
	    	break;
	    }
	}
}