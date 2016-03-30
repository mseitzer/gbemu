use mem_map;
use memory;
use timer;
use gpu;
use int_controller::{self, Interrupt};

pub trait Bus {
    fn read(&self, u16) -> u8;
    fn write(&mut self, u16, u8);
    fn has_irq(&self) -> bool;
    fn ack_irq(&mut self) -> Option<Interrupt>;
    fn update(&mut self, u8);
}

pub struct Hardware {
    memory: Box<memory::Memory>,
    gpu: gpu::Gpu,
    timer: timer::Timer,
    int_controller: int_controller::IntController,

    bios_mapped: bool,
    bios: Box<[u8]>,

    cart_rom: Box<[u8]>,
    rom_bank_selector: u8,
}

impl Hardware {
    pub fn new(bios: Box<[u8]>, cart_rom: Box<[u8]>) -> Hardware {
        Hardware {
            memory: memory::Memory::new(),
            gpu: gpu::Gpu::new(),
            timer: timer::Timer::new(),

            bios_mapped: true,
            bios: bios,

            cart_rom: cart_rom,
            rom_bank_selector: 1,

            int_controller: int_controller::IntController::new()
        }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        use mem_map::Addr::*;

        match mem_map::map_address(addr) {
            rom_bank0(a) => {
                if self.bios_mapped && a < 256 {
                    self.bios[a as usize]
                } else {
                    self.cart_rom[a as usize]
                }
            }

            rom_bank1(a) => {
                assert!(self.rom_bank_selector >= 1);
                self.cart_rom[(16384 * self.rom_bank_selector as u16 + a) as usize]
            }
            
            vram(a) => self.memory.read_vram(a),
            eram(a) => self.memory.read_eram(a),
            ram(a) => self.memory.read_ram(a),
            zram(a) => self.memory.read_zram(a),
            sprites(a) => self.memory.read_sprites(a),
            zero => 0x00,

            io(a) => {
                // TODO: IO
                match a {
                    // Timer
                    0x04 => self.timer.read_divider_reg(),
                    0x05 => self.timer.read_counter_reg(),
                    0x06 => self.timer.read_modulo_reg(),
                    0x07 => self.timer.read_control_reg(),

                    // GPU
                    0x41 => self.gpu.read_stat_reg(),

                    0x44 => self.gpu.read_line_reg(),

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
            vram(a) => self.memory.write_vram(a, value),
            eram(a) => self.memory.write_eram(a, value),
            ram(a) => self.memory.write_ram(a, value),
            sprites(a) => self.memory.write_sprites(a, value),
            zram(a) => self.memory.write_zram(a, value),
            zero => {},

            io(a) => {
                // TODO: IO
                match a {
                    // Timer
                    0x04 => self.timer.write_divider_reg(value),
                    0x05 => self.timer.write_counter_reg(value),
                    0x06 => self.timer.write_modulo_reg(value),
                    0x07 => self.timer.write_control_reg(value),

                    // GPU
                    0x41 => self.gpu.write_stat_reg(value),

                    0x44 => {
                        // Unclear if this is allowed or not
                        panic!("Attempting to write to line reg")
                    }

                    // Unmap bios
                    0x50 if value != 0 => self.bios_mapped = false,

                    // Interrupts
                    0x0f => self.int_controller.write_pending_reg(value),
                    0xff => self.int_controller.write_enabled_reg(value),

                    _ => println!("Output action {:#04x} not implemented", a)
                }
            }

            _ => panic!("Write to non-writeable memory region detected! ({:#06x})",
                addr)
        }
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

    fn update(&mut self, cycles: u8) {
        self.timer.tick(cycles, &mut self.int_controller);

        self.gpu.step(cycles, &mut self.int_controller);
    }
}