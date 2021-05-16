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
        .run();
}

fn setup(mut commands: Commands) {
    // カメラを生成する
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    // 背景の色を黒くする
    commands.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)));
}