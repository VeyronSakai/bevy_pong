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
    spawn_paddle(&mut commands, windows, paddle_color_material);
}

fn spawn_paddle(commands: &mut Commands, windows: Res<Windows>, paddle_color_material: Handle<ColorMaterial>) {
    let window = windows.get_primary().unwrap();

    let sprite_width = 0.3 / 10 as f32 * window.width() as f32;
    let sprite_height = 2.0 / 10 as f32 * window.height() as f32;

    let sprite_size = Vec2::new(sprite_width, sprite_height);

    commands
        .spawn_bundle(SpriteBundle {
            material: paddle_color_material.clone(),
            sprite: Sprite {
                size: sprite_size,
                ..Default::default()
            },
            transform: Transform { translation: Vec3::new(-window.width() / 2.0 + sprite_width, 0.0, 0.0), ..Default::default() },
            ..Default::default()
        })
        .insert(Paddle)
        .insert(PaddleVelocity { val: 0.0 })
        .insert(Position { x: 0.0, y: 0.0 });
}

pub struct Materials {
    pub paddle_body_material: Handle<ColorMaterial>,
}
