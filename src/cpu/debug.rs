use instructions;
use super::super::hardware::Bus;

use std::collections::HashSet;

pub struct DebugInfo {
	pub cur_instr: instructions::Instr,
	pub breakpoints: HashSet<u16>
}

impl DebugInfo {
	pub fn new() -> DebugInfo {
		DebugInfo {
			cur_instr: instructions::Instr {
				op: instructions::Op::inv,
				imm: instructions::Immediate::None
			},
			breakpoints: HashSet::new()
		}
	}

	pub fn instr(&self) -> instructions::Instr {
		self.cur_instr
	}

	pub fn add_breakpoint(&mut self, addr: u16) {
		self.breakpoints.insert(addr);
	}

	pub fn remove_breakpoint(&mut self, addr: u16) -> bool {
		self.breakpoints.remove(&addr)
	}

	pub fn contains_breakpoint(&self, addr: u16) -> bool {
		self.breakpoints.contains(&addr)
	}
}

impl<B> super::Cpu<B> where B: Bus {
	pub fn get_pc(&self) -> u16 {
		self.pc
	}

	pub fn single_step(&mut self, db: &mut DebugInfo) {
		let instr = self.fetch_instr();
		db.cur_instr = instr;
		self.execute_instr(instr);

		self.handle_updates();
        self.handle_interrupts();
        self.handle_updates();
	}

	pub fn continue_exec(&mut self, db: &mut DebugInfo) {
		loop {
			if db.breakpoints.contains(&self.pc) {
				let orig_pc = self.pc;
				let instr = self.fetch_instr();
				db.cur_instr = instr;
				self.pc = orig_pc;
				break;
			}

			let instr = self.fetch_instr();
			db.cur_instr = instr;
			self.execute_instr(instr);

			self.handle_updates();
            self.handle_interrupts();
            self.handle_updates();
		}
	}

	pub fn read_mem(&self, addr: u16) -> u8 {
		self.bus.read(addr)
	}
}