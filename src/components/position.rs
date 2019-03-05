use specs::{ Component, VecStorage };

#[derive(Debug)]
pub struct Position {
    x: f32,
    y: f32,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}
