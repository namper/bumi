use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Position {
    vec3: Vec3,
}

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Ran d".to_string())));
    commands.spawn((Person, Name("York d".to_string())));
    commands.spawn((Person, Name("Shel k".to_string())));
}

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("Hello {}!", name.0)
        }
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_startup_system(add_people)
            .add_system(greet_people);
    }
}

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_systems((add_ball, add_camera, draw_ball))
            .add_system(ball_movement);
    }
}

fn add_ball(mut commands: Commands) {
    commands.spawn((
        Ball,
        Position {
            vec3: Vec3::new(-150.0, 0.0, 0.0),
        },
    ));
}

fn add_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn draw_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<&Position, With<Ball>>,
) {
    for pos in &query {
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(50.).into()).into(),
                material: materials.add(ColorMaterial::from(Color::PURPLE)),
                transform: Transform::from_translation(pos.vec3),
                ..default()
            },
            Ball,
        ));
    }
}

fn ball_movement(mut ball_position: Query<(&mut Ball, &mut Transform)>) {
    for (_ball, mut transform) in &mut ball_position {
        transform.translation.x += 1.;
    }
}

#[derive(Resource)]
struct GreetTimer(Timer);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(BallPlugin)
        .run();
}
