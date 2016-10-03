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
const ROM_BANK_SIZE: usize = 16384;
const RAM_BANK_SIZE: usize = 8192;

bitflags! {
    flags CartridgeFeatures: u32 {
        const ROM = 1 << 0,
        const MBC1 = 1 << 1,
        const MBC2 = 1 << 2,
        const MBC3 = 1 << 3,
        const MBC5 = 1 << 4,
        const RAM = 1 << 5,
        const MMM01 = 1 << 6,
        const BATTERY = 1 << 7,
        const SRAM = 1 << 8,
        const RUMBLE = 1 << 9,
    }
}

impl CartridgeFeatures {
    fn from_type_byte(value: u8) -> CartridgeFeatures {
        match value {
            0x00 => ROM,
            0x01 => ROM | MBC1,
            0x02 => ROM | MBC1 | RAM,
            0x03 => ROM | MBC1 | RAM | BATTERY,
            0x05 => ROM | MBC2,
            0x06 => ROM | MBC2 | BATTERY,
            0x08 => ROM | RAM,
            0x09 => ROM | RAM | BATTERY,
            0x0B => ROM | MMM01,
            0x0C => ROM | MMM01 | SRAM,
            0x0D => ROM | MMM01 | SRAM | BATTERY,
            0x12 => ROM | MBC3 | RAM,
            0x13 => ROM | MBC3 | RAM | BATTERY,
            0x19 => ROM | MBC5,
            0x1A => ROM | MBC5 | RAM,
            0x1B => ROM | MBC5 | RAM | BATTERY,
            0x1C => ROM | MBC5 | RUMBLE,
            0x1D => ROM | MBC5 | RUMBLE | SRAM,
            0x1E => ROM | MBC5 | RUMBLE | SRAM | BATTERY,
            _    => panic!("Unsupported cartridge type")
        }
    }
}

#[derive(Debug)]
enum MBC {
    None, MBC1, MBC2, MBC3
}

impl MBC {
    fn from_features(features: CartridgeFeatures) -> MBC {
        if features.contains(MBC1) {
            MBC::MBC1
        } else if features.contains(MBC2) {
            MBC::MBC2
        } else if features.contains(MBC3) {
            MBC::MBC3
        } else {
            MBC::None
        }
    }
}

pub struct Cartridge {
    rom: Box<[u8]>,
    rom_bank: u8,
    ram: Vec<u8>,
    ram_bank: u8,
    ram_mode: bool,
    ram_enabled: bool,
    mbc: MBC
}

impl Cartridge {
    pub fn new(rom: Box<[u8]>) -> Cartridge {
        let features = CartridgeFeatures::from_type_byte(rom[0x147]);
        let mbc = MBC::from_features(features);
        let ram_size = match mbc {
            MBC::None => 0,
            MBC::MBC1 => 32768,
            _ => panic!("Unsupported MBC type")
        };

        Cartridge {
            rom: rom,
            rom_bank: 1,
            ram: vec![0; ram_size],
            ram_bank: 0,
            ram_mode: false,
            ram_enabled: false,
            mbc: mbc
        }
    }

    pub fn read_rom_bank0(&self, addr: u16) -> u8 {
        self.rom[addr as usize]
    }

    pub fn read_rom_bank1(&self, addr: u16) -> u8 {
        assert!(self.rom_bank >= 1);
        match self.mbc {
            MBC::None => self.rom[ROM_BANK_SIZE + addr as usize],
            MBC::MBC1 => self.rom[ROM_BANK_SIZE * self.rom_bank as usize + addr as usize],
            _ => panic!("Unsupported MBC type")
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        match self.mbc {
            MBC::None => {},
            MBC::MBC1 => {
                match addr {
                    0x0000...0x1FFF if value == 0x0A => self.ram_enabled = true,
                    0x0000...0x1FFF => self.ram_enabled = false,
                    0x2000...0x3FFF if value == 0 => {
                        self.rom_bank &= 0b11100000;
                        self.rom_bank |= 1;
                    },
                    0x2000...0x3FFF => {
                        self.rom_bank &= 0b11100000;
                        self.rom_bank |= value & 0b11111;
                    },
                    0x4000...0x5FFF if !self.ram_mode => {
                        self.rom_bank &= 0b10011111;
                        self.rom_bank |= (value & 0b11) << 5;
                    },
                    0x4000...0x5FFF => self.ram_bank = value & 0b11,
                    0x6000...0x7FFF if value == 0 => self.ram_mode = false,
                    0x6000...0x7FFF if value == 1 => self.ram_mode = true,
                    _ => panic!("Invalid write to cartridge location")
                }
            },
            _ => panic!("Unsupported MBC type")
        }
    }

    pub fn read_ram(&self, addr: u16) -> u8 {
        if self.ram_enabled && self.ram.len() > 0 {
            self.ram[RAM_BANK_SIZE * self.ram_bank as usize + addr as usize]
        } else {
            0x00
        }
    }

    pub fn write_ram(&mut self, addr: u16, value: u8) {
        if self.ram_enabled && self.ram.len() > 0 {
            self.ram[RAM_BANK_SIZE * self.ram_bank as usize + addr as usize] = value;
        }
    }
}