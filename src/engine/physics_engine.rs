use avian2d::{prelude::*, PhysicsPlugins};
use bevy::prelude::*;
use bevy_tnua::{builtins::TnuaBuiltinClimb, control_helpers::TnuaCrouchEnforcerPlugin, prelude::{TnuaBuiltinJump, TnuaBuiltinWalk, TnuaController, TnuaControllerPlugin}, TnuaAction, TnuaObstacleRadar, TnuaUserControlsSystemSet};
use bevy_tnua_avian2d::TnuaAvian2dPlugin;

use crate::{
    animation::sprite_animation::SpriteAnimState,
    engine::{GameState, asset_loader::ImageAssets},
};

pub struct PhysicsEnginePlugin;

impl Plugin for PhysicsEnginePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            PhysicsPlugins::new(FixedPostUpdate),
            TnuaControllerPlugin::new(FixedUpdate),
            TnuaAvian2dPlugin::new(FixedUpdate),
            TnuaCrouchEnforcerPlugin::new(FixedUpdate),
        ));
    }
}

