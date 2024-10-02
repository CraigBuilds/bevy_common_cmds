use bevy::prelude::*;
use bevy::ecs::system::EntityCommands;
use bevy::color::palettes::css as css;
use std::collections::HashMap;
pub trait CmdExtensions {
    fn spawn_2d_camera(&mut self, args: Args) -> EntityCommands;
    fn spawn_square(&mut self, args: Args) -> EntityCommands;
    fn spawn_square_spite(&mut self, args: Args) -> EntityCommands;
}

impl CmdExtensions for Commands<'_, '_> {

    fn spawn_2d_camera(&mut self, args: Args) -> EntityCommands {
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

    fn spawn_square(&mut self, args: Args) -> EntityCommands {
        //get or insert the handle from/into the cache if given, otherwise create a new one and drop the handle.
        let handle = if let  Some(handles_cache) = args.handles_cache {
            if let Some(handle) = handles_cache.get("string") {
                handle.clone()
            }
            else {
                let asset_container: &mut Assets<Mesh> = args.mesh_assets.unwrap();
                let handle = asset_container.add(Mesh::from(Rectangle::new(50.0, 100.0)));
                handles_cache.insert("square".to_string(), handle.clone());
                handle
            }
        }
        else {
            let asset_container: &mut Assets<Mesh> = args.mesh_assets.unwrap();
            let handle = asset_container.add(Mesh::from(Rectangle::new(50.0, 100.0)));
            handle
        };

        self.spawn(
            PbrBundle {
                mesh: handle,
                transform: Transform {
                    translation: Vec3::new(args.pos[0], args.pos[1], args.pos[2]),
                    rotation: Quat::from_rotation_z(args.rotation),
                    //remove this?
                    scale: Vec3::new(args.scale, args.scale, args.scale),
                    ..Default::default()
                },
                ..Default::default()
            }
        )
    }

    fn spawn_square_spite(&mut self, args: Args) -> EntityCommands {
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

pub struct Args<'a> {
    pub pos: [f32; 3],
    //radians
    pub rotation: f32,
    pub scale: f32,
    pub color: Color,
    //this only needs to be passed in the first time you want to make the asset. Subsequent versions will use the cached handles.
    pub mesh_assets: Option<&'a mut Assets<Mesh>>,
    pub handles_cache: Option<&'a mut HashMap<String, Handle<Mesh>>>
}

impl<'a> Default for Args<'a> {
    fn default() -> Self {
        Args {
            pos: [0.0, 0.0, 0.0],
            rotation: 0.0,
            scale: 10.0,
            color: Color::WHITE,
            mesh_assets: None,
            handles_cache: None
        }
    }
}

pub trait MoreColors {
    //Extended colors from the CSS4 specification, Also known as X11 colors, which were standardized in HTML 4.0.
    const RED: Color = Color::Srgba(css::RED);
    const GREEN: Color = Color::Srgba(css::GREEN);
    const BLUE: Color = Color::Srgba(css::BLUE);
    const WHITE: Color = Color::Srgba(css::WHITE);
    const BLACK: Color = Color::Srgba(css::BLACK);
    const YELLOW: Color = Color::Srgba(css::YELLOW);
    const AQUA: Color = Color::Srgba(css::AQUA);
    const FUCHSIA: Color = Color::Srgba(css::FUCHSIA);
    const GRAY: Color = Color::Srgba(css::GRAY);
    const LIME: Color = Color::Srgba(css::LIME);
    const MAROON: Color = Color::Srgba(css::MAROON);
    const NAVY: Color = Color::Srgba(css::NAVY);
    const OLIVE: Color = Color::Srgba(css::OLIVE);
    const PURPLE: Color = Color::Srgba(css::PURPLE);
    const SILVER: Color = Color::Srgba(css::SILVER);
    const TEAL: Color = Color::Srgba(css::TEAL);
    const ORANGE: Color = Color::Srgba(css::ORANGE);
    const BROWN: Color = Color::Srgba(css::BROWN);
    const CRIMSON: Color = Color::Srgba(css::CRIMSON);
    const DARK_BLUE: Color = Color::Srgba(css::DARK_BLUE);
    const DARK_GREEN: Color = Color::Srgba(css::DARK_GREEN);
    const DARK_RED: Color = Color::Srgba(css::DARK_RED);
}

impl MoreColors for Color {}