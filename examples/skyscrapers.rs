use bevy::{
    input_focus::tab_navigation::TabGroup, prelude::*, reflect::TypePath,
    render::render_resource::AsBindGroup, shader::ShaderRef, window::WindowResolution,
};

use bevy_panini::prelude::*;

mod helpers;
use helpers::helper_sliders::{
    SliderScaledValue, ValueLabel, VerticalSliderPlugin, spawn_vertical_slider_ui,
};

fn main() -> AppExit {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: WindowResolution::new(2048, 1024),
            title: "Panini Skyscrapers".into(),
            ..default()
        }),
        ..default()
    }))
    .add_plugins((PaniniPlugin, VerticalSliderPlugin))
    .add_plugins(MaterialPlugin::<GroundMaterial>::default())
    .add_plugins(MaterialPlugin::<WallMaterial>::default())
    .add_systems(Startup, setup)
    .add_systems(Update, (rotate, settings_change));

    app.run()
}

// Define the custom material types
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct GroundMaterial {}

impl Material for GroundMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/skyscrapers_ground.wgsl".into()
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct WallMaterial {}

impl Material for WallMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/skyscrapers_walls.wgsl".into()
    }
}

/// Set up a simple 3D scene with 1 unit being about 10 metres.
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
    mut ground_materials: ResMut<Assets<GroundMaterial>>,
    mut wall_materials: ResMut<Assets<WallMaterial>>,
    assets: Res<AssetServer>,
) {
    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(Vec3::new(0.0, 0.2, 0.0))
            .looking_at(Vec3::new(1.0, 1.5, 0.0), Vec3::Y),
        Camera {
            clear_color: Color::srgb(0.48, 0.62, 0.77).into(),
            ..default()
        },
        // Add the setting to the camera.
        // This component is also used to determine on which camera to run the post processing effect.
        Projection::custom(
            PaniniProjection::new()
                .with_panini_depth(0.5)
                .with_fov_y(0.8),
        ),
        Rotates,
    ));

    // skyscrapers
    let mesh = meshes.add(Cuboid::default());
    let wall_material = wall_materials.add(WallMaterial {});

    commands.spawn((
        Mesh3d(mesh.clone()),
        MeshMaterial3d(wall_material.clone()),
        // MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
        Transform::from_xyz(1.0, 1.0, 0.0).with_scale(Vec3::new(0.8, 2.0, 0.8)),
    ));
    commands.spawn((
        Mesh3d(mesh.clone()),
        MeshMaterial3d(wall_material.clone()),
        // MeshMaterial3d(materials.add(Color::srgb(0.74, 0.80, 0.60))),
        Transform::from_xyz(-1.0, 1.5, 0.0).with_scale(Vec3::new(0.8, 3.0, 0.8)),
    ));
    commands.spawn((
        Mesh3d(mesh.clone()),
        MeshMaterial3d(wall_material.clone()),
        // MeshMaterial3d(materials.add(Color::srgb(0.60, 0.72, 0.80))),
        Transform::from_xyz(0.0, 2.0, 1.0).with_scale(Vec3::new(0.8, 4.0, 0.8)),
    ));
    commands.spawn((
        Mesh3d(mesh.clone()),
        MeshMaterial3d(wall_material),
        // MeshMaterial3d(materials.add(Color::srgb(0.76, 0.80, 0.60))),
        Transform::from_xyz(0.0, 2.5, -1.0).with_scale(Vec3::new(0.8, 5.0, 0.8)),
    ));

    // ground
    let ground_material = ground_materials.add(GroundMaterial {});

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::new(3.0, 3.0)))),
        MeshMaterial3d(ground_material),
        // MeshMaterial3d(materials.add(Color::srgb(0.10, 0.72, 0.1))),
        Transform::from_xyz(0.0, -0.01, 0.0).with_scale(Vec3::new(10.0, 0.02, 10.0)),
    ));

    // light
    commands.spawn(DirectionalLight {
        illuminance: 1_000.,
        ..default()
    });
    commands.spawn((
        DirectionalLight {
            illuminance: 1_000.,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::YXZ, 0.5, -0.5, 0.0)),
    ));

    // UI
    commands
        .spawn((
            Node {
                width: percent(100),
                height: percent(100),
                align_items: AlignItems::Start,
                justify_content: JustifyContent::FlexEnd,
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                column_gap: px(50),
                ..default()
            },
            TabGroup::default(),
        ))
        .with_children(|parent| {
            spawn_vertical_slider_ui("Tilt", 20.0, 0.0, 90.0, parent, &assets);
            spawn_vertical_slider_ui("FOV Y", 120.0, 10.0, 170.0, parent, &assets);
            spawn_vertical_slider_ui("Panini\nDepth", 0.0, 0.0, 2.0, parent, &assets);
            spawn_vertical_slider_ui("Hard\nCompr", 0.0, 0.0, 1.0, parent, &assets);
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

fn settings_change(
    mut projection_query: Query<(&mut Projection, &mut Transform)>,
    slider_query: Query<(&ValueLabel, &SliderScaledValue), Changed<SliderScaledValue>>,
) {
    let mut fov_y = None;
    let mut panini_depth = None;
    let mut compression = None;
    let mut tilt = None;

    for (label, slider_value) in &slider_query {
        match label.0.as_str() {
            "Panini\nDepth" => panini_depth = Some(slider_value.0),
            "FOV Y" => fov_y = Some(slider_value.0.to_radians()),
            "Hard\nCompr" => compression = Some(slider_value.0),
            "Tilt" => tilt = Some(slider_value.0.to_radians()),
            _ => (),
        }
    }

    for (mut projection, mut transform) in &mut projection_query {
        if let Projection::Custom(projection) = &mut *projection {
            if let Some(projection) = projection.get_mut::<PaniniProjection>() {
                if let Some(fov_y) = fov_y {
                    projection.update_fov_y(fov_y);
                }
                if let Some(panini_depth) = panini_depth {
                    projection.update_panini_depth(panini_depth);
                    projection.update_enabled(panini_depth > 0.001);
                }
                if let Some(compression) = compression {
                    projection.update_compression(compression);
                }
                if let Some(tilt) = tilt {
                    let (yaw, _current_tilt, roll) = transform.rotation.to_euler(EulerRot::YXZ);
                    transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, tilt, roll);
                }
            }
        }
    }
}
