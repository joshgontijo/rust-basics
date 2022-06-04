use sdl2::pixels::Color;
use ecs::{builder, World};
use ecs::component::Fetch;
use ecs::system::Sys;
use crate::engine::Ctx;

mod engine;

#[derive(Debug)]
struct Speed(u32);

#[derive(Debug)]
struct Health(u32);


struct TestSystem;

impl Sys for TestSystem {
    type Query = (Speed, Health);
    type Ctx = Ctx;

    fn run(&self, ctx: &mut Self::Ctx, (speed, health): <Self::Query as Fetch>::Data) {
        // println!("{speed:?} {health:?}");
        engine::fill_rect(ctx, 100, 100, 32, 32, Color::RED);
    }
}


fn main() {
    let mut world = builder::<Ctx>()
        .register_component::<Speed>()
        .register_component::<Health>()
        .build();


    world.new_entity()
        .with_component(Speed(10))
        .with_component(Health(100));

    world.with_system(TestSystem);

    engine::run(world).unwrap();
}