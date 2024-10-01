use bevy::prelude::*;
use std::collections::HashMap;

pub struct Args {
    pub pos: [f32; 3],
    //radians
    pub rotation: f32,
    pub scale: f32,
    pub color: Color,
    //this only needs to be passed in the first time you want to make the asset. Subsequent versions will use the cached handles.
    pub mesh_assets: Option<ResMut<Assets<Mesh>>>,
    //private
    mesh_handles: HashMap<String, Handle<Mesh>>
}

impl Default for Args {
    fn default() -> Self {
        Args {
            pos: [0.0, 0.0, 0.0],
            rotation: 0.0,
            scale: 10.0,
            color: Color::WHITE,
            mesh_assets: None,
            mesh_handles: HashMap::new()
        }
    }
}

impl CommonCommandsExtensionTrait for Commands<'_, '_> {

    fn spawn_2d_camera(&mut self, args: Args) -> &mut EntityCommands {
        self.spawn(
            Camera2dBundle{
                transform: Transform {
                    translation: Vec3::new(args.pos[0], args.pos[1], 1.0),
                    rotation: Quat::from_rotation_z(args.rotation),
                    ..Default::default()
                },
                ..Default::default()
            }
        )
    }

    // fn spawn_square(&mut self, args: Args) -> &mut EntityCommands {
        
    //     //check if it has already been made, if not add it to the mesh_assets resource
    //     let handle = args.mesh_handles.get("square").unwrap_or_else(|| {
    //         let mesh = Mesh::from(shape::Quad {
    //             size: Vec2::new(1.0, 1.0),
    //             flip: false
    //         });
    //         let handle = args.mesh_assets.as_mut().expect("!!!!!!!!!!!!!!!!!!!!!!!!").add(mesh);
    //         args.mesh_handles.insert("square".to_string(), handle.clone());
    //         &handle
    //     });

    //     self.spawn(
    //         PbrBundle {
    //             mesh: handle.clone(),
    //             transform: Transform {
    //                 translation: Vec3::new(args.pos[0], args.pos[1], args.pos[2]),
    //                 rotation: Quat::from_rotation_z(args.rotation),
    //                 //remove this?
    //                 scale: Vec3::new(args.scale, args.scale, args.scale),
    //                 ..Default::default()
    //             },
    //             ..Default::default()
    //         }
    //     )
    // }

    fn spawn_square_spite(&mut self, args: Args) -> &mut EntityCommands {
        self.spawn(
            SpriteBundle {
                sprite: Sprite{
                    color: args.color,
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(args.pos[0], args.pos[1], 0.0),
                    scale: Vec3::new(args.scale, args.scale, 1.0),
                    ..Default::default()
                },
                ..Default::default()
            }
        )
    }

}

pub trait CommonCommandsExtensionTrait {
    fn spawn_2d_camera(&mut self, args: Args) -> &mut EntityCommands;
    // fn spawn_square(&mut self, args: Args) -> &mut EntityCommands;
    fn spawn_square_spite(&mut self, args: Args) -> &mut EntityCommands;
}

pub trait MoreColors {
    const RED: Color = Color::srgb(1.0, 0.0, 0.0);
}

impl MoreColors for Color {}