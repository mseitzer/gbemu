#[macro_use]
extern crate bitflags;
extern crate getopts;

use getopts::Options;
use std::io;
use std::io::prelude::*;
use std::env;
use std::error::Error;
use std::fmt;
use std::fs::File;

#[macro_use]
mod util;

mod cpu;
mod hardware;
mod mem_map;
mod memory;
mod gpu;
mod timer;
mod int_controller;
mod instructions;
mod debug;

struct Gameboy {
    cpu: cpu::Cpu<hardware::Hardware>
}

impl Gameboy {
    fn new(bios: Box<[u8]>, rom: Box<[u8]>) -> Gameboy {
        let hardware = hardware::Hardware::new(bios, rom);

        Gameboy {
            cpu: cpu::Cpu::new(hardware)
        }
    }

    fn start(&mut self) {
        self.cpu.run();
    }
}


impl fmt::Display for Gameboy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.cpu)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optflag("d", "debug", "Starts the emulator in debug mode");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.free.is_empty() {
        println!("Need an input file!");
        return;
    }

    let rom_path = matches.free[0].clone();
    let rom_buf = read_file(rom_path);
    let bios_buf = read_file("rom.bin".to_string());

    if matches.opt_present("d") {
        debug::start(bios_buf, rom_buf);
    } else {
        let mut gb = Gameboy::new(bios_buf, rom_buf);
        gb.start();
    }
}

fn read_file(path: String) -> Box<[u8]> {
    let mut fd = match File::open(&path) {
        Err(why) => panic!("Can't open file '{}': {}", path, why.description()),
        Ok(f) => f
    };

    let mut buf = Vec::new();
    if let Err(why) = fd.read_to_end(&mut buf) {
        panic!("Can't read file '{}': {}", path, why.description());
    }

    buf.into_boxed_slice()
}