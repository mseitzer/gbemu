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
use int_controller::{IntController, Interrupt};

#[derive(Debug)]
pub enum Key {
    Right,
    Left,
    Up,
    Down,
    A,
    B,
    Select,
    Start,
}

impl Key {
    fn column(&self) -> usize {
        use self::*;

        match *self {
            Key::Right | Key::Left | Key::Up | Key::Down => 0,
            Key::A | Key::B | Key::Select | Key::Start => 1
        }
    }
}

bitflags! {
    flags KeysPressed: u8 {
        const KEY_RIGHT     = 1 << 0,
        const KEY_A         = 1 << 0,
        const KEY_LEFT      = 1 << 1,
        const KEY_B         = 1 << 1,
        const KEY_UP        = 1 << 2,
        const KEY_SELECT    = 1 << 2,
        const KEY_DOWN      = 1 << 3,
        const KEY_START     = 1 << 3
    }
}

impl KeysPressed {
    fn from_key(key: Key) -> KeysPressed {
        match key {
            Key::Right  => KEY_RIGHT,
            Key::Left   => KEY_LEFT,
            Key::Up     => KEY_UP,
            Key::Down   => KEY_DOWN,
            Key::A      => KEY_A,
            Key::B      => KEY_B,
            Key::Select => KEY_SELECT,
            Key::Start  => KEY_START,
        }
    }   
}

pub struct Joypad {
    keys_pressed: [KeysPressed; 2],
    active_column: usize,
}

impl Joypad {
    pub fn new() -> Joypad {
        Joypad {
            keys_pressed: [KeysPressed::empty(); 2],
            active_column: 2
        }
    }

    pub fn key_pressed(&mut self, key: Key, int_controller: &mut IntController) {
        self.keys_pressed[key.column()].insert(KeysPressed::from_key(key));
        int_controller.set_int_pending(Interrupt::Joypad);
    }

    pub fn key_released(&mut self, key: Key) {
        self.keys_pressed[key.column()].remove(KeysPressed::from_key(key));
    }

    pub fn read_joypad_reg(&self) -> u8 {
        let val = match self.active_column {
            0 => 0b100000 | (!self.keys_pressed[0].bits & 0b1111),
            1 => 0b010000 | (!self.keys_pressed[1].bits & 0b1111),
            2 => !(self.keys_pressed[0].bits | self.keys_pressed[1].bits) & 0b1111,
            3 => 0b111111,
            _ => unreachable!()
        };
        return val;
    }

    pub fn write_joypad_reg(&mut self, value: u8) {
        self.active_column = match (value >> 4) & 0b11 {
            0b00 => 2,
            0b10 => 0,
            0b01 => 1,
            0b11 => 3,
            _    => unreachable!()
        }
    }
}