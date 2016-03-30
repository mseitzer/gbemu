use super::int_controller::{Interrupt, IntController};

pub struct Timer {
	divider_reg: u8,
	counter_reg: u8,
	modulo_reg: u8,

	control_reg: u8,
	counter_rate: u8,
	active: bool,

	threshold: u8,
	clock: u64,

	divider_threshold: u8,
}

impl Timer {
	pub fn new() -> Timer {
		Timer {
			divider_reg: 0,
			counter_reg: 0,
			modulo_reg: 0,
			control_reg: 0,
			counter_rate: 1,
			active: false,
			threshold: 0,
			clock: 0,
			divider_threshold: 0
		}
	}

	pub fn tick(&mut self, cycles: u8, int_controller: &mut IntController) {
		self.threshold += cycles;

		if self.threshold >= 4 {
			// Clock ticks once
			self.clock += 1;
			self.threshold -= 4;

			// Divider clock ticks at 1/16th the rate of the main clock
			self.divider_threshold += 1;
			if self.divider_threshold == 16 {
				self.divider_reg = self.divider_reg.wrapping_add(1);
				self.divider_threshold = 0;
			}
		}

		if self.active && self.clock >= self.counter_rate as u64 {
			self.clock = 0;
			let (value, overflow) = self.counter_reg.overflowing_add(1);

			if overflow {
				self.counter_reg = self.modulo_reg;
				int_controller.set_int_pending(Interrupt::Timer);
			} else {
				self.counter_reg = value;
			}
		}
	}

	pub fn read_divider_reg(&self) -> u8 {
		self.divider_reg
	}

	pub fn write_divider_reg(&mut self, _: u8) {
		self.divider_reg = 0;
	}

	pub fn read_counter_reg(&self) -> u8 {
		self.counter_reg
	}

	pub fn write_counter_reg(&mut self, value: u8) {
		self.counter_reg = value;
	}

	pub fn read_modulo_reg(&self) -> u8 {
		self.modulo_reg
	}

	pub fn write_modulo_reg(&mut self, value: u8) {
		self.modulo_reg = value;
	}

	pub fn read_control_reg(&self) -> u8 {
		self.control_reg
	}

	pub fn write_control_reg(&mut self, value: u8) {
		self.control_reg = value;
		self.counter_rate = match value & 0b11 {
			0b00 => 64,
			0b01 => 1,
			0b10 => 4,
			0b11 => 16,
			_ => panic!("Illegal value selected for timer speed!")
		};
		self.active = value & 0b100 != 0;
	}
}