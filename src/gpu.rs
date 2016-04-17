use std::cmp;
use super::int_controller::{Interrupt, IntController};
use events;

pub const SCREEN_WIDTH:     usize = 160;
pub const SCREEN_HEIGHT:    usize = 144;
const BG_WIDTH:             usize = 256;
const BG_HEIGHT:            usize = 256;
const NUM_TILES:            usize = 384;
const NUM_SPRITES:          usize = 40;
const TILES_IN_SCREEN:      usize = 32;
const TILE_MAP_SIZE:        usize = 1024;
const TILE_WIDTH:           usize = 8;
const TILE_HEIGHT:          usize = 8;
const TILE_DATA0_OFS:       usize = 256;
const OAM_ENTRY_SIZE:       usize = 4;

pub type Framebuffer = [u8; SCREEN_WIDTH * SCREEN_HEIGHT * 3];

#[derive(Copy, Clone, Debug)]
struct Tile {
    pub data: [u8; 16]
}

impl Tile {
    fn new() -> Tile {
        Tile {
            data: [0; 16]
        }
    }

    fn get_color_code(&self, x: usize, line: usize) -> u8 {
        let idx = 2 * line;
        let shift = TILE_WIDTH-1-(x as usize);
        let lo = (self.data[idx] >> shift) & 0x1;
        let hi = (self.data[idx+1] >> shift) & 0x1;
        (hi << 1) | lo
    }
}

#[derive(Copy, Clone, Debug)]
enum Color {
    WHITE,
    LIGHT_GRAY,
    DARK_GRAY,
    BLACK,
}

