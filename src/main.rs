use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(
            Startup,
            (spawn_camera, spawn_light, spawn_floor, spawn_player),
        )
        .run();
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let player: MaterialMeshBundle<StandardMaterial> = PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube::new(1.0))),
        material: materials.add(Color::BLUE.into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    };

    commands.spawn(player);
}

fn spawn_light(mut commands: Commands) {
    let light = PointLightBundle {
        point_light: PointLight {
            intensity: 2000.0,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 5.0, 0.0),
        ..default()
    };

    commands.spawn(light);
}

fn spawn_camera(mut commands: Commands) {
    let camera = Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    };

    commands.spawn(camera);
}

fn spawn_floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let floor = PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane::from_size(15.0))),
        material: materials.add(Color::DARK_GREEN.into()),
        ..default()
    };

    commands.spawn(floor);
}
