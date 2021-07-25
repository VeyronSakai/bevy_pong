use bevy::prelude::*;

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
        .add_startup_stage("game_setup", SystemStage::single(size_scaling.system()))
        .add_system_to_stage(CoreStage::PreUpdate, handle_input.system())
        .add_system_set_to_stage(
            CoreStage::Update,
            SystemSet::new()
                .with_system(move_paddle.system())
                .with_system(position_translation.system()),
        )
        .run();
}

fn handle_input(input: Res<Input<KeyCode>>, mut paddles: Query<(&mut PaddleVelocity), With<Paddle>>) {
    if let Some(mut velocity) = paddles.iter_mut().next() {
        velocity.val = 0.0;

        if input.pressed(KeyCode::Up) {
            velocity.val += PADDLE_SPEED;
        }
        if input.pressed(KeyCode::Down) {
            velocity.val += -PADDLE_SPEED;
        }
    }
}

fn position_translation(mut q: Query<(&Position, &mut Transform)>) {
    for (pos, mut transform) in q.iter_mut() {
        transform.translation[1] = pos.y;
    }
}

// Tag Component of Paddle Entity
struct Paddle;

struct PaddleVelocity {
    val: f32,
}

fn move_paddle(mut q: Query<(&mut PaddleVelocity, &mut Position), With<Paddle>>) {
    for (velocity, mut pos) in q.iter_mut() {
        pos.y = pos.y + velocity.val * 0.2;
    }
}

struct Position {
    x: f32,
    y: f32,
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
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

    commands
        .spawn_bundle(SpriteBundle {
            material: paddle_color_material.clone(),
            ..Default::default()
        })
        .insert(PaddleSize::new(0.3, 1.0))
        .insert(Paddle)
        .insert(PaddleVelocity { val: 0.0 })
        .insert(Position { x: 0.0, y: 0.0 });
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
        .insert(PaddleSize::new(0.3, 1.0))
        .insert(Paddle)
        .insert(PaddleVelocity { val: 0.0 })
        .insert(Position { x: 0.0, y: 0.0 });
}

fn size_scaling(windows: Res<Windows>, mut q: Query<(&PaddleSize, &mut Sprite)>) {
    let window = windows.get_primary().unwrap();
    for (sprite_size, mut sprite) in q.iter_mut() {
        sprite.size = Vec2::new(
            sprite_size.width / 10 as f32 * window.width() as f32,
            sprite_size.height / 10 as f32 * window.height() as f32,
        );
    }
}

pub struct PaddleSize {
    pub width: f32,
    pub height: f32,
}

impl PaddleSize {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            width: x,
            height: y,
        }
    }
}