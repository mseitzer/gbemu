extern crate sdl2;

use self::sdl2::pixels::{PixelFormatEnum, Color};
use self::sdl2::event::Event;
use self::sdl2::keyboard::Keycode;
use self::sdl2::render::{Texture, TextureAccess, BlendMode};

use super::Gameboy;
use events;
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
        texture.set_blend_mode(BlendMode::None);

        renderer.clear();

        let mut event_pump = self.context.event_pump().unwrap();

        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                    },
                    _ => {}
                }
            }
            
            let events = gameboy.simulate();
            if let Some(events) = events {
                if events.contains(events::RENDER) {
                    self.update_texture(&mut texture, gameboy.framebuffer());
                    renderer.copy(&texture, None, None);
                    renderer.present();
                }
            }
        }
    }
}