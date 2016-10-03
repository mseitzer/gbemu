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

#[macro_use]
extern crate bitflags;
extern crate argparse;
extern crate sdl2;

use argparse::{ArgumentParser, Store, StoreTrue};
use std::io::prelude::*;
use std::error::Error;
use std::fs::File;

#[macro_use]
mod util;

mod cartridge;
mod cpu;
mod debug;
mod events;
mod frontend;
mod gameboy;
mod gpu;
mod hardware;
mod instructions;
mod int_controller;
mod joypad;
mod mem_map;
mod memory;
mod timer;

fn main() {
    let mut debug_mode = false;
    let mut bios_path = String::from("rom.bin");
    let mut rom_path = String::new();

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("GBEmu - A Gameboy emulator");
        ap.refer(&mut debug_mode)
            .add_option(&["-d", "--debug"], StoreTrue, "Start in debug mode");
        ap.refer(&mut bios_path)
            .add_option(&["--bios"], Store, "Path to Gameboy BIOS");
        ap.refer(&mut rom_path)
            .add_argument("ROM Path", Store, "Path to the ROM to emulate")
            .required();
        ap.parse_args_or_exit();
    }

    let rom_buf = read_file(rom_path);
    let bios_buf = read_file(bios_path);

    if debug_mode {
        debug::start(bios_buf, rom_buf);
    } else {
        let mut gb = gameboy::Gameboy::new(bios_buf, rom_buf);
        let mut frontend = frontend::Frontend::new();
        frontend.run(&mut gb);
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
