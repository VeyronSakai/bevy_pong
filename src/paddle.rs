use bevy::prelude::*;
use crate::common::*;

pub fn spawn_paddle(commands: &mut Commands, windows: &Res<Windows>, paddle_color_material: &Handle<ColorMaterial>, side_type: PaddleSideType) {
    let window = windows.get_primary().unwrap();

    let sprite_width = 0.3 / 10 as f32 * window.width() as f32;
    let sprite_height = 2.0 / 10 as f32 * window.height() as f32;

    let sprite_size = Vec2::new(sprite_width, sprite_height);

    let pos_x = if side_type == PaddleSideType::Left {
        -window.width() / 2.0 + sprite_width
    } else {
        window.width() / 2.0 - sprite_width
    };

    commands
        .spawn_bundle(SpriteBundle {
            material: (*paddle_color_material).clone(),
            sprite: Sprite {
                size: sprite_size,
                ..Default::default()
            },
            transform: Transform { translation: Vec3::new(pos_x, 0.0, 0.0), ..Default::default() },
            ..Default::default()
        })
        .insert(Paddle { side_type })
        .insert(PaddleVelocity { val: 0.0 })
        .insert(Position { x: 0.0, y: 0.0 });
}

#[derive(PartialEq, Copy, Clone)]
pub enum PaddleSideType {
    Left,
    Right,
}

// Tag Component of Paddle Entity
pub struct Paddle {
    pub side_type: PaddleSideType,
}

pub struct PaddleVelocity {
    pub val: f32,
}

pub fn move_paddle(mut q: Query<(&mut PaddleVelocity, &mut Position), With<Paddle>>) {
    for (velocity, mut pos) in q.iter_mut() {
        pos.y = pos.y + velocity.val * 0.2;
    }
}

pub fn update_paddle_translation(mut q: Query<(&Position, &mut Transform)>) {
    for (pos, mut transform) in q.iter_mut() {
        transform.translation[1] = pos.y;
    }
}

pub struct Materials {
    pub paddle_body_material: Handle<ColorMaterial>,
}