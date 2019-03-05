extern crate specs;

mod components;
mod systems;
mod resources;

use specs::{ World, Builder, RunNow, DispatcherBuilder, Dispatcher };
use self::components::*;
use self::systems::*;
use self::resources::*;

fn main() {
    run();
}

fn run() {
    // INITIALIZE WORLD
    let mut world = World::new();

    // INITIALIZE DISPATCHER
    let mut dispatcher = DispatcherBuilder::new()
        .with(HelloSystem, "hello_system",  &[])
        .with(UpdatePos,   "update_pos",    &["hello_system"])
        .with(HelloSystem, "hello_updated", &["update_pos"])
        .build();

    // SETUP
    dispatcher.setup(&mut world.res);

    // ADD RESOURCES
    world.add_resource(DeltaTime(0.05));

    // REGISTER COMPONENTS
    // world.register::<Position>();
    // world.register::<Velocity>();

    // CREATE ENTITIES
    world
        .create_entity()
        .with(Position { x: 4.0, y: 7.0 })
        .build();
    world
        .create_entity()
        .with(Position { x: 2.0, y: 5.0 })
        .with(Velocity { x: 0.1, y: 0.2 })
        .build();

    // RUN
    use std::thread::sleep;
    use std::time::Duration;

    for i in 0 .. 10 {
        println!("ITERATION {}", i);

        dispatcher.dispatch(&mut world.res);
        world.maintain();
        let mut dt = world.write_resource::<DeltaTime>();
        *dt = DeltaTime(0.6);

        sleep(Duration::from_millis(500));
    }
}
