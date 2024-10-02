# bevy_cmd_extensions

# TODO
 - use BON builder macro

 ```rust
use bon::bon;

#[bon]
impl CmdExtensions for Commands<'_, '_> {
    #[builder]
    #[builder(finish_fn = and_spawn_it)]
    fn build_2d_camera(
        &mut self,

        #[builder(into, default = [0.0, 0.0])] 
        with_pos: &[f32, 2],

        #[builder(default)] 
        with_rotation: f32

    ) -> EntityCommands {
        ...
    }
}

//use 

//with defaults
cmds.build_2d_camera()
.and_spawn_it();

//with custom values
cmds.build_2d_camera()
.with_pos([0.0, 0.0])
.with_rotation(0.0)
.and_spawn_it();

//build a square(uses a custom start_fn)
cmds.build_sprite(Sprite::default())
.with_scale(10)
.and_spawn_it()

//build a square with physics
cmds.build_sprite(Sprite::default())
.with_scale(10)
.and_spawn_it()
.then_build_dynamics()
.and_insert_them()
```