use std::error::Error;
use std::time::Duration;
use sdl2::event::Event;
use sdl2::{image, Sdl};
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;
use ecs::World;


pub struct Ctx {
    canvas: WindowCanvas,

}

pub fn run(mut world: World<Ctx>) -> Result<(), Box<dyn Error>> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    println!("Init");

    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;

    let window = video_subsystem.window("game tutorial", 800, 600)
        .position_centered()
        .resizable()
        .build()
        .expect("could not initialize video subsystem");

    let canvas = window.into_canvas()
        .accelerated()
        .build()
        .expect("could not make a canvas");

    let mut ctx = Ctx {
        canvas,
    };

    // let texture_creator = canvas.texture_creator();
    // let texture = texture_creator.load_texture("assets/reaper.png")?;

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }
                _ => {}
            }
        }

        ctx.canvas.set_draw_color(Color::BLACK);
        ctx.canvas.clear();


        world.run_systems(&mut ctx);

        ctx.canvas.present();

        // Time management!
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}

pub fn fill_rect(ctx: &mut Ctx, x: i32, y: i32, w: u32, h: u32, color: Color) {
    ctx.canvas.set_draw_color(color);
    ctx.canvas.fill_rect(Rect::new(x, y, w, h)).unwrap();
}
