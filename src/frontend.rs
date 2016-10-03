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
use sdl2;
use sdl2::pixels::{PixelFormatEnum, Color};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Texture;
use std::thread;
use std::time::{Duration, Instant};

use gameboy::Gameboy;
use events;
use joypad;
use gpu::{Framebuffer, SCREEN_WIDTH, SCREEN_HEIGHT};

const DISPLAY_WIDTH: usize = 4 * SCREEN_WIDTH;
const DISPLAY_HEIGHT: usize = 4 * SCREEN_HEIGHT;

pub struct Frontend {
    context: sdl2::Sdl,
    video: sdl2::VideoSubsystem
}

impl Frontend {
    pub fn new() -> Frontend {
        let context = sdl2::init().unwrap();
        let video = context.video().unwrap();
        Frontend {
            context: context,
            video: video
        }
    }

    fn update_texture(&self, texture: &mut Texture, framebuffer: &Framebuffer) {
        texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
            for y in 0..SCREEN_HEIGHT {
                for x in 0..SCREEN_WIDTH {
                    let ofs = y*pitch + x*3;
                    let (r, g, b) = framebuffer[y*SCREEN_WIDTH + x].to_rgb();
                    buffer[ofs+0] = r;
                    buffer[ofs+1] = g;
                    buffer[ofs+2] = b;
                }
            }
        }).unwrap();
    }

    pub fn run(&mut self, gameboy: &mut Gameboy) {
        let window = self.video.window("GBEmu", DISPLAY_WIDTH as u32,
                                                DISPLAY_HEIGHT as u32)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let mut renderer = window.renderer().build().unwrap();

        let mut texture = renderer.create_texture_streaming(
            PixelFormatEnum::RGB24, SCREEN_WIDTH  as u32, SCREEN_HEIGHT as u32
        ).unwrap();
        renderer.set_draw_color(Color::RGB(0xff, 0xff, 0xff));
        renderer.clear();
        renderer.present();


        let mut event_pump = self.context.event_pump().unwrap();

        let turbo = false;
        let frame_duration = Duration::from_secs(1) / 60;
        let mut emu_cycles: u64 = 0;
        let mut last_time = Instant::now();
        let mut target_time = last_time + frame_duration;
        'main: loop {
            let now = Instant::now();
            let delta = now - last_time;
            last_time = now;
            target_time = target_time + frame_duration;

            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} => break 'main,
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'main
                    },

                    Event::KeyDown { keycode: Some(keycode), .. } => {
                        if let Some(key) = Frontend::map_keycode(keycode) {
                            gameboy.press_key(key);
                        }
                    },
                    Event::KeyUp { keycode: Some(keycode), .. } => {
                        if let Some(key) = Frontend::map_keycode(keycode) {
                            gameboy.release_key(key);
                        }
                    },
                    _ => {}
                }
            }

            let target_cycles = emu_cycles + if turbo {
                // Simulate as many cycles as needed for one GB frame update to occur
                ((4194304 / 4) / 60) as u64
            } else {
                // Simulate as many cycles as needed for approx. 1M instructions 
                // to be executed in a second. This is adapted depending on how
                // long it takes to execute the instructions plus rendering a frame.
                (delta * (4194304 / 4)).as_secs()
            };

            loop {
                let (cycles, events) = gameboy.simulate(target_cycles);

                if events.contains(events::RENDER) {
                    self.update_texture(&mut texture, gameboy.framebuffer());
                }

                if cycles >= target_cycles {
                    emu_cycles = cycles;
                    break;
                }
            }

            renderer.copy(&texture, None, None);
            renderer.present();

            if !turbo {
                let now = Instant::now();
                if now < target_time {
                    thread::sleep(target_time - now);
                }
            }
        }
    }

    fn map_keycode(keycode: Keycode) -> Option<joypad::Key> {
        match keycode {
            Keycode::Right  => Some(joypad::Key::Right),
            Keycode::Left   => Some(joypad::Key::Left),
            Keycode::Down   => Some(joypad::Key::Down),
            Keycode::Up     => Some(joypad::Key::Up),
            Keycode::A      => Some(joypad::Key::Left),
            Keycode::D      => Some(joypad::Key::Right),
            Keycode::S      => Some(joypad::Key::Down),
            Keycode::W      => Some(joypad::Key::Up),
            Keycode::F      => Some(joypad::Key::A),
            Keycode::G      => Some(joypad::Key::B),
            Keycode::J      => Some(joypad::Key::A),
            Keycode::K      => Some(joypad::Key::B),
            Keycode::Space  => Some(joypad::Key::Select),
            Keycode::Return => Some(joypad::Key::Start),
            _               => None
        }
    }
}