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
