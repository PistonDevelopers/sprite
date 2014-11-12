
#![crate_name = "sprite"]
#![deny(missing_docs)]
#![warn(dead_code)]
#![feature(globs)]

//! A library for sprite hierarchy and scene management

extern crate uuid;
extern crate ai_behavior;
extern crate event;
extern crate graphics;

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
pub use ease::{
    EaseFunction,

    EaseQuadraticIn,
    EaseQuadraticOut,
    EaseQuadraticInOut,

    EaseCubicIn,
    EaseCubicOut,
    EaseCubicInOut,

    EaseQuarticIn,
    EaseQuarticOut,
    EaseQuarticInOut,

    EaseQuinticIn,
    EaseQuinticOut,
    EaseQuinticInOut,

    EaseSineIn,
    EaseSineOut,
    EaseSineInOut,

    EaseCircularIn,
    EaseCircularOut,
    EaseCircularInOut,

    EaseExponentialIn,
    EaseExponentialOut,
    EaseExponentialInOut,

    EaseElasticIn,
    EaseElasticOut,
    EaseElasticInOut,

    EaseBackIn,
    EaseBackOut,
    EaseBackInOut,

    EaseBounceIn,
    EaseBounceOut,
    EaseBounceInOut,
};

pub mod ease;

mod animation;
mod scene;
mod sprite;
