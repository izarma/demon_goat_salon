use bevy::prelude::*;

pub enum AnimationState {
    Idle,
    Walk(f32),
    Jump,
}