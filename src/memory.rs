
pub struct Memory {
    // Cartridge RAM: 0xA000-0xBFFF
    // External RAM which is available through the cartridge
    eram: [u8; 8192],

    // RAM: 0xC000-0xDFFF
    // Shadow copy of the RAM from 0xE000-0xFDFF
    ram: [u8; 8192],

    // Graphics sprite information: 0xFE00-0xFE9F
    sprites: [u8; 160],

    // Zero page RAM: 0xFF80-0xFFFF
    zram: [u8; 128],
}

impl Memory {
    pub fn new() -> Box<Memory> {
        Box::new(Memory {
            eram: [0; 8192],
            ram: [0; 8192],
            zram: [0; 128],
            sprites: [0; 160]
        })
    }

    pub fn read_eram(&self, addr: u16) -> u8 {
        self.eram[addr as usize]
    }

    pub fn read_ram(&self, addr: u16) -> u8 {
        self.ram[addr as usize]
    }

    pub fn read_zram(&self, addr: u16) -> u8 {
        self.zram[addr as usize]
    }

    pub fn write_eram(&mut self, addr: u16, value: u8) {
        self.eram[addr as usize] = value;
    }

    pub fn write_ram(&mut self, addr: u16, value: u8) {
        self.ram[addr as usize] = value;
    }

    pub fn write_zram(&mut self, addr: u16, value: u8) {
        self.zram[addr as usize] = value;
    }

    /* Maybe sprites will go to a different component */
    pub fn read_sprites(&self, addr: u16) -> u8 {
        self.sprites[addr as usize]
    }

    pub fn write_sprites(&mut self, addr: u16, value: u8) {
        self.sprites[addr as usize] = value;
    }
}