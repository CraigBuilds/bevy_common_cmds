# bevy_cmd_extensions

# TODO
 - use BON builder macro

 ```rust
use bon::bon;

#[bon]
impl CmdExtensions for Commands<'_, '_> {
    #[builder]
    #[builder(finish_fn = spawn)]
    fn 2d_camera(
        &mut self,

        #[builder(into, default = [0.0, 0.0])] 
        pos: &[f32, 2],

        #[builder(default)] 
        rotation: f32

    ) -> EntityCommands {
        ...
    }
}

//use 

//with defaults
cmds.2d_camera().spawn();
//with custom values
cmds.2d_camera().pos([0.0, 0.0]).rotation(0.0).spawn();
```