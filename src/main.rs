use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
    window::PresentMode,
};
use bevy_window::PrimaryWindow;

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

fn ball_movement(
    mut ball_position: Query<(&mut Ball, &mut Transform)>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    
    let window = window_query.get_single().unwrap();

    if let Ok((mut _ball, mut transform)) = ball_position.get_single_mut() {
        for event in cursor_moved_events.iter() {
            transform.translation.x = event.position.x - window.width() / 2.;
            transform.translation.y = event.position.y - window.height() / 2.;
        }
    }
}

#[derive(Resource)]
struct GreetTimer(Timer);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bumi".into(),
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
