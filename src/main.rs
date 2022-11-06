/**
 * simple_bevy_scene
 * Purpose: Undertanding how making a simple scene works
 * Edward Naidoo - Tutorial by Logic Projects
 * Nov 6, 2022
 */
use bevy::prelude::*;

// Window sizing
pub const HEIGHT:f32 = 720.0;
pub const WIDTH:f32 = 1280.0;

/**
 * Main Camera
 */
fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

/**
 * Creating everything inside of a Scene
 */
fn spawn_basic_scene(
    mut commands: Commands, 
    mut mesh: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    // Plane/Ground
    commands.spawn_bundle(PbrBundle {
        mesh: mesh.add(Mesh::from(shape::Plane {size:5.0})),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    // Cube
    commands.spawn_bundle(PbrBundle {
        mesh: mesh.add(Mesh::from(shape::Cube {size:1.0})),
        material: materials.add(Color::rgb(0.67, 0.84, 0.92).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    }); 
}
fn main() {
    // App
    App::new()
        // World Colour is Black 
        .insert_resource(ClearColor(Color::BLACK))
        // Needed to determine App Window size
        .insert_resource(WindowDescriptor{
            width: WIDTH,
            height: HEIGHT,
            title: "simple bevy scene".to_string(),
            ..Default::default()
        })
        // Loads the Functions I made
        .add_startup_system(spawn_basic_scene)
        .add_startup_system(spawn_camera)
        .add_plugins(DefaultPlugins)
        .run();
}