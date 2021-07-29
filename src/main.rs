mod paddle;
mod common;

use bevy::prelude::*;
use crate::paddle::*;
use crate::common::*;
use bevy::sprite::collide_aabb::*;

const PADDLE_SPEED: f32 = 20.0;
const BALL_SPEED: f32 = 3.0;

fn main() {
    App::build()
        // ウインドウの生成
        .insert_resource(WindowDescriptor {
            title: "Pong".to_string(),
            width: 500.0,
            height: 500.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        // 初期化処理。StartUp Stageで実行される。
        .add_startup_system(setup.system())
        .add_system_to_stage(CoreStage::PreUpdate, handle_input.system())
        .add_system_set_to_stage(
            CoreStage::Update,
            SystemSet::new()
                .with_system(move_paddle.system())
                .with_system(update_paddle_translation.system())
                .with_system(update_ball_translation.system())
                .with_system(ball_collide_paddle.system())
                .with_system(ball_collide_wall.system()),
        )
        .run();
}

fn update_ball_translation(mut query: Query<(&Velocity, &mut Transform), With<Ball>>) {
    if let Ok((velocity, mut transform)) = query.single_mut() {
        transform.translation.x += velocity.value.x * BALL_SPEED;
        transform.translation.y += velocity.value.y * BALL_SPEED;
    }
}

fn ball_collide_wall(mut query: Query<(&Transform, &mut Velocity), With<Ball>>, windows: Res<Windows>) {
    if let Ok((transform, mut velocity)) = query.single_mut() {
        let window = windows.get_primary().unwrap();
        if transform.translation.y < -window.height() / 2.0 {
            velocity.value.y = -velocity.value.y;
        }

        if transform.translation.y > window.height() / 2.0 {
            velocity.value.y = -velocity.value.y;
        }
    }
}

fn ball_collide_paddle(mut ball_query: Query<(&Transform, &Sprite, &mut Velocity), With<Ball>>, paddle_query: Query<(&Transform, &Sprite), With<Paddle>>) {
    if let Ok((ball_transform, ball_sprite, mut ball_velocity)) = ball_query.single_mut() {
        for (paddle_transform, paddle_sprite) in paddle_query.iter() {
            let collision = collide(
                ball_transform.translation,
                ball_sprite.size,
                paddle_transform.translation,
                paddle_sprite.size,
            );

            let collision = match collision {
                Some(collision) => collision,
                None => continue,
            };

            let mut reflect_x = false;
            let mut reflect_y = false;

            match collision {
                Collision::Left => reflect_x = ball_velocity.value[0] > 0.0,
                Collision::Right => reflect_x = ball_velocity.value[0] < 0.0,
                Collision::Top => reflect_y = ball_velocity.value[1] < 0.0,
                Collision::Bottom => reflect_y = ball_velocity.value[1] > 0.0,
            };

            if reflect_x {
                ball_velocity.value[0] = -ball_velocity.value[0];
            }

            if reflect_y {
                ball_velocity.value[1] = -ball_velocity.value[1];
            }
        }
    }
}

fn handle_input(input: Res<Input<KeyCode>>, mut paddles: Query<(&mut PaddleVelocity, &Paddle)>) {
    for (mut velocity, paddle) in paddles.iter_mut() {
        velocity.val = 0.0;

        if paddle.side_type == PaddleSideType::Left {
            if input.pressed(KeyCode::W) {
                velocity.val += PADDLE_SPEED;
            }

            if input.pressed(KeyCode::S) {
                velocity.val += -PADDLE_SPEED;
            }
        } else {
            if input.pressed(KeyCode::Up) {
                velocity.val += PADDLE_SPEED;
            }

            if input.pressed(KeyCode::Down) {
                velocity.val += -PADDLE_SPEED;
            }
        }
    }
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>, windows: Res<Windows>) {
    let paddle_color = Color::rgb(0.7, 0.7, 0.7);

    let paddle_color_material = materials.add(paddle_color.into());

    // カメラを生成する
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    // Material
    commands.insert_resource(Materials {
        paddle_body_material: paddle_color_material.clone(),
    });
    // 背景の色を黒くする
    commands.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)));

    // Paddleの生成
    spawn_paddle(&mut commands, &windows, &paddle_color_material, PaddleSideType::Left);
    spawn_paddle(&mut commands, &windows, &paddle_color_material, PaddleSideType::Right);

    let ball_color = Color::rgb(0.7, 0.7, 0.7);

    let ball_color_material = materials.add(ball_color.into());

    let window = windows.get_primary().unwrap();

    let sprite_width = 0.3 / 10 as f32 * window.width() as f32;
    let sprite_height = 0.3 / 10 as f32 * window.height() as f32;

    let sprite_size = Vec2::new(sprite_width, sprite_height);

    commands.spawn_bundle(
        SpriteBundle {
            material: ball_color_material.clone(),
            sprite: Sprite {
                size: sprite_size,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Ball)
        .insert(Velocity::new(Vec2::new(1.0, -1.0)));
}

pub struct Ball;

pub struct Velocity {
    pub value: Vec2,
}

impl Velocity {
    fn new(value: Vec2) -> Self {
        Velocity { value: value.normalize() }
    }
}