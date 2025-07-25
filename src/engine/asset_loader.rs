use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_seedling::sample::Sample;

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(path = "images/imp/Imp-IDLE-Sprite-sheet.png")]
    pub imp_idle: Handle<Image>,
    // #[asset(path = "images/imp/Imp-WALK-Sprite-sheet.png")]
    // pub imp_walk: Handle<Image>,
    // #[asset(path = "images/imp/Imp-JUMP-Sprite-sheet.png")]
    // pub imp_jump: Handle<Image>,
    #[asset(path = "images/goat/goat-base.png")]
    pub goat_base: Handle<Image>,
    #[asset(path = "images/goat/goat-jaw.png")]
    pub goat_jaw: Handle<Image>,
    #[asset(path = "images/goat/goat-ears.png")]
    pub goat_ears: Handle<Image>,
    #[asset(path = "images/goat/goat-hair-A1-left.png")]
    pub goat_hair_a1_left: Handle<Image>,
    #[asset(path = "images/goat/goat-hair-A1-right.png")]
    pub goat_hair_a1_right: Handle<Image>,
    #[asset(path = "images/goat/goat-hair-A2-left.png")]
    pub goat_hair_a2_left: Handle<Image>,
    #[asset(path = "images/goat/goat-hair-A2-right.png")]
    pub goat_hair_a2_right: Handle<Image>,
    #[asset(path = "ui/game_over.png")]
    pub game_over_text: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio/salon-background.ogg")]
    pub background: Handle<Sample>,
}
