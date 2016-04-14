use sdl2;
use sdl2::pixels::{PixelFormatEnum, Color};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{Texture, TextureAccess, BlendMode};
use time::{Duration, SteadyTime};
use std::thread;

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
                    let fb_ofs = (y*SCREEN_WIDTH + x)*3;
                    buffer[ofs+0] = framebuffer[fb_ofs+0];
                    buffer[ofs+1] = framebuffer[fb_ofs+1];
                    buffer[ofs+2] = framebuffer[fb_ofs+2];
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
        let frame_duration = Duration::seconds(1) / 60;
        let mut emu_cycles: u64 = 0;
        let mut last_time = SteadyTime::now();
        let mut target_time = last_time + frame_duration;
        'main: loop {
            let now = SteadyTime::now();
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
                (delta * (4194304 / 4)).num_seconds() as u64
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
                let now = SteadyTime::now();
                if now < target_time {
                    let delta = (target_time - now).num_milliseconds() as u32;
                    thread::sleep_ms(delta);
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
            Keycode::A      => Some(joypad::Key::Right),
            Keycode::D      => Some(joypad::Key::Left),
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