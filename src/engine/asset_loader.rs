use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_seedling::sample::Sample;

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(path = "images/imp/Imp-IDLE-Sprite-sheet.png")]
    pub imp_idle: Handle<Image>,
    // #[asset(path = "images/imp/Run.png")]
    // pub imp_walk: Handle<Image>,
    // #[asset(path = "images/imp/Jump.png")]
    // pub imp_jump: Handle<Image>,
    #[asset(path = "images/goat/goat-base.png")]
    pub goat_base: Handle<Image>,
    #[asset(path = "images/goat/goat-jaw.png")]
    pub goat_jaw: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio/salon-background.ogg")]
    pub background: Handle<Sample>,
}