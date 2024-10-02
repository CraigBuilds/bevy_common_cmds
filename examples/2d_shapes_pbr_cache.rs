use bevy::{prelude::*, utils::HashMap};
use bevy_cmd_extensions::*;

#[derive(Resource, Default)]
struct MeshHandlesCache(HashMap<String, Handle<Mesh>>);

#[derive(Resource, Default)]
struct MaterialsHandlesCache(HashMap<String, Handle<ColorMaterial>>);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<MeshHandlesCache>()
        .init_resource::<MaterialsHandlesCache>()
        .add_systems(Startup, (register_assets,spawn_entities).chain())
        .run();
}
fn register_assets(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut mesh_cache: ResMut<MeshHandlesCache>,
    mut mat_cache: ResMut<MaterialsHandlesCache>
) {
    let h_mesh = meshes.add(Rectangle::new(10.0, 10.0));
    let h_mat = materials.add(Color::RED);
    mesh_cache.0.insert("10x10rect".to_owned(), h_mesh);
    mat_cache.0.insert("red".to_owned(), h_mat);

}

fn spawn_entities(
    mut cmds: Commands,
    mesh_cache: Res<MeshHandlesCache>,
    mat_cache: Res<MaterialsHandlesCache>
) {

    cmds.spawn_2d_camera(Args::default());

    cmds.spawn_square(Args{
        mesh: Some(GetHandle::Reuse(mesh_cache.0.get("10x10rect").unwrap().clone())),
        mat: Some(GetHandle::Reuse(mat_cache.0.get("red").unwrap().clone())),
        ..default()
    });
}