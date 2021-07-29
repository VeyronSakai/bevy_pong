use bevy::prelude::Vec2;

pub struct Position {
    pub x: f32,
    pub y: f32,
}

pub struct Velocity {
    pub value: Vec2,
}

impl Velocity {
    pub fn new(value: Vec2) -> Self {
        Velocity { value: value.normalize() }
    }
}