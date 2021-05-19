use bevy::prelude::*;

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
        .add_startup_stage("game_setup", SystemStage::single(spawn_paddle.system()))
        .run();
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    // カメラを生成する
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    // Material
    commands.insert_resource(Materials {
        paddle_body_material: materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
    });
    // 背景の色を黒くする
    commands.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)));
}

pub struct Materials {
    pub paddle_body_material: Handle<ColorMaterial>,
}

pub fn spawn_paddle(
    mut commands: Commands,
    materials: Res<Materials>,
) {
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.paddle_body_material.clone(),
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            ..Default::default()
        });
}