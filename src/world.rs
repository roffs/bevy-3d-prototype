use std::f32::consts::PI;

use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_light, spawn_floor, spawn_obstacles).chain());
    }
}

fn spawn_light(mut commands: Commands) {
    let light = (
        PointLightBundle {
            point_light: PointLight {
                intensity: 2000.0,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 5.0, 0.0),
            ..default()
        },
        Name::new("Main light"),
    );

    commands.spawn(light);
}

fn spawn_floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: Res<AssetServer>,
) {
    let texture_handle = assets.load("floor_texture.png");
    let floor = (
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane::from_size(15.0))),
            material: materials.add(StandardMaterial {
                base_color_texture: Some(texture_handle.clone()),
                // alpha_mode: AlphaMode::Opaque,
                // unlit: true,
                ..default()
            }),
            ..default()
        },
        Name::new("Floor"),
    );

    commands.spawn(floor);
}

fn spawn_obstacles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: Res<AssetServer>,
) {
    let blue_cube = cube_bundle(
        &mut meshes,
        &mut materials,
        Transform::from_xyz(-5.0, 0.5, -3.25),
        assets.load("grid_textures/Blue/g1800.png"),
        String::from("Blue cube"),
    );

    let yellow_cube = cube_bundle(
        &mut meshes,
        &mut materials,
        Transform::from_xyz(3.4, 0.75, 1.5).with_scale(Vec3::splat(1.5)),
        assets.load("grid_textures/Yellow/g2905.png"),
        String::from("Yellow cube"),
    );

    commands.spawn(blue_cube);
    commands.spawn(yellow_cube);
}

fn cube_bundle(
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    transform: Transform,
    texture_handle: Handle<Image>,
    name: String,
) -> (MaterialMeshBundle<StandardMaterial>, Name) {
    (
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube::new(1.0))),
            material: materials.add(StandardMaterial {
                base_color_texture: Some(texture_handle.clone()),
                ..default()
            }),
            transform,
            ..default()
        },
        Name::new(name),
    )
}
