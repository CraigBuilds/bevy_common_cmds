use bevy::prelude::*;

pub struct Args {
    pub pos: [f32; 3],
    pub rotation: f32, //radians
    pub scale: f32,
    pub color: Color
}

impl Default for Args {
    fn default() -> Self {
        Args {
            pos: [0.0, 0.0, 0.0],
            rotation: 0.0,
            scale: 10.0,
            color: Color::WHITE
        }
    }
}

impl CommonCommandsExtensionTrait for Commands<'_, '_> {

    fn spawn_2d_camera(&mut self, args: Args) -> Entity {
        self.spawn(
            Camera2dBundle{
                transform: Transform {
                    translation: Vec3::new(args.pos[0], args.pos[1], 1.0),
                    rotation: Quat::from_rotation_z(args.rotation),
                    ..Default::default()
                },
                ..Default::default()
            }
        ).id()
    }

    fn spawn_square(&mut self, args: Args) -> Entity {
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
        ).id()
    }

}

pub trait CommonCommandsExtensionTrait {
    fn spawn_2d_camera(&mut self, args: Args) -> Entity;
    fn spawn_square(&mut self, args: Args) -> Entity;
}

pub trait MoreColors {
    const RED: Color = Color::srgb(1.0, 0.0, 0.0);
}

impl MoreColors for Color {}