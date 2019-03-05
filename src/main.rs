extern crate specs;

mod components;
mod systems;

use specs::{ World, Builder, RunNow };
use self::components::*;
use self::systems::*;

fn main() {
    let mut world = World::new();
    world.register::<Position>();

    let ball = world
        .create_entity()
        .with(Position::new(4.0, 7.0))
        .build();

    let mut sys = HelloSystem;
    sys.run_now(&world.res);
    world.maintain();
}
