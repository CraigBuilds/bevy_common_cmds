use bevy::prelude::*;
use bevy_common_cmds::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_entities)
        .run();
}

fn spawn_entities(mut cmds: Commands) {

    cmds.spawn_2d_camera(Args::default());

    cmds.spawn_square(Args::default());

    cmds.spawn_square(Args{
        pos: [20.0, 20.0, 0.0],
        ..default()
    });

    cmds.spawn_square(Args{
        pos: [-20.0, -20.0, 0.0],
        color: Color::RED,
        ..default()
    });

}