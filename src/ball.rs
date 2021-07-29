use bevy::prelude::*;
use crate::common::Velocity;

const BALL_SPEED: f32 = 3.0;

pub struct Ball;

pub fn update_ball_translation(mut query: Query<(&Velocity, &mut Transform), With<Ball>>) {
    if let Ok((velocity, mut transform)) = query.single_mut() {
        transform.translation.x += velocity.value.x * BALL_SPEED;
        transform.translation.y += velocity.value.y * BALL_SPEED;
    }
}