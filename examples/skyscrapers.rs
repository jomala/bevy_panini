use bevy::prelude::*;
use bevy_panini::prelude::*;

fn main() -> AppExit {
    App::new()
        .add_plugins((DefaultPlugins, PaniniPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, (rotate, update_settings))
        .run()
}

/// Set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)).looking_at(Vec3::new(1.0, 1.5, 0.0), Vec3::Y),
        Camera {
            clear_color: Color::WHITE.into(),
            ..default()
        },
        // Add the setting to the camera.
        // This component is also used to determine on which camera to run the post processing effect.
        PaniniSettings::new(1.0),
        Rotates,
    ));

    // cuboids
    let mesh = meshes.add(Cuboid::default());

    commands.spawn((
        Mesh3d(mesh.clone()),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
        Transform::from_xyz(1.0, 1.0, 0.0).with_scale(Vec3::new(0.8, 2.0, 0.8)),
    ));
    commands.spawn((
        Mesh3d(mesh.clone()),
        MeshMaterial3d(materials.add(Color::srgb(0.74, 0.80, 0.60))),
        Transform::from_xyz(-1.0, 1.5, 0.0).with_scale(Vec3::new(0.8, 3.0, 0.8)),
    ));
    commands.spawn((
        Mesh3d(mesh.clone()),
        MeshMaterial3d(materials.add(Color::srgb(0.60, 0.72, 0.80))),
        Transform::from_xyz(0.0, 2.0, 1.0).with_scale(Vec3::new(0.8, 4.0, 0.8)),
    ));
    commands.spawn((
        Mesh3d(mesh.clone()),
        MeshMaterial3d(materials.add(Color::srgb(0.76, 0.80, 0.60))),
        Transform::from_xyz(0.0, 2.5, -1.0).with_scale(Vec3::new(0.8, 5.0, 0.8)),
    ));

    // light
    commands.spawn(DirectionalLight {
        illuminance: 1_000.,
        ..default()
    });
}

#[derive(Component)]
struct Rotates;

/// Rotates any entity around the y axis
fn rotate(time: Res<Time>, mut query: Query<&mut Transform, With<Rotates>>) {
    for mut transform in &mut query {
        transform.rotate_y(0.15 * time.delta_secs());
    }
}

// Change the intensity over time to show that the effect is controlled from the main world
fn update_settings(mut settings: Query<&mut PaniniSettings>, time: Res<Time>) {
    // for mut setting in &mut settings {
    //     let mut intensity = ops::sin(time.elapsed_secs());
    //     // Make it loop periodically
    //     intensity = ops::sin(intensity);
    //     // Remap it to 0..1 because the intensity can't be negative
    //     intensity = intensity * 0.5 + 0.5;
    //     // Scale it to a more reasonable level
    //     intensity *= 0.015;

    //     // Set the intensity.
    //     // This will then be extracted to the render world and uploaded to the GPU automatically by the [`UniformComponentPlugin`]
    //     setting.intensity = intensity;
    // }
}
