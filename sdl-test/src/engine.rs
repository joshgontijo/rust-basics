use std::any::Any;
use std::collections::HashMap;
use std::error::Error;
use std::marker::PhantomData;
use std::path::Path;
use std::thread;
use std::time::Duration;

use sdl2::{EventPump, image};
use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;
use ecs::World;

pub struct Engine;

pub struct TextureManager<'a> {
    pub(crate) owner: &'a TextureCreator<WindowContext>,
    pub(crate) tex: HashMap<usize, Texture<'a>>,
}

impl<'a> TextureManager<'a> {
    pub(crate) fn load_texture<P: AsRef<Path>>(&mut self, path: P) -> Result<usize, Box<dyn Error>> {
        let result = self.owner.load_texture(path)?;
        self.tex.insert(0, result);
        Ok(0)
    }
}

struct Textures(Vec<usize>);

pub struct Ctx<'a> {
    pub canvas: WindowCanvas,
    pub textures: TextureManager<'a>,
}

fn render(ctx: &mut Ctx) -> Result<(), Box<dyn Error>> {
    ctx.canvas.set_draw_color(Color::BLACK);
    ctx.canvas.clear();


    ctx.canvas.present();
    Ok(())
}


impl Engine {
    pub fn run(mut world: World<Ctx>) -> Result<(), Box<dyn Error>> {
        let (mut canvas, mut event_pump, mut texture_creator) = Engine::init()?;

        let mut textures = TextureManager {
            owner: &texture_creator,
            tex: Default::default(),
        };

        let text_id = textures.load_texture("assets/reaper.png")?;

        let mut ctx = Ctx {
            canvas,
            textures
        };



        // world.add_resource(Textures(vec![text_id]));

        'running: loop {
            for event in &mut event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running;
                    }
                    _ => {}
                }
            };

            world.run_systems(&mut ctx);


            thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }

        Ok(())
    }

    fn init() -> Result<(WindowCanvas, EventPump, TextureCreator<WindowContext>), Box<dyn Error>> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        println!("Init");

        let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;

        let window = video_subsystem.window("test", 800, 600)
            .position_centered()
            .resizable()
            .build()
            .expect("could not initialize video subsystem");

        let canvas = window.into_canvas()
            .accelerated()
            .build()
            .expect("could not make a canvas");

        let texture_creator = canvas.texture_creator();
        let event_pump = sdl_context.event_pump()?;

        Ok((
            canvas,
            event_pump,
            texture_creator,
        ))
    }
}