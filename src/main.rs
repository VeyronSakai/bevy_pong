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
        // PostUpdateで実行したいので、add_system_setではなく、add_system_set_to_stageを使う
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(size_scaling.system()),
        )
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
            ..Default::default()
        })
        .insert(SpriteSize::new(0.3, 1.));
}

fn size_scaling(windows: Res<Windows>, mut q: Query<(&SpriteSize, &mut Sprite)>) {
    let window = windows.get_primary().unwrap();
    for (sprite_size, mut sprite) in q.iter_mut() {
        sprite.size = Vec2::new(
            sprite_size.width / 10 as f32 * window.width() as f32,
            sprite_size.height / 10 as f32 * window.height() as f32,
        );
    }
}

pub struct SpriteSize {
    pub width: f32,
    pub height: f32,
}

impl SpriteSize {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            width: x.clone(),
            height: y.clone(),
        }
    }
}