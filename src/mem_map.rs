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
pub const ROM_BANK0_LO: u16 = 0x0000;
pub const ROM_BANK0_HI: u16 = 0x3FFF;
pub const ROM_BANK1_LO: u16 = 0x4000;
pub const ROM_BANK1_HI: u16 = 0x7FFF;
pub const TILE_DATA_LO: u16 = 0x8000;
pub const TILE_DATA_HI: u16 = 0x97FF;
pub const TILE_MAP1_LO: u16 = 0x9800;
pub const TILE_MAP1_HI: u16 = 0x9BFF;
pub const TILE_MAP2_LO: u16 = 0x9C00;
pub const TILE_MAP2_HI: u16 = 0x9FFF;
pub const ERAM_LO: u16 = 0xA000;
pub const ERAM_HI: u16 = 0xBFFF;
pub const RAM_LO: u16 = 0xC000;
pub const RAM_HI: u16 = 0xDFFF;
pub const RAM_LO2: u16 = 0xE000;
pub const RAM_HI2: u16 = 0xFDFF;
pub const SPRITES_LO: u16 = 0xFE00;
pub const SPRITES_HI: u16 = 0xFE9F;
pub const UNMAPPED_LO: u16 = 0xFEA0;
pub const UNMAPPED_HI: u16 = 0xFEFF;
pub const IO_LO: u16 = 0xFF00;
pub const IO_HI: u16 = 0xFF7F;
pub const ZRAM_LO: u16 = 0xFF80;
pub const ZRAM_HI: u16 = 0xFFFE;
pub const INT_ENABLE_REG: u16 = 0xFFFF;

pub enum Addr {
    ROMBank0(u16),
    ROMBank1(u16),
    TileData(u16),
    TileMap1(u16),
    TileMap2(u16),
    ERAM(u16),
    RAM(u16),
    Sprites(u16),
    IO(u16),
    ZRAM(u16),
    Zero
}

pub fn map_address(addr: u16) -> Addr {
    use self::Addr::*;

    match addr {
        ROM_BANK0_LO ... ROM_BANK0_HI => ROMBank0(addr-ROM_BANK0_LO),
        ROM_BANK1_LO ... ROM_BANK1_HI => ROMBank1(addr-ROM_BANK1_LO),
        TILE_DATA_LO ... TILE_DATA_HI => TileData(addr-TILE_DATA_LO),
        TILE_MAP1_LO ... TILE_MAP1_HI => TileMap1(addr-TILE_MAP1_LO),
        TILE_MAP2_LO ... TILE_MAP2_HI => TileMap2(addr-TILE_MAP2_LO),
        ERAM_LO ... ERAM_HI => ERAM(addr-ERAM_LO),
        RAM_LO ... RAM_HI => RAM(addr-RAM_LO),
        RAM_LO2 ... RAM_HI2 => RAM(addr-RAM_LO2),
        SPRITES_LO ... SPRITES_HI => Sprites(addr-SPRITES_LO),
        UNMAPPED_LO ... UNMAPPED_HI => Zero,
        IO_LO ... IO_HI => IO(addr-IO_LO),
        ZRAM_LO ... ZRAM_HI => ZRAM(addr-ZRAM_LO),
        INT_ENABLE_REG => IO(addr-IO_LO),
        _ => panic!("Access to unknown memory region detected! ({:#06x})", addr)
    }
}