#![crate_name = "sprite"]
#![deny(missing_docs)]
#![deny(missing_copy_implementations)]

//! A library for sprite hierarchy and scene management

extern crate uuid;
extern crate ai_behavior;
extern crate event;
extern crate graphics;
extern crate interpolation;

pub use animation::{
    Animation,
    AnimationState,

    MoveTo,
    MoveBy,
    RotateTo,
    RotateBy,
    ScaleTo,
    ScaleBy,

    FlipX,
    FlipY,

    Show,
    Hide,
    ToggleVisibility,
    Blink,
    FadeIn,
    FadeOut,
    FadeTo,

    Ease,
};
pub use scene::Scene;
pub use sprite::Sprite;
pub use interpolation::EaseFunction;

mod animation;
mod scene;
mod sprite;
