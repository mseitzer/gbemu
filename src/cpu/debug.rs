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
		self.regs.pc
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
			if db.breakpoints.contains(&self.regs.pc) {
				let orig_pc = self.regs.pc;
				let instr = self.fetch_instr();
				db.cur_instr = instr;
				self.regs.pc = orig_pc;
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