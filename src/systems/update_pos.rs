use specs::{ System, ReadStorage, WriteStorage, ReadExpect, Join };
use crate::components::*;
use crate::resources::DeltaTime;

pub struct UpdatePos;

impl<'a> System<'a> for UpdatePos {
    type SystemData = (ReadExpect<'a, DeltaTime>,
                       ReadStorage<'a, Velocity>,
                       WriteStorage<'a, Position>);

    fn run(&mut self, data: Self::SystemData) {
        let (dt, vel, mut pos) = data;
        let dt = dt.0;

        for (vel, pos) in (&vel, &mut pos).join() {
            pos.x += vel.x * dt;
            pos.y += vel.y * dt;
        }
    }
}
