use bevy::prelude::*;
use bevy_cmd_extensions::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_entities)
        .run();
}

fn spawn_entities(mut cmds: Commands) {

    cmds.spawn_2d_camera(Args::default());

    cmds.spawn_square_spite(Args::default());

    cmds.spawn_square_spite(Args{
        pos: [20.0, 20.0, 0.0],
        ..default()
    });

    cmds.spawn_square_spite(Args{
        pos: [-20.0, -20.0, 0.0],
        color: Color::RED,
        ..default()
    });
}