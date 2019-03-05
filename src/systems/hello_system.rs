use specs::{ System, ReadStorage, Join };
use crate::components::*;

pub struct HelloSystem;

impl<'a> System<'a> for HelloSystem {
    type SystemData = ReadStorage<'a, Position>;

    fn run(&mut self, position: Self::SystemData) {
        for pos in position.join() {
            println!("Hello {:?}", pos);
        }
    }
}
