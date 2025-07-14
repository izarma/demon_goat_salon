use bevy::prelude::*;

pub struct ImpPlugin;

impl Plugin for ImpPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(());
    }
}
