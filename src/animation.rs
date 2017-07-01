use std::rc::Rc;
use graphics::ImageSize;
use graphics::math::Scalar;

use ai_behavior::{
    Status,
    Success,
    Running,
};

use interpolation::EaseFunction;
use sprite::Sprite;

pub struct FuncImpl<F : Fn()>{
    f: F
}


/// creates a custom function to be triggered 
/// during animations 
/// This function takes care of the boilerplate required to fullfill 
/// trait requirements.
pub fn custom_function<F : Fn() + 'static>(f: F) -> Rc<Box<Func>>
{
    Rc::new(
        Box::new(
            FuncImpl{
                f: f,
            }
        )
    )
}

/// trait to allow multiple closures to be called from the same 
/// Function Animation State
pub trait Func{
    fn execute(&self);
}

impl<F> Func for FuncImpl<F>
where F : Fn()
{
    fn execute(&self){
        (self.f)()
    }
}

impl PartialEq for Func{
    fn eq(&self, other: &Func) -> bool{
        return self as *const _ ==  other as *const _;
    }
}

/// Animations supported by Sprite
#[derive(Clone, PartialEq)]
pub enum Animation {
    /// duration, x, y
    ///
    /// Move sprite to specified position
    MoveTo(f64, Scalar, Scalar),
    /// duration, x, y
    ///
    /// Move sprite to specified position, relatively
    MoveBy(f64, Scalar, Scalar),
    /// duration, deg
    ///
    /// Rotate sprite to specified degree
    RotateTo(f64, Scalar),
    /// duration, deg
    ///
    /// Rotate sprite to specified degree, relatively
    RotateBy(f64, Scalar),
    /// duration, sx, sy
    ///
    /// Scale sprite to specified scale
    ScaleTo(f64, Scalar, Scalar),
    /// duration, sx, sy
    ///
    /// Scale sprite to specified scale, relatively
    ScaleBy(f64, Scalar, Scalar),
    /// Flip sprite in x direction
    FlipX(bool),
    /// Flip sprite in y direction
    FlipY(bool),
    /// Set the sprite's visibility to true
    Show,
    /// Set the sprite's visibility to false
    Hide,
    /// Toggle the sprite's visibility
    ToggleVisibility,
    /// duration, times
    Blink(f64, usize),
    /// duration
    ///
    /// Fade in the sprite, set its opacity from 0 to 1 in `dt` seconds
    FadeIn(f64),
    /// duration
    ///
    /// Fade out the sprite, set its opacity from 1 to 0 in `dt` seconds
    FadeOut(f64),
    /// duration, opacity
    ///
    /// Set the sprite's opacity to specified value in `dt` seconds
    FadeTo(f64, f64),
    /// ease_function, animation
    ///
    /// Tweening the animation with ease function
    Ease(EaseFunction, Box<Animation>),

    /// custom_function
    /// 
    /// Execute custom code:
    /// The Rc makes the type clonable, 
    /// The box allows us to have different initializations using different
    /// closures. Use the custom_function function to instanciate your callback
    Function(Rc<Box<Func>>),
}

