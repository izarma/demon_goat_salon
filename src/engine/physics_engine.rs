use avian2d::PhysicsPlugins;
use bevy::prelude::*;
use bevy_tnua::prelude::TnuaControllerPlugin;
use bevy_tnua_avian2d::TnuaAvian2dPlugin;

pub struct PhysicsEnginePlugin;

impl Plugin for PhysicsEnginePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            PhysicsPlugins::new(FixedPostUpdate),
            TnuaControllerPlugin::new(FixedUpdate),
            TnuaAvian2dPlugin::new(FixedUpdate),
        ));
    }
}
