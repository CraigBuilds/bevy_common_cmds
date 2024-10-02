use bevy::prelude::*;
use bevy_cmd_extensions::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_entities)
        .run();
}

fn spawn_entities(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {

    cmds.spawn_2d_camera(Args::default());

    cmds.spawn_pbr_square(Args{
        mesh: Some(GetHandle::NewFrom(&mut meshes)),
        mat: Some(GetHandle::NewFrom(&mut materials)),
        ..default()
    })
    .add_dynamics(Args::default())
    .add_collider(Args::default());
}