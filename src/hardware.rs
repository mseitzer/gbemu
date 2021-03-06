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
use mem_map;
use memory;
use timer;
use gpu;
use int_controller::{self, Interrupt};
use events;
use cartridge;
use joypad;

mod dma {
    #[derive(Copy, Clone, Debug)]
    enum DmaState {
        Inactive,
        Requested,
        Starting,
        Copying,
        Ending,
    }

    pub struct Dma {
        state: DmaState,
        source: u16,
        clock: u64,
    }

    impl Dma {
        pub fn new() -> Dma {
            Dma {
                state: DmaState::Inactive,
                source: 0x0000,
                clock: 0,
            }
        }

        pub fn is_active(&self) -> bool {
            match self.state {
                DmaState::Inactive => false,
                _                  => true
            }
        }

        pub fn initiate(&mut self, source: u8) {
            self.state = DmaState::Requested;
            self.source = (source as u16) << 8;
            self.clock = 0;
        }

        pub fn tick(&mut self, cycles: u8) -> (u16, u16, u16) {
            // DMA takes 162 cycles (probably):
            // 1 cycle startup, 160 cycles copy, 1 cycle ending
            use self::DmaState::*;

            let new_clock = self.clock + cycles as u64;
            let new_state = match self.state {
                Inactive => Inactive,
                Requested if new_clock == 1 => Starting,
                Requested if new_clock > 1 => Copying,
                Starting => Copying,
                Copying if new_clock == 161 => Ending,
                Copying if new_clock > 161 => Inactive,
                Ending if new_clock >= 162 => Inactive,
                state @ _ => state
            };
            
            let res = match new_state {
                Copying => {
                    let ofs = self.clock.saturating_sub(1) as u16;
                    let len = match self.state {
                        Requested | Starting => (new_clock - 1 - self.clock) as u16,
                        _ => (new_clock - self.clock) as u16,
                    };
                    (self.source, ofs, len)
                },
                Ending | Inactive => {
                    let ofs = self.clock.saturating_sub(1) as u16;
                    let len = 160u16.saturating_sub(ofs);
                    (self.source, ofs, len)
                },
                _ => (0, 0, 0)
            };

            self.state = new_state;
            self.clock = new_clock;
            res
        }
    }
}

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
    dma: dma::Dma,
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
            dma: dma::Dma::new(),
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
            ROMBank0(_) => self.cartridge.write(addr, value),
            ROMBank1(_) => self.cartridge.write(addr, value),
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
                        //panic!("Attempting to write to line reg")
                    }
                    0x45 => self.gpu.write_line_match_reg(value),
                    0x46 => {
                        self.dma.initiate(value);
                    },
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

                    _ => {}
                }
            }
        }
    }

    pub fn framebuffer(&self) -> &gpu::Framebuffer {
        self.gpu.get_framebuffer()
    }

    pub fn press_key(&mut self, key: joypad::Key) {
        self.joypad.key_pressed(key, &mut self.int_controller);
    }

    pub fn release_key(&mut self, key: joypad::Key) {
        self.joypad.key_released(key);
    }
}

impl Bus for Hardware {
    fn read(&self, addr: u16) -> u8 {
        if self.dma.is_active() 
            && !(mem_map::ZRAM_LO <= addr && addr < mem_map::ZRAM_HI) {
            return 0xff;
        }
        self.read_byte(addr)
    }

    fn write(&mut self, addr: u16, value: u8) {
        if self.dma.is_active() 
            && !(mem_map::ZRAM_LO <= addr && addr < mem_map::ZRAM_HI)
            && addr != 0xff46 {
            return;
        }
        self.write_byte(addr, value)
    }

    fn has_irq(&self) -> bool {
        self.int_controller.has_irq()
    }

    fn ack_irq(&mut self) -> Option<Interrupt> {
        self.int_controller.ack_irq()
    }

    fn update(&mut self, cycles: u8) -> events::Events {
        if self.dma.is_active() {
            let (source, ofs, len) = self.dma.tick(cycles);

            for i in ofs..ofs+len {
                let value = self.read_byte(source + i);
                self.gpu.write_oam(i, value);
            }
        }

        self.timer.tick(cycles, &mut self.int_controller);

        self.gpu.step(cycles, &mut self.int_controller)
    }
}