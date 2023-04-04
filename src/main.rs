use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::PresentMode};
use bevy_window::PrimaryWindow;

const BOUNDS: Vec2 = Vec2::new(800.0, 640.0);

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct Knot;

#[derive(Component)]
struct HeadKnot;

#[derive(Component)]
struct Rope {
    chain: Vec<Knot>,
}

pub struct RopePlugin;

impl Plugin for RopePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_knots).add_system(knot_movement);
    }
}

fn setup_knots(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window_q: Query<&Window, With<PrimaryWindow>>,
    // rope_q: Query<&Rope>,
) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
    let window = window_q.single();
    // let rope = rope_q.single();

    // for _ in &rope.chain[1..] {
    //     commands.spawn((
    //         MaterialMesh2dBundle {
    //             mesh: meshes.add(shape::Circle::new(50.).into()).into(),
    //             material: materials.add(ColorMaterial::from(Color::PURPLE)),
    //             transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 0.),
    //             ..default()
    //         },
    //         Knot,
    //     ));
    // }
    //
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(20.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::GREEN)),
            transform: Transform::from_xyz(window.width() / 3., window.height() / 2., 0.),
            ..default()
        },
        Knot
    ));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(50.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 1.),
            ..default()
        },
        (Knot, HeadKnot)
    ));
}

fn knot_movement(
    mut knot_q: Query<(&mut Knot, &mut Transform), With<HeadKnot>>,
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
        if let Ok((_, mut transform)) = knot_q.get_single_mut() {
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
        .add_plugin(RopePlugin)
        .run();
}
