use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(path = "images/werewolf.png")]
    pub werewolf: Handle<Image>,
    #[asset(path = "images/couch.png")]
    pub couch: Handle<Image>,
}
