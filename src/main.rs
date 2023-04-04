use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::PresentMode};
use bevy_window::PrimaryWindow;

const BOUNDS: Vec2 = Vec2::new(800.0, 640.0);

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct Ball;

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
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();

    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 0.),
            ..default()
        },
        MainCamera,
    ));
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(50.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 0.),
            ..default()
        },
        Ball,
    ));
}

fn ball_movement(
    mut ball_q: Query<(&mut Ball, &mut Transform)>,
    primary_window_q: Query<&Window, With<PrimaryWindow>>,
    main_camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = main_camera_q.single();
    let window = primary_window_q.single();

    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates,
    // and truncate Z coordinate
    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        if let Ok((mut _ball, mut transform)) = ball_q.get_single_mut() {
            transform.translation.x = world_position.x;
            transform.translation.y = world_position.y;
        }
    }
}

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
