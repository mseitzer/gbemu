use mem_map;
use memory;
use timer;
use gpu;
use int_controller::{self, Interrupt};
use events;
use cartridge;
use joypad;

pub trait Bus {
    fn read(&self, u16) -> u8;
    fn write(&mut self, u16, u8);
    fn has_irq(&self) -> bool;
    fn ack_irq(&mut self) -> Option<Interrupt>;
    fn update(&mut self, u8) -> events::Events;
}

pub struct Hardware {
    memory: Box<memory::Memory>,
    gpu: gpu::Gpu,
    timer: timer::Timer,
    joypad: joypad::Joypad,
    int_controller: int_controller::IntController,

    bios_mapped: bool,
    bios: Box<[u8]>,

    cartridge: cartridge::Cartridge,
}

impl Hardware {
    pub fn new(bios: Box<[u8]>, cart_rom: Box<[u8]>) -> Hardware {
        Hardware {
            memory: memory::Memory::new(),
            gpu: gpu::Gpu::new(),
            timer: timer::Timer::new(),
            joypad: joypad::Joypad::new(),
            int_controller: int_controller::IntController::new(),

            bios_mapped: true,
            bios: bios,

            cartridge: cartridge::Cartridge::new(cart_rom),

        }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        use mem_map::Addr::*;

        match mem_map::map_address(addr) {
            ROMBank0(a) => {
                if self.bios_mapped && a < 256 {
                    self.bios[a as usize]
                } else {
                    self.cartridge.read_rom_bank0(a)
                }
            },
            ROMBank1(a) => self.cartridge.read_rom_bank1(a),
            
            TileData(a) => self.gpu.read_tile_data(a),
            TileMap1(a) => self.gpu.read_tile_map1(a),
            TileMap2(a) => self.gpu.read_tile_map2(a),
            ERAM(a) => self.cartridge.read_ram(a),
            RAM(a) => self.memory.read_ram(a),
            ZRAM(a) => self.memory.read_zram(a),
            Sprites(a) => self.gpu.read_oam(a),
            Zero => 0x00,

            IO(a) => {
                match a {
                    // Joypad
                    0x00 => self.joypad.read_joypad_reg(),

                    // Timer
                    0x04 => self.timer.read_divider_reg(),
                    0x05 => self.timer.read_counter_reg(),
                    0x06 => self.timer.read_modulo_reg(),
                    0x07 => self.timer.read_control_reg(),

                    // GPU
                    0x40 => self.gpu.read_lcdc_reg(),
                    0x41 => self.gpu.read_stat_reg(),
                    0x42 => self.gpu.read_scroll_y_reg(),
                    0x43 => self.gpu.read_scroll_x_reg(),
                    0x44 => self.gpu.read_line_reg(),
                    0x45 => self.gpu.read_line_match_reg(),
                    0x47 => self.gpu.read_bg_palette_reg(),
                    0x48 => self.gpu.read_obj_palette0_reg(),
                    0x49 => self.gpu.read_obj_palette1_reg(),
                    0x4A => self.gpu.read_window_y_reg(),
                    0x4B => self.gpu.read_window_x_reg(),

                    // Interrupts
                    0x0f => self.int_controller.read_pending_reg(),
                    0xff => self.int_controller.read_enabled_reg(),

                    _ => {println!("Input action {:#04x} not implemented", a); 0x00}
                }
            }
        }
    }

    pub fn write_byte(&mut self, addr: u16, value: u8) {
        use mem_map::Addr::*;

        match mem_map::map_address(addr) {
            ROMBank0(a) => self.cartridge.write(addr, value),
            ROMBank1(a) => self.cartridge.write(addr, value),
            TileData(a) => self.gpu.write_tile_data(a, value),
            TileMap1(a) => self.gpu.write_tile_map1(a, value),
            TileMap2(a) => self.gpu.write_tile_map2(a, value),
            ERAM(a) => self.cartridge.write_ram(a, value),
            RAM(a) => self.memory.write_ram(a, value),
            Sprites(a) => self.gpu.write_oam(a, value),
            ZRAM(a) => self.memory.write_zram(a, value),
            Zero => {},

            IO(a) => {
                match a {
                    // Joypad
                    0x00 => self.joypad.write_joypad_reg(value),

                    // Timer
                    0x04 => self.timer.write_divider_reg(value),
                    0x05 => self.timer.write_counter_reg(value),
                    0x06 => self.timer.write_modulo_reg(value),
                    0x07 => self.timer.write_control_reg(value),

                    // GPU
                    0x40 => self.gpu.write_lcdc_reg(value),
                    0x41 => self.gpu.write_stat_reg(value),
                    0x42 => self.gpu.write_scroll_y_reg(value),
                    0x43 => self.gpu.write_scroll_x_reg(value),
                    0x44 => {
                        // Unclear if this is allowed or not
                        panic!("Attempting to write to line reg")
                    }
                    0x45 => self.gpu.write_line_match_reg(value),
                    0x47 => self.gpu.write_bg_palette_reg(value),
                    0x48 => self.gpu.write_obj_palette0_reg(value),
                    0x49 => self.gpu.write_obj_palette1_reg(value),
                    0x4A => self.gpu.write_window_y_reg(value),
                    0x4B => self.gpu.write_window_x_reg(value),

                    // Unmap bios
                    0x50 if value != 0 => self.bios_mapped = false,

                    // Interrupts
                    0x0f => self.int_controller.write_pending_reg(value),
                    0xff => self.int_controller.write_enabled_reg(value),

                    _ => println!("Output action {:#04x} not implemented", a)
                }
            }
            //_ => panic!("Write to non-writeable memory region detected! ({:#06x})",
            //    addr)
        }
    }

    pub fn framebuffer(&self) -> &gpu::Framebuffer {
        self.gpu.get_framebuffer()
    }

    pub fn press_key(&mut self, key: joypad::Key) {
        self.joypad.key_pressed(key);
    }

    pub fn release_key(&mut self, key: joypad::Key) {
        self.joypad.key_released(key);
    }
}

impl Bus for Hardware {
    fn read(&self, addr: u16) -> u8 {
        self.read_byte(addr)
    }

    fn write(&mut self, addr: u16, value: u8) {
        self.write_byte(addr, value)
    }

    fn has_irq(&self) -> bool {
        self.int_controller.has_irq()
    }

    fn ack_irq(&mut self) -> Option<Interrupt> {
        self.int_controller.ack_irq()
    }

    fn update(&mut self, cycles: u8) -> events::Events {
        self.timer.tick(cycles, &mut self.int_controller);

        self.gpu.step(cycles, &mut self.int_controller)
    }
}