impl Animation {
    /// Generate a new state from Animation with specified Sprite
    pub fn to_state<I: ImageSize>(&self, sprite: &Sprite<I>) -> AnimationState {
        use Animation::*;
        use AnimationState as S;

        match *self {
            MoveTo(dur, dx, dy) => {
                let (bx, by) = sprite.get_position();
                S::Move(0.0, bx, by, dx - bx, dy - by, dur)
            },
            MoveBy(dur, cx, cy) => {
                let (bx, by) = sprite.get_position();
                S::Move(0.0, bx, by, cx, cy, dur)
            },
            RotateTo(dur, d) => {
                let b = sprite.get_rotation();
                S::Rotate(0.0, b, d - b, dur)
            },
            RotateBy(dur, c) => {
                let b = sprite.get_rotation();
                S::Rotate(0.0, b, c, dur)
            },
            ScaleTo(dur, dx, dy) => {
                let (bx, by) = sprite.get_scale();
                S::Scale(0.0, bx, by, dx - bx, dy - by, dur)
            },
            ScaleBy(dur, cx, cy) => {
                let (bx, by) = sprite.get_scale();
                S::Scale(0.0, bx, by, cx, cy, dur)
            },
            FlipX(flip_x) => {
                let flip_y = sprite.get_flip_y();
                S::Flip(flip_x, flip_y)
            },
            FlipY(flip_y) => {
                let flip_x = sprite.get_flip_x();
                S::Flip(flip_x, flip_y)
            },
            Show => {
                S::Visibility(true)
            },
            Hide => {
                S::Visibility(false)
            },
            ToggleVisibility => {
                let visible = sprite.get_visible();
                S::Visibility(!visible)
            },
            Blink(dur, times) => {
                S::Blink(0.0, dur, 0, 2 * times)
            },
            FadeIn(dur) => {
                let b = sprite.get_opacity() as f64;
                S::Fade(0.0, b, 1.0 - b, dur)
            },
            FadeOut(dur) => {
                let b = sprite.get_opacity() as f64;
                S::Fade(0.0, b, 0.0 - b, dur)
            },
            FadeTo(dur, d) => {
                let b = sprite.get_opacity() as f64;
                S::Fade(0.0, b, d - b, dur)
            },
            Ease(f, ref animation) => {
                S::Ease(f, Box::new(animation.to_state(sprite)))
            },
            Function(ref f) => {
                S::Function(f.clone())
            },
        }
    }
}

/// The state of animation
#[derive(Clone)]
pub enum AnimationState {
    /// time, begin_x, begin_y, change_x, change_y, duration
    Move(f64, Scalar, Scalar, Scalar, Scalar, f64),
    /// time, begin, change, duration
    Rotate(f64, Scalar, Scalar, f64),
    /// time, begin_x, begin_y, change_x, change_y, duration
    Scale(f64, Scalar, Scalar, Scalar, Scalar, f64),
    /// flip_x, flip_y
    Flip(bool, bool),
    /// visible
    Visibility(bool),
    /// past_time, duration, blinked_times, total_times
    Blink(f64, f64, usize, usize),
    /// time, begin, change, duration
    Fade(f64, f64, f64, f64),
    /// ease_function, animation
    Ease(EaseFunction, Box<AnimationState>),
    /// Custom function
    Function(Rc<Box<Func>>),
}

