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

use cpu;
use hardware;
use joypad;
use gpu;
use events;

pub struct Gameboy {
    cpu: cpu::Cpu<hardware::Hardware>
}

impl Gameboy {
    pub fn new(bios: Box<[u8]>, rom: Box<[u8]>) -> Gameboy {
        let hardware = hardware::Hardware::new(bios, rom);

        Gameboy {
            cpu: cpu::Cpu::new(hardware)
        }
    }

    pub fn simulate(&mut self, target_cycles: u64) -> (u64, events::Events) {
        while self.cpu.total_cycles() < target_cycles {
            let events = self.cpu.step();
            if !events.is_empty() {
                return (self.cpu.total_cycles(), events)
            }
        }
        (self.cpu.total_cycles(), events::Events::empty())
    }

    pub fn framebuffer(&mut self) -> &gpu::Framebuffer {
        self.cpu.hardware().framebuffer()
    }

    pub fn press_key(&mut self, key: joypad::Key) {
        self.cpu.hardware().press_key(key);
    }

    pub fn release_key(&mut self, key: joypad::Key) {
        self.cpu.hardware().release_key(key);
    }
}

impl fmt::Display for Gameboy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.cpu)
    }
}
