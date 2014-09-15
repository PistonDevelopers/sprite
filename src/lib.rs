
#![crate_name = "sprite"]
#![deny(missing_doc)]
#![warn(dead_code)]
#![feature(globs)]

//! A library for sprite hierarchy and scene management

extern crate uuid;
extern crate event;
extern crate graphics;

pub use action::{
    Action,
    ActionState,
};
pub use scene::Scene;
pub use sprite::Sprite;

/// Define several actions
pub mod action;
mod scene;
mod sprite;

