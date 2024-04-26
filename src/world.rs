use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

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
                shadows_enabled: true,
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
            mesh: meshes.add(Plane3d::default().mesh().size(8.0, 8.0)),
            material: materials.add(StandardMaterial {
                base_color_texture: Some(texture_handle.clone()),
                ..default()
            }),
            ..default()
        },
        Name::new("Floor"),
        RigidBody::Fixed,
        Collider::cuboid(4.0, 0.0, 4.0),
    );

    commands.spawn(floor);
}

fn spawn_obstacles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: Res<AssetServer>,
) {
    let blue_cube = (
        PbrBundle {
            mesh: meshes.add(Cuboid::default().mesh()),
            material: materials.add(StandardMaterial {
                base_color_texture: Some(assets.load("grid_textures/Blue/g1800.png").clone()),
                ..default()
            }),
            transform: Transform::from_xyz(-1.5, 0.0, -2.5),
            ..default()
        },
        Name::new("Blue cube"),
        RigidBody::Fixed,
        Collider::cuboid(0.5, 0.5, 0.5),
    );

    let blue_cube2 = (
        PbrBundle {
            mesh: meshes.add(Cuboid::default().mesh()),
            material: materials.add(StandardMaterial {
                base_color_texture: Some(assets.load("grid_textures/Blue/g1800.png").clone()),
                ..default()
            }),
            transform: Transform::from_xyz(-0.5, -0.1, -2.5),
            ..default()
        },
        Name::new("Blue cube"),
        RigidBody::Fixed,
        Collider::cuboid(0.5, 0.5, 0.5),
    );

    let blue_cube3 = (
        PbrBundle {
            mesh: meshes.add(Cuboid::default().mesh()),
            material: materials.add(StandardMaterial {
                base_color_texture: Some(assets.load("grid_textures/Blue/g1800.png").clone()),
                ..default()
            }),
            transform: Transform::from_xyz(0.5, -0.2, -2.5),
            ..default()
        },
        Name::new("Blue cube"),
        RigidBody::Fixed,
        Collider::cuboid(0.5, 0.5, 0.5),
    );

    let blue_cube4 = (
        PbrBundle {
            mesh: meshes.add(Cuboid::default().mesh()),
            material: materials.add(StandardMaterial {
                base_color_texture: Some(assets.load("grid_textures/Blue/g1800.png").clone()),
                ..default()
            }),
            transform: Transform::from_xyz(1.5, -0.3, -2.5),
            ..default()
        },
        Name::new("Blue cube"),
        RigidBody::Fixed,
        Collider::cuboid(0.5, 0.5, 0.5),
    );

    let blue_cube5 = (
        PbrBundle {
            mesh: meshes.add(Cuboid::default().mesh()),
            material: materials.add(StandardMaterial {
                base_color_texture: Some(assets.load("grid_textures/Blue/g1800.png").clone()),
                ..default()
            }),
            transform: Transform::from_xyz(2.5, -0.4, -2.5),
            ..default()
        },
        Name::new("Blue cube"),
        RigidBody::Fixed,
        Collider::cuboid(0.5, 0.5, 0.5),
    );

    let yellow_cube = (
        PbrBundle {
            mesh: meshes.add(Cuboid::default().mesh().scaled_by(Vec3::splat(1.5))),
            material: materials.add(StandardMaterial {
                base_color_texture: Some(assets.load("grid_textures/Yellow/g2905.png").clone()),
                ..default()
            }),
            transform: Transform::from_xyz(2.0, 0.75, 2.0),
            ..default()
        },
        Name::new("Yellow cube"),
        RigidBody::Fixed,
        Collider::cuboid(0.75, 0.75, 0.75),
    );

    commands.spawn(blue_cube);
    commands.spawn(blue_cube2);
    commands.spawn(blue_cube3);
    commands.spawn(blue_cube4);
    commands.spawn(blue_cube5);
    commands.spawn(yellow_cube);
}
