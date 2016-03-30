use super::int_controller::{Interrupt, IntController};

#[derive(Copy, Clone, Debug, PartialEq)]
enum GpuMode {
    HBlank = 0b00,
    VBlank = 0b01,
    ScanlineOAM = 0b10,
    ScanlineVRAM = 0b11
}

pub struct Gpu {
    mode: GpuMode,
    clock: u32,
    line: u8,
    line_match_reg: u8,

    line_match_stat_int: bool,
    oam_stat_int: bool,
    vblank_stat_int: bool,
    hblank_stat_int: bool,

    stat_hi_bit: u8,
}

impl Gpu {
    pub fn new() -> Gpu {
        Gpu {
            mode: GpuMode::VBlank,
            clock: 0,
            line: 0,
            line_match_reg: 0,

            line_match_stat_int: false,
            oam_stat_int: false,
            vblank_stat_int: true,
            hblank_stat_int: false,

            stat_hi_bit: 1
        }
    }

    pub fn step(&mut self, cycles: u8, int_controller: &mut IntController) {
        use self::GpuMode::*;

        self.clock += cycles as u32;

        let mut next_line = self.line;

        match self.mode {
            HBlank if self.clock >= 51 => {
                self.clock -= 51;
                next_line = self.line + 1;

                if self.line == 143 {
                    self.update_mode(VBlank, int_controller);
                    // TODO: render framebuffer here
                } else {
                    self.update_mode(ScanlineOAM, int_controller);
                }
            },
            VBlank if self.clock >= 114 => {
                self.clock -= 114;
                next_line = self.line + 1;
                if next_line > 153 {
                    self.update_mode(ScanlineOAM, int_controller);
                    next_line = 0;
                }
            },
            ScanlineOAM if self.clock >= 20 => {
                self.clock -= 20;
                self.update_mode(ScanlineVRAM, int_controller);
            },
            ScanlineVRAM if self.clock >= 43 => {
                self.clock -= 43;
                self.update_mode(HBlank, int_controller);
                // TODO: write scanline to framebuffer here
            },
            _ => {}
        }

        if next_line != self.line {
            self.line = next_line;
            if self.line_match_stat_int && next_line == self.line_match_reg {
                int_controller.set_int_pending(Interrupt::LCDCStatus);
            }
        }
    }

    fn update_mode(&mut self, mode: GpuMode, int_controller: &mut IntController) {
        use self::GpuMode::*;

        if mode != self.mode {
            match mode {
                HBlank if self.hblank_stat_int => 
                    int_controller.set_int_pending(Interrupt::LCDCStatus),
                ScanlineOAM if self.oam_stat_int => 
                    int_controller.set_int_pending(Interrupt::LCDCStatus),
                VBlank if self.vblank_stat_int => {
                    int_controller.set_int_pending(Interrupt::LCDCStatus);
                    int_controller.set_int_pending(Interrupt::VBlank);
                },
                VBlank => int_controller.set_int_pending(Interrupt::VBlank),
                _ => {}
            }
        }
        self.mode = mode;
    }

    pub fn read_stat_reg(&self) -> u8 {
        return self.mode as u8
            | ((self.line == self.line_match_reg) as u8) << 2
            | (self.hblank_stat_int as u8) << 3
            | (self.vblank_stat_int as u8) << 4
            | (self.oam_stat_int as u8) << 5
            | (self.line_match_stat_int as u8) << 6
            | self.stat_hi_bit;
    }

    pub fn write_stat_reg(&mut self, value: u8) {
        self.hblank_stat_int = value & 0b1000 != 0;
        self.vblank_stat_int = value & 0b10000 != 0;
        self.oam_stat_int = value & 0b100000 != 0;
        self.line_match_stat_int = value & 0b1000000 != 0;
        self.stat_hi_bit = value & 0b10000000;
    }

    pub fn read_line_reg(&self) -> u8 {
        self.line
    }

    pub fn read_line_match_reg(&self) -> u8 {
        self.line_match_reg
    }

    pub fn write_line_match_reg(&mut self, value: u8) {
        self.line_match_reg = value;
    }

    pub fn line_match(&self) -> bool {
        self.line_match_stat_int && self.line == self.line_match_reg
    }
}