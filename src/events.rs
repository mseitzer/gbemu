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
bitflags! {
    pub flags Events: u8 {
        const RENDER = 1
    }
}