mod paddle;
mod common;

use bevy::prelude::*;
use crate::paddle::*;
use crate::common::*;

const PADDLE_SPEED: f32 = 20.0;

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
                .with_system(position_translation.system()),
        )
        .run();
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
}
