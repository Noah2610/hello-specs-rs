extern crate specs;

mod components;
mod systems;

use specs::{ World, Builder, RunNow, DispatcherBuilder };
use self::components::*;
use self::systems::*;

fn main() {
    let mut world = World::new();
    world.register::<Position>();
    world.register::<Velocity>();

    world
        .create_entity()
        .with(Position { x: 4.0, y: 7.0 })
        .build();
    world
        .create_entity()
        .with(Position { x: 2.0, y: 5.0 })
        .with(Velocity { x: 0.1, y: 0.2 })
        .build();

    // let mut hello_system = HelloSystem;
    // hello_system.run_now(&world.res);
    // world.maintain();

    let mut dispatcher = DispatcherBuilder::new()
        .with(HelloSystem, "hello_system",  &[])
        .with(UpdatePos,   "update_pos",    &["hello_system"])
        .with(HelloSystem, "hello_updated", &["update_pos"])
        .build();

    dispatcher.dispatch(&mut world.res);
}