impl AnimationState {
    /// Update the state and change the sprite's properties
    pub fn update<I: ImageSize>(
        &self,
        sprite: &mut Sprite<I>,
        dt: f64
    ) -> (Option<AnimationState>, Status, f64) {
        use AnimationState::*;

        match *self {
            Move(t, bx, by, cx, cy, d) => {
                let factor = (t + dt) / d;
                update_position(sprite, factor, t + dt, bx, by, cx, cy, d)
            },
            Rotate(t, b, c, d) => {
                let factor = (t + dt) / d;
                update_rotation(sprite, factor, t + dt, b, c, d)
            },
            Scale(t, bx, by, cx, cy, d) => {
                let factor = (t + dt) / d;
                update_scale(sprite, factor, t + dt, bx, by, cx, cy, d)
            },
            Flip(flip_x, flip_y) => {
                sprite.set_flip_x(flip_x);
                sprite.set_flip_y(flip_y);
                (None, Success, dt)
            },
            Visibility(visible) => {
                sprite.set_visible(visible);
                (None, Success, dt)
            },
            Blink(past, dur, cur, total) => {
                let period = dur / total as f64;
                if past + dt >= (cur + 1) as f64 * period {
                    let visible = sprite.get_visible();
                    sprite.set_visible(!visible);
                    if past + dt >= dur {
                        (None, Success, past + dt - dur)
                    } else {
                        (Some(Blink(past + dt, dur, cur + 1, total)),
                         Running, 0.0)
                    }
                } else {
                    (Some(Blink(past + dt, dur, cur, total)),
                     Running, 0.0)
                }
            },
            Fade(t, b, c, d) => {
                let factor = (t + dt) / d;
                update_opacity(sprite, factor, t + dt, b, c, d)
            },
            Ease(f, ref state) => {
                let mut support_ease = true;
                let (state, status, remain) = match **state {
                    Move(t, bx, by, cx, cy, d) => {
                        let factor = ::interpolation::Ease::calc((t + dt) / d, f);
                        update_position(sprite, factor, t + dt,
                                        bx, by, cx, cy, d)
                    },
                    Rotate(t, b, c, d) => {
                        let factor = ::interpolation::Ease::calc((t + dt) / d, f);
                        update_rotation(sprite, factor, t + dt, b, c, d)
                    },
                    Scale(t, bx, by, cx, cy, d) => {
                        let factor = ::interpolation::Ease::calc((t + dt) / d, f);
                        update_scale(sprite, factor, t + dt, bx, by, cx, cy, d)
                    },
                    Fade(t, b, c, d) => {
                        let factor = ::interpolation::Ease::calc((t + dt) / d, f);
                        update_opacity(sprite, factor, t + dt, b, c, d)
                    },
                    _ => {
                        support_ease = false;
                        state.update(sprite, dt)
                    }
                };

                if !support_ease {
                    return (state, status, remain);
                }

                if let Some(state) = state {
                    (Some(AnimationState::Ease(f, Box::new(state))),
                     status, remain)
                } else {
                    (None, status, remain)
                }
            },
            Function(ref f) => {
                f.execute();
                (None, Success, dt)
            },
        }
    }
}

fn update_position<I: ImageSize>(
    sprite: &mut Sprite<I>,
    factor: f64,
    t: f64,
    bx: f64,
    by: f64,
    cx: f64,
    cy: f64,
    d: f64
) -> (Option<AnimationState>, Status, f64) {
    if t >= d {
        sprite.set_position(bx + cx, by + cy);
        (None, Success, t - d)
    } else {
        sprite.set_position(bx + cx * factor, by + cy * factor);
        (Some(AnimationState::Move(t, bx, by, cx, cy, d)),
         Running, 0.0)
    }
}

fn update_rotation<I: ImageSize>(
    sprite: &mut Sprite<I>,
    factor: f64,
    t: f64,
    b: f64,
    c: f64,
    d: f64
) -> (Option<AnimationState>, Status, f64) {
    if t >= d {
        sprite.set_rotation(b + c);
        (None, Success, t - d)
    } else {
        sprite.set_rotation(b + c * factor);
        (Some(AnimationState::Rotate(t, b, c, d)),
         Running, 0.0)
    }
}

fn update_scale<I: ImageSize>(
    sprite: &mut Sprite<I>,
    factor: f64,
    t: f64,
    bx: f64,
    by: f64,
    cx: f64,
    cy: f64,
    d: f64
) -> (Option<AnimationState>, Status, f64) {
    if t >= d {
        sprite.set_scale(bx + cx, by + cy);
        (None, Success, t - d)
    } else {
        sprite.set_scale(bx + cx * factor, by + cy * factor);
        (Some(AnimationState::Scale(t, bx, by, cx, cy, d)),
         Running, 0.0)
    }
}

fn update_opacity<I: ImageSize>(
    sprite: &mut Sprite<I>,
    factor: f64,
    t: f64,
    b: f64,
    c: f64,
    d: f64
) -> (Option<AnimationState>, Status, f64) {
    if t >= d {
        sprite.set_opacity((b + c) as f32);
        (None, Success, t - d)
    } else {
        sprite.set_opacity((b + c * factor) as f32);
        (Some(AnimationState::Fade(t, b, c, d)),
         Running, 0.0)
    }
}
