use raylib::{color::Color, ffi::Vector2};

pub struct ElementSolid{
    mass: f32,
    colour: Color,
    position: Vector2,
}


impl ElementSolid {
    pub fn new(mass: f32, colour: Color, position: Vector2) -> ElementSolid{
        ElementSolid {
            mass,
            colour,
            position: Vector2 { x: 0.0, y: 0.0 },
        }
    }
} 