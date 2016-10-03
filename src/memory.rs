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
pub struct Memory {
    // RAM: 0xC000-0xDFFF
    // Shadow copy of the RAM from 0xE000-0xFDFF
    ram: [u8; 8192],

    // Zero page RAM: 0xFF80-0xFFFF
    zram: [u8; 128],
}

impl Memory {
    pub fn new() -> Box<Memory> {
        Box::new(Memory {
            ram: [0; 8192],
            zram: [0; 128],
        })
    }

    pub fn read_ram(&self, addr: u16) -> u8 {
        self.ram[addr as usize]
    }

    pub fn read_zram(&self, addr: u16) -> u8 {
        self.zram[addr as usize]
    }

    pub fn write_ram(&mut self, addr: u16, value: u8) {
        self.ram[addr as usize] = value;
    }

    pub fn write_zram(&mut self, addr: u16, value: u8) {
        self.zram[addr as usize] = value;
    }
}