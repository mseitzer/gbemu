
pub const ROM_BANK0_LO: u16 = 0x0000;
pub const ROM_BANK0_HI: u16 = 0x3FFF;
pub const ROM_BANK1_LO: u16 = 0x4000;
pub const ROM_BANK1_HI: u16 = 0x7FFF;
pub const VRAM_LO: u16 = 0x8000;
pub const VRAM_HI: u16 = 0x9FFF;
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
pub const ZRAM_HI: u16 = 0xFFFF;

#[allow(non_camel_case_types)]
pub enum Addr {
    rom_bank0(u16),
    rom_bank1(u16),
    vram(u16),
    eram(u16),
    ram(u16),
    sprites(u16),
    io(u16),
    zram(u16),
    zero
}

pub fn map_address(addr: u16) -> Addr {
    use self::Addr::*;

    match addr {
        ROM_BANK0_LO ... ROM_BANK0_HI => rom_bank0(addr-ROM_BANK0_LO),
        ROM_BANK1_LO ... ROM_BANK1_HI => rom_bank1(addr-ROM_BANK1_LO),
        VRAM_LO ... VRAM_HI => vram(addr-VRAM_LO),
        ERAM_LO ... ERAM_HI => eram(addr-ERAM_LO),
        RAM_LO ... RAM_HI => ram(addr-RAM_LO),
        RAM_LO2 ... RAM_HI2 => ram(addr-RAM_LO2),
        SPRITES_LO ... SPRITES_HI => sprites(addr-SPRITES_LO),
        UNMAPPED_LO ... UNMAPPED_HI => zero,
        IO_LO ... IO_HI => io(addr-IO_LO),
        ZRAM_LO ... ZRAM_HI => zram(addr-ZRAM_LO),
        _ => panic!("Access to unknown memory region detected! ({:#06x})", addr)
    }
}