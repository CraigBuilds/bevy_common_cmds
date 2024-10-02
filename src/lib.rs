use bevy::prelude::*;
use bevy::ecs::system::EntityCommands;
use bevy::color::palettes::css as css;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
#[allow(unused_imports)]
use bevy_collider_gen::rapier2d::single_convex_hull_collider_translated;
#[allow(unused_imports)]
use bevy_collider_gen::Edges;

pub trait CmdExtensions {
    fn spawn_2d_camera(&mut self, args: Args) -> EntityCommands;
    fn spawn_pbr_square(&mut self, args: Args) -> EntityCommands;
    fn spawn_square_spite(&mut self, args: Args) -> EntityCommands;
}
pub trait EntityCmdExtensions {
    fn add_collider(&mut self, args: Args) -> &mut Self;
    fn add_dynamics(&mut self, args: Args) -> &mut Self;
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

    //TODO split into 2 functions. Spawn pbr shape, and spawn pbr square.
    //square cannot reuse handle, as it might be given something that isn't a square. 
    //maybe don;t even have the square function, just have a pbr shape function that takes a shape.
    fn spawn_pbr_square(&mut self, args: Args) -> EntityCommands {
        
        let mesh_handle = match args.mesh {
            Some(GetHandle::Reuse(h)) => {h.clone()},
            Some(GetHandle::NewFrom(a)) => {a.add(Rectangle::new(10.0, 10.0))}
            _=> panic!("requires a mesh")
        };
        let mat_handle = match args.mat {
            Some(GetHandle::Reuse(h)) => {h.clone()},
            Some(GetHandle::NewFrom(a)) => {a.add(args.color)}
            _=> panic!("requires a mesh")  
        };          
        
        //spawn the entity
        self.spawn(
            //TODO get working with pbr.i.e standard material instead of color material
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(mesh_handle),
                material: mat_handle,
                transform: Transform::from_xyz(50., 0., 0.),
                ..Default::default()
            }
        )
    }

    //maybe turn this into spawn_spite, that accepts a spite.
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

impl<'a> EntityCmdExtensions for EntityCommands<'a> {
    
    /// Create a collider from mesh. Seems to be missing from 2d...
    fn add_collider(&mut self, _args: Args) -> &mut Self {
        // let colliders = multi_convex_polyline_collider_translated(args.sprite_image);
        self
    }

    /// Move under gravity, friction, etc.
    fn add_dynamics(&mut self, _args: Args) -> &mut Self {
        self
    }
}


pub enum GetHandle<'a, T: Asset> {
    Reuse(Handle<T>),
    NewFrom(&'a mut Assets<T>),
    TakeFrom(Handle<T>, &'a mut Assets<T>)
}

pub struct Args<'a> {
    pub pos: [f32; 3],
    pub rotation: f32, //radians
    pub scale: f32,
    pub color: Color,
    pub mesh: Option<GetHandle<'a, Mesh>>,
    pub mat: Option<GetHandle<'a, ColorMaterial>>
}

impl<'a> Default for Args<'a> {
    fn default() -> Self {
        Args {
            pos: [0.0, 0.0, 0.0],
            rotation: 0.0,
            scale: 10.0,
            color: Color::WHITE,
            mesh: None,
            mat: None
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