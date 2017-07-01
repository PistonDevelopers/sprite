#![crate_name = "sprite"]
#![deny(missing_docs)]
#![deny(missing_copy_implementations)]

//! A library for sprite hierarchy and scene management

extern crate uuid;
extern crate ai_behavior;
extern crate input;
extern crate graphics;
extern crate interpolation;

pub use animation::{
    Animation,
    AnimationState,
    custom_function
};
pub use animation::Animation::*;
pub use scene::Scene;
pub use sprite::Sprite;
pub use interpolation::EaseFunction;

mod animation;
mod scene;
mod sprite;