impl Color {
    fn to_rgb(&self) -> (u8, u8, u8) {
        match *self {
            Color::WHITE        => (0xff, 0xff, 0xff),
            Color::LIGHT_GRAY   => (0xc0, 0xc0, 0xc0),
            Color::DARK_GRAY    => (0x60, 0x60, 0x60),
            Color::BLACK        => (0x00, 0x00, 0x00)
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Palette {
    pub data: u8,
}

impl Palette {
    fn new() -> Palette {
        Palette {
            data: 0,
        }
    }

    fn get_color(&self, color_id: u8) -> Color {
        use self::Color::*;
        const colors: [Color; 4] = [WHITE, LIGHT_GRAY, DARK_GRAY, BLACK];
        
        match color_id & 0b11 {
            0b00 => colors[ (self.data & 0b11) as usize],
            0b01 => colors[((self.data & 0b1100) >> 2) as usize],
            0b10 => colors[((self.data & 0b110000) >> 4) as usize],
            0b11 => colors[((self.data & 0b11000000) >> 6) as usize],
            _    => unreachable!()
        }
    }
}

bitflags! {
    flags SpriteFlags: u8 {
        const BG_PRIO   = 1 << 7,
        const Y_FLIP    = 1 << 6,
        const X_FLIP    = 1 << 5,
        const PALETTE1  = 1 << 4        
    }
}

#[derive(Copy, Clone, Debug)]
struct Sprite {
    pub y: u8,
    pub x: u8,
    pub tile_idx: u8,
    pub flags: SpriteFlags
}

impl Sprite {
    fn new() -> Sprite {
        Sprite {
            y: 0,
            x: 0,
            tile_idx: 0,
            flags: SpriteFlags::empty()
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum GpuMode {
    HBlank = 0b00,
    VBlank = 0b01,
    ScanlineOAM = 0b10,
    ScanlineVRAM = 0b11
}

impl GpuMode {
    fn from_bits(value: u8) -> GpuMode {
        match value & 0b11 {
            0b00 => GpuMode::HBlank,
            0b01 => GpuMode::VBlank,
            0b10 => GpuMode::ScanlineOAM,
            0b11 => GpuMode::ScanlineVRAM,
            _    => unreachable!()
        }
    }
}

bitflags! {
    flags LCDCFlags: u8 {
        const SHOW_BG           = 1 << 0,
        const SHOW_SPRITES      = 1 << 1,
        const WIDE_SPRITES      = 1 << 2,
        const BG_TILE_MAP       = 1 << 3,
        const TILE_DATA         = 1 << 4,
        const SHOW_WINDOW       = 1 << 5,
        const WINDOW_TILE_MAP   = 1 << 6,
        const DISPLAY_ENABLED   = 1 << 7,
    }
}

bitflags! {
    flags StatFlags: u8 {
        const LINE_MATCH        = 1 << 2,
        const HBLANK_INT        = 1 << 3,
        const VBLANK_INT        = 1 << 4,
        const OAM_INT           = 1 << 5,
        const LINE_MATCH_INT    = 1 << 6,
        const STAT_HI_BIT       = 1 << 7
    }
}

pub struct Gpu {
    mode: GpuMode,
    clock: u32,
    line: u8,
    line_match_reg: u8,

    lcdc_reg: LCDCFlags,
    stat_reg: StatFlags,

    scroll_x: u8,
    scroll_y: u8,
    window_x: u8,
    window_y: u8,

    bg_palette: Palette,
    obj_palette0: Palette,
    obj_palette1: Palette,

    tiles: [Tile; NUM_TILES],
    tile_map: [[u8; TILE_MAP_SIZE]; 2],

    oam: [Sprite; NUM_SPRITES],

    framebuffer: Framebuffer,
}

impl Gpu {
    pub fn new() -> Gpu {
        Gpu {
            mode: GpuMode::VBlank,
            clock: 0,
            line: 0,
            line_match_reg: 0,

            lcdc_reg: LCDCFlags::empty(),
            stat_reg: VBLANK_INT | STAT_HI_BIT,

            scroll_x: 0,
            scroll_y: 0,
            window_x: 0,
            window_y: 0,

            bg_palette: Palette::new(),
            obj_palette0: Palette::new(),
            obj_palette1: Palette::new(),

            tiles: [Tile::new(); NUM_TILES],
            tile_map: [[0; TILE_MAP_SIZE]; 2],

            oam: [Sprite::new(); NUM_SPRITES],

            framebuffer: [0; SCREEN_WIDTH * SCREEN_HEIGHT * 3],
        }
    }

    pub fn step(&mut self, cycles: u8, int_controller: &mut IntController) 
        -> events::Events {
        use self::GpuMode::*;

        self.clock += cycles as u32;

        let mut events = events::Events::empty();
        let mut next_line = self.line;

        match self.mode {
            HBlank if self.clock >= 51 => {
                self.clock -= 51;
                next_line = self.line + 1;

                if next_line == 144 {
                    self.update_mode(VBlank, int_controller);
                         
                    if self.lcdc_reg.contains(DISPLAY_ENABLED) {
                        events = events::RENDER;
                    }
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
                self.render_line();
            },
            _ => {}
        }

        if next_line != self.line {
            self.line = next_line;
            if self.stat_reg.contains(LINE_MATCH_INT) 
                && next_line == self.line_match_reg {
                int_controller.set_int_pending(Interrupt::LCDCStatus);
            }
        }

        return events;
    }

    fn update_mode(&mut self, mode: GpuMode, int_controller: &mut IntController) {
        use self::GpuMode::*;
        
        match mode {
            HBlank if self.stat_reg.contains(HBLANK_INT) => 
                int_controller.set_int_pending(Interrupt::LCDCStatus),
            ScanlineOAM if self.stat_reg.contains(OAM_INT) => 
                int_controller.set_int_pending(Interrupt::LCDCStatus),
            VBlank if self.stat_reg.contains(VBLANK_INT) => {
                int_controller.set_int_pending(Interrupt::LCDCStatus);
                int_controller.set_int_pending(Interrupt::VBlank);
            },
            VBlank => int_controller.set_int_pending(Interrupt::VBlank),
            _ => {}
        }
        self.mode = mode;
    }

    fn write_framebuffer(&mut self, x: usize, y: usize, color: Color) {
        let (r, g, b) = color.to_rgb();
        self.framebuffer[(y * SCREEN_WIDTH + x) * 3] = r;
        self.framebuffer[(y * SCREEN_WIDTH + x) * 3 + 1] = g;
        self.framebuffer[(y * SCREEN_WIDTH + x) * 3 + 2] = b;
    }

    fn get_tile(&self, x: usize, y: usize, use_map1: bool) -> Tile {
        let idx = (y / TILE_HEIGHT) * TILES_IN_SCREEN + x / TILE_WIDTH;
        let tile_idx = if use_map1 {
            self.tile_map[1][idx] as usize
        } else {
            self.tile_map[0][idx] as usize
        };

        if self.lcdc_reg.contains(TILE_DATA) {
            self.tiles[tile_idx]
        } else {
            let ofs = (tile_idx as i8) as i32; // Here, the offset is signed
            self.tiles[(TILE_DATA0_OFS as i32 + ofs) as usize]
        }
    }

    fn write_tile_line(&mut self, tile: &Tile, palette: Palette, 
                       y_ofs: usize, start_x: usize, len: usize) {
        for i in 0..len {
            let x_ofs = (start_x+i) as usize % TILE_WIDTH;
            let color_code = tile.get_color_code(x_ofs, y_ofs);
            let color = palette.get_color(color_code);
            let line = self.line as usize;
            self.write_framebuffer(start_x+i, line, color);
        }
    }

    fn render_line(&mut self) {
        if self.lcdc_reg.contains(SHOW_BG) {
            let y = self.line;
            let mut x = 0;
            while x < SCREEN_WIDTH {
                let bg_x = (self.scroll_x as usize + x as usize) % BG_WIDTH;
                let bg_y = (self.scroll_y as usize + y as usize) % BG_HEIGHT;
                let y_ofs = bg_y % TILE_HEIGHT;

                let mut len = ((bg_x + 8) & !0b111) - bg_x;
                if x + len >= SCREEN_WIDTH {
                    len = SCREEN_WIDTH - x;
                }

                let use_map1 = self.lcdc_reg.contains(BG_TILE_MAP);
                let tile = self.get_tile(bg_x, bg_y, use_map1);
                let palette = self.bg_palette;
                self.write_tile_line(&tile, palette, y_ofs, x, len);
                x += len;
            }
        }

        if self.lcdc_reg.contains(SHOW_WINDOW) && self.line >= self.window_y {
            let y = self.line;
            let mut x = cmp::max(self.window_x as i32 - 7, 0) as usize;
            while x < SCREEN_WIDTH {
                let wnd_x = (self.scroll_x as usize + x as usize) % BG_WIDTH;
                let wnd_y = (self.scroll_y as usize + y as usize) % BG_HEIGHT;
                let y_ofs = wnd_y % TILE_HEIGHT;
                
                let mut len = ((wnd_x + 8) & !0b111) - wnd_x;
                if x + len >= SCREEN_WIDTH {
                    len = SCREEN_WIDTH - x;
                }
                
                let use_map1 = self.lcdc_reg.contains(WINDOW_TILE_MAP);
                let tile = self.get_tile(wnd_x, wnd_y, use_map1);
                let palette = self.bg_palette;
                self.write_tile_line(&tile, palette, y_ofs, x, len);
                x += len;
            }
        }

        if self.lcdc_reg.contains(SHOW_SPRITES) {
            let sprite_width = if self.lcdc_reg.contains(WIDE_SPRITES) { 16 }
                               else { 8 };
            let mut idx = 0;
            while idx < self.oam.len() {
                let sprite = self.oam[idx];
                let y = sprite.y as i32 - 16;
                if y <= self.line as i32 && y + 8 > self.line as i32 {
                    let sprite_x = sprite.x as i32 - 8;
                    let start_x = cmp::max(sprite_x, 0);
                    let end_x = cmp::min(sprite_x+sprite_width, SCREEN_WIDTH as i32);
                    
                    let palette = if sprite.flags.contains(PALETTE1) {
                        self.obj_palette1
                    } else {
                        self.obj_palette0
                    };

                    let y_ofs = if sprite.flags.contains(Y_FLIP) {
                        TILE_HEIGHT - 1 - (self.line as i32 - y) as usize
                    } else {
                        (self.line as i32 - y) as usize
                    };

                    for x in start_x..end_x {
                        let x_ofs = if sprite.flags.contains(X_FLIP) {
                            sprite_width - 1 - (x % sprite_width)
                        } else {
                            x % sprite_width
                        };

                        let tile = if sprite_width == 8 {
                            self.tiles[sprite.tile_idx as usize]
                        } else if sprite_width == 16 && x_ofs < 8 {
                            self.tiles[(sprite.tile_idx & !0b1) as usize]
                        } else {
                            self.tiles[((sprite.tile_idx & !0b1) + 1) as usize]
                        };

                        let color_code = tile.get_color_code(
                            (x_ofs % 8) as usize, y_ofs)
                        ;
                        let color = palette.get_color(color_code);
                        let line = self.line as usize;
                        self.write_framebuffer(x as usize, line, color);
                    }
                }
                idx += 1;
            }
        }
    }

    pub fn get_framebuffer(&self) -> &Framebuffer {
        &self.framebuffer
    }

    // 0x8000-0x97FF
    pub fn read_tile_data(&self, addr: u16) -> u8 {
        let tile_idx = addr as usize / 16;
        let data_idx = addr as usize % 16;
        self.tiles[tile_idx].data[data_idx]
    }

    pub fn write_tile_data(&mut self, addr: u16, value: u8) {
        let tile_idx = addr as usize / 16;
        let data_idx = addr as usize % 16;
        self.tiles[tile_idx].data[data_idx] = value;
    }

    // 0x9800-0x9BFF
    pub fn read_tile_map1(&self, addr: u16) -> u8 {
        self.tile_map[0][addr as usize]
    }

    pub fn write_tile_map1(&mut self, addr: u16, value: u8) {
        self.tile_map[0][addr as usize] = value;
    }

    // 0x9C00-0x9FFF
    pub fn read_tile_map2(&self, addr: u16) -> u8 {
        self.tile_map[1][addr as usize]
    }

    pub fn write_tile_map2(&mut self, addr: u16, value: u8) {
        self.tile_map[1][addr as usize] = value;
    }

    // Sprites: 0xFE00-0xFE9F
    pub fn read_oam(&self, addr: u16) -> u8 {
        let sprite_idx = addr as usize / OAM_ENTRY_SIZE;
        let sprite_ofs = addr as usize % OAM_ENTRY_SIZE;
        match sprite_ofs {
            0 => self.oam[sprite_idx].y,
            1 => self.oam[sprite_idx].x,
            2 => self.oam[sprite_idx].tile_idx,
            3 => self.oam[sprite_idx].flags.bits,
            _ => unreachable!()
        }
    }

    pub fn write_oam(&mut self, addr: u16, value: u8) {
        let sprite_idx = addr as usize / OAM_ENTRY_SIZE;
        let sprite_ofs = addr as usize % OAM_ENTRY_SIZE;
        match sprite_ofs {
            0 => self.oam[sprite_idx].y = value,
            1 => self.oam[sprite_idx].x = value,
            2 => self.oam[sprite_idx].tile_idx = value,
            3 => {
                let flags = SpriteFlags::from_bits_truncate(value);
                self.oam[sprite_idx].flags = flags;
            }
            _ => unreachable!()
        }
    }

    // IO: 0xFF40
    pub fn read_lcdc_reg(&self) -> u8 {
        return self.lcdc_reg.bits;
    }

    pub fn write_lcdc_reg(&mut self, value: u8) {
        self.lcdc_reg = LCDCFlags::from_bits_truncate(value);
    }

    // IO: 0xFF41
    pub fn read_stat_reg(&self) -> u8 {
        return self.stat_reg.bits | self.mode as u8;
    }

    pub fn write_stat_reg(&mut self, value: u8) {
        self.stat_reg = StatFlags::from_bits_truncate(value & 0xFC);
    }

    // IO: 0xFF42
    pub fn read_scroll_y_reg(&self) -> u8 {
        return self.scroll_y;
    }

    pub fn write_scroll_y_reg(&mut self, value: u8) {
        self.scroll_y = value;
    }

    // IO: 0xFF43
    pub fn read_scroll_x_reg(&self) -> u8 {
        return self.scroll_x;
    }

    pub fn write_scroll_x_reg(&mut self, value: u8) {
        self.scroll_x = value;
    }

    // IO: 0xFF44
    pub fn read_line_reg(&self) -> u8 {
        self.line
    }

    // IO: 0xFF45
    pub fn read_line_match_reg(&self) -> u8 {
        self.line_match_reg
    }

    pub fn write_line_match_reg(&mut self, value: u8) {
        self.line_match_reg = value;
    }

    // IO: 0xFF47
    pub fn read_bg_palette_reg(&self) -> u8 {
        self.bg_palette.data
    }

    pub fn write_bg_palette_reg(&mut self, value: u8) {
        self.bg_palette.data = value;
    }

    // IO: 0xFF48
    pub fn read_obj_palette0_reg(&self) -> u8 {
        self.obj_palette0.data
    }

    pub fn write_obj_palette0_reg(&mut self, value: u8) {
        self.obj_palette0.data = value;
    }

    // IO: 0xFF49
    pub fn read_obj_palette1_reg(&self) -> u8 {
        self.obj_palette1.data
    }

    pub fn write_obj_palette1_reg(&mut self, value: u8) {
        self.obj_palette1.data = value;
    }

    // IO: 0xFF4A
    pub fn read_window_y_reg(&self) -> u8 {
        self.window_y
    }

    pub fn write_window_y_reg(&mut self, value: u8) {
        self.window_y = value;
    }

    // IO: 0xFF4B
    pub fn read_window_x_reg(&self) -> u8 {
        self.window_x
    }

    pub fn write_window_x_reg(&mut self, value: u8) {
        self.window_x = value;
    }


    fn print_tile(tile: Tile, palette: Palette) {
        for i in 0..TILE_HEIGHT {
            for j in 0..TILE_WIDTH {
                let color_code = tile.get_color_code(j, i);
                let color = palette.get_color(color_code);
                let ch = match color {
                    Color::BLACK => '■',
                    Color::DARK_GRAY => '▩',
                    Color::LIGHT_GRAY => '▥',
                    Color::WHITE => ' ',
                };
                print!("{}", ch);
            }
            print!("\n");
        }
        print!("\n");
    }
    
    fn print_framebuffer(&self) {
        for i in 0..SCREEN_HEIGHT {
            for j in 0..SCREEN_WIDTH {
                let color = self.framebuffer[(i*SCREEN_WIDTH+j)*3];
                let ch = match color {
                    0x00 => '■',
                    0x60 => '▩',
                    0xc0 => '▥',
                    0xFF => ' ',
                    _    => ' ',
                };
                print!("{}", ch);
            }
            print!("\n");
        }
        print!("\n");
    }
}