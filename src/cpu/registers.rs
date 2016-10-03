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
use instructions::{Reg8, Reg16};

bitflags! {
    pub flags Flags: u8 {
        const ZERO     = 1 << 7,
        const SUB      = 1 << 6,
        const HCARRY   = 1 << 5,
        const CARRY    = 1 << 4,
    }
}

impl Flags {
    pub fn test(&self, test: bool) -> Flags {
        if test { *self } else { Flags::empty() }
    }

    pub fn force(&mut self, flag: Flags, insert: bool) {
        if insert {
            self.insert(flag)
        } else {
            self.remove(flag)
        }
    }
}

pub struct Registers {
    pub pc: u16,
    pub sp: u16,
    pub a: u8,
    pub f: Flags,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            pc: 0x0000,
            sp: 0x0000,
            a: 0x00,
            f: Flags::empty(),
            b: 0x00,
            c: 0x00,
            d: 0x00,
            e: 0x00,
            h: 0x00,
            l: 0x00
        }
    }

    pub fn read8(&self, src: Reg8) -> u8 {
        match src {
            Reg8::A => self.a,
            Reg8::F => self.f.bits(),
            Reg8::B => self.b,
            Reg8::C => self.c,
            Reg8::D => self.d,
            Reg8::E => self.e,
            Reg8::H => self.h,
            Reg8::L => self.l,
        }
    }

    pub fn read16(&self, src: Reg16) -> u16 {
        match src {
            Reg16::AF => ((self.a as u16) << 8) + self.f.bits() as u16,
            Reg16::BC => ((self.b as u16) << 8) + self.c as u16,
            Reg16::DE => ((self.d as u16) << 8) + self.e as u16,
            Reg16::HL => ((self.h as u16) << 8) + self.l as u16,
            Reg16::SP => self.sp
        }
    }

    pub fn write8(&mut self, dest: Reg8, value: u8) {
        match dest {
            Reg8::A => self.a = value,
            Reg8::F => self.f = Flags::from_bits_truncate(value),
            Reg8::B => self.b = value,
            Reg8::C => self.c = value,
            Reg8::D => self.d = value,
            Reg8::E => self.e = value,
            Reg8::H => self.h = value,
            Reg8::L => self.l = value,
        }
    }

    pub fn write16(&mut self, dest: Reg16, value: u16) {
        let lo = (value & 0x00ff) as u8;
        let hi = (value >> 8) as u8;
        match dest {
            Reg16::AF => {
                self.a = hi;
                self.f = Flags::from_bits_truncate(lo);
            },
            Reg16::BC => {
                self.b = hi;
                self.c = lo;
            },
            Reg16::DE => {
                self.d = hi;
                self.e = lo;
            },
            Reg16::HL => {
                self.h = hi;
                self.l = lo;
            },
            Reg16::SP => {
                self.sp = value;
            }
        };
    }
}