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

#[derive(Copy, Clone, Debug)]
pub enum Interrupt {
    VBlank          = 1 << 0,
    LCDCStatus      = 1 << 1,
    Timer           = 1 << 2, 
    SerialTransfer  = 1 << 3,
    Joypad          = 1 << 4,
}

impl Interrupt {
    fn from_bits(bits: u8) -> Option<Interrupt> {
        match bits {
            1 => Some(Interrupt::VBlank),
            2 => Some(Interrupt::LCDCStatus),
            4 => Some(Interrupt::Timer),
            8 => Some(Interrupt::SerialTransfer),
            16 => Some(Interrupt::Joypad),
            _ => None
        }
    }

    pub fn isr_addr(&self) -> u16 {
        match *self {
            Interrupt::VBlank          => 0x40,
            Interrupt::LCDCStatus      => 0x48,
            Interrupt::Timer           => 0x50, 
            Interrupt::SerialTransfer  => 0x58,
            Interrupt::Joypad          => 0x60,
        }
    }
}

bitflags! {
    flags InterruptFlags: u8 {
        #[allow(dead_code)]
        const INT_VBLANK            = 1 << 0,
        #[allow(dead_code)]
        const INT_LCDCSTATUS        = 1 << 1,
        #[allow(dead_code)]
        const INT_TIMER             = 1 << 2,
        #[allow(dead_code)]
        const INT_SERIAL_TRANSFER   = 1 << 3,
        #[allow(dead_code)]
        const INT_JOYPAD            = 1 << 4
    }
}

impl InterruptFlags {
    fn get_highest_priority(&self) -> InterruptFlags {
        // Get only rightmost bit: r = x & -x = x & (!x + 1)
        let bits = self.bits & (!self.bits).wrapping_add(1);
        InterruptFlags::from_bits_truncate(bits)
    }
}

pub struct IntController {
    ints_enabled: InterruptFlags,
    ints_pending: InterruptFlags
}

impl IntController {
    pub fn new() -> IntController {
        IntController {
            ints_enabled: InterruptFlags::empty(),
            ints_pending: InterruptFlags::empty()
        }
    }

    pub fn read_enabled_reg(&self) -> u8 {
        self.ints_enabled.bits
    }

    pub fn write_enabled_reg(&mut self, value: u8) {
        self.ints_enabled.bits = value;
    }

    pub fn read_pending_reg(&self) -> u8 {
        self.ints_pending.bits
    }

    pub fn write_pending_reg(&mut self, value: u8) {
        self.ints_pending.bits = value;
    }

    pub fn set_int_pending(&mut self, int: Interrupt) {
        let flag = InterruptFlags::from_bits_truncate(int as u8);
        self.ints_pending = self.ints_pending | flag
    }

    pub fn has_irq(&self) -> bool {
        return self.ints_enabled & self.ints_pending != InterruptFlags::empty();
    }

    pub fn ack_irq(&mut self) -> Option<Interrupt> {
        let allowed_ints = self.ints_enabled & self.ints_pending;
        let highest_prio = allowed_ints.get_highest_priority();
        self.ints_pending = self.ints_pending - highest_prio;
        Interrupt::from_bits(highest_prio.bits)
    }
}