use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::PresentMode};

const BOUNDS: Vec2 = Vec2::new(800.0, 640.0);

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Person;

enum Direction {
    WEST,
    EAST,
}

#[derive(Component)]
struct Ball {
    direction: Direction,
}
pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_ball).add_system(ball_movement);
    }
}

fn setup_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(50.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            transform: Transform::from_xyz(0. - BOUNDS.x, 0., 0.),
            ..default()
        },
        Ball {
            direction: Direction::EAST,
        },
    ));
}

fn ball_movement(mut ball_position: Query<(&mut Ball, &mut Transform)>) {
    for (mut ball, mut transform) in &mut ball_position {
        if transform.translation.x >= BOUNDS.x {
            ball.direction = Direction::WEST;
        } else if transform.translation.x <= -BOUNDS.x {
            ball.direction = Direction::EAST;
        }

        match ball.direction {
            Direction::WEST => transform.translation.x -= 1.,
            Direction::EAST => transform.translation.x += 1.,
        }
    }
}

#[derive(Resource)]
struct GreetTimer(Timer);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "I am a window!".into(),
                resolution: (BOUNDS.x, BOUNDS.y).into(),
                present_mode: PresentMode::AutoVsync,
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(BallPlugin)
        .run();
}
