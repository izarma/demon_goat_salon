use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(path = "images/imp/Idle.png")]
    pub imp_idle: Handle<Image>,
    #[asset(path = "images/imp/Run.png")]
    pub imp_walk: Handle<Image>,
    #[asset(path = "images/imp/Jump.png")]
    pub imp_jump: Handle<Image>,
    #[asset(path = "images/goat/goat-base.png")]
    pub goat_base: Handle<Image>,
}
