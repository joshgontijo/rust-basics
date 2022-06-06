use std::error::Error;
use sdl2::pixels::Color;
use ecs::World;
use crate::engine::Engine;

mod engine;

#[derive(Debug)]
struct Speed(i32);

#[derive(Debug)]
struct Tile {
    x: i32,
    y: i32,
    w: u32,
    h: u32,
}

impl Tile {
    pub fn new(x: i32, y: i32, w: u32, h: u32) -> Self {
        Self { x, y, w, h }
    }
}

#[derive(Debug)]
struct Drawable {
    color: Color,
}


fn main() -> Result<(), Box<dyn Error>> {
    let mut world = ecs::builder()
        .register_component::<Speed>()
        .register_component::<Tile>()
        .register_component::<Drawable>()
        .build();


    let _ = world.new_entity()
        .with_component(Tile::new(200, 200, 50, 50))
        .with_component(Drawable { color: Color::CYAN })
        .id();


    Engine::run(world)
}

mod systems {
    use sdl2::pixels::Color;
    use sdl2::rect::Rect;
    use crate::{Drawable, Tile};
    use crate::engine::Ctx;

    pub fn render(ctx: &mut Ctx, (tile, drawable, ): (Tile, Drawable, )) {
        ctx.canvas.set_draw_color(Color::BLACK);
        ctx.canvas.clear();

        ctx.canvas.set_draw_color(drawable.color);
        ctx.canvas.draw_rect(Rect::new(tile.x,tile.y, tile.w, tile.h)).unwrap();

        ctx.canvas.present();

    }
}