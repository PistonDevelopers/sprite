
//! A module contains implementation of ease functions.

use std::f64::consts::{
    PI,
    PI_2,
};
use std::num::{
    Float,
    FloatMath,
};

pub use ease::EaseFunction::{
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

#[allow(missing_docs)]
#[deriving(Copy, Clone, PartialEq)]
pub enum EaseFunction {
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
}

impl EaseFunction {
    /// Calculate the eased value, normalized
    pub fn calc(self, p: f64) -> f64 {
        match self {
            EaseQuadraticIn => quadratic_in(p),
            EaseQuadraticOut => quadratic_out(p),
            EaseQuadraticInOut => quadratic_in_out(p),

            EaseCubicIn => cubic_in(p),
            EaseCubicOut => cubic_out(p),
            EaseCubicInOut => cubic_in_out(p),

            EaseQuarticIn => quartic_in(p),
            EaseQuarticOut => quartic_out(p),
            EaseQuarticInOut => quartic_in_out(p),

            EaseQuinticIn => quintic_in(p),
            EaseQuinticOut => quintic_out(p),
            EaseQuinticInOut => quintic_in_out(p),

            EaseSineIn => sine_in(p),
            EaseSineOut => sine_out(p),
            EaseSineInOut => sine_in_out(p),

            EaseCircularIn => circular_in(p),
            EaseCircularOut => circular_out(p),
            EaseCircularInOut => circular_in_out(p),

            EaseExponentialIn => exponential_in(p),
            EaseExponentialOut => exponential_out(p),
            EaseExponentialInOut => exponential_in_out(p),

            EaseElasticIn => elastic_in(p),
            EaseElasticOut => elastic_out(p),
            EaseElasticInOut => elastic_in_out(p),

            EaseBackIn => back_in(p),
            EaseBackOut => back_out(p),
            EaseBackInOut => back_in_out(p),

            EaseBounceIn => bounce_in(p),
            EaseBounceOut => bounce_out(p),
            EaseBounceInOut => bounce_in_out(p),
        }
    }
}


/// Applies EaseQuadraticIn function to the input value.
/// Value below 0.0 is interpreted as 0.0, and value above 1.0 is interpreted as 1.0.
pub fn quadratic_in(mut p: f64) -> f64 {
    p = normalized(p);
    p * p
}

/// Applies EaseQuadraticOut function to the input value.
/// Value below 0.0 is interpreted as 0.0, and value above 1.0 is interpreted as 1.0.
pub fn quadratic_out(mut p: f64) -> f64 {
    p = normalized(p);
    -(p * (p - 2.0))
}

/// Applies EaseQuadraticInOut function to the input value.
/// Value below 0.0 is interpreted as 0.0, and value above 1.0 is interpreted as 1.0.
pub fn quadratic_in_out(mut p: f64) -> f64 {
    p = normalized(p);
    if p < 0.5 {
        2.0 * p * p
    } else {
        (-2.0 * p * p) + (4.0 * p) - 1.0
    }
}


/// Applies EaseCubicIn function to the input value.
/// Value below 0.0 is interpreted as 0.0, and value above 1.0 is interpreted as 1.0.
pub fn cubic_in(mut p: f64) -> f64 {
    p = normalized(p);
    p * p * p
}

/// Applies EaseCubicOut function to the input value.
/// Value below 0.0 is interpreted as 0.0, and value above 1.0 is interpreted as 1.0.
pub fn cubic_out(mut p: f64) -> f64 {
    p = normalized(p);
    let f = p - 1.0;
    f * f * f + 1.0
}

/// Applies EaseCubicInOut function to the input value.
/// Value below 0.0 is interpreted as 0.0, and value above 1.0 is interpreted as 1.0.
pub fn cubic_in_out(mut p: f64) -> f64 {
    p = normalized(p);
    if p < 0.5 {
        4.0 * p * p * p
    } else {
        let f = (2.0 * p) - 2.0;
        0.5 * f * f * f + 1.0
    }
}


/// Applies EaseQuarticIn function to the input value.
/// Value below 0.0 is interpreted as 0.0, and value above 1.0 is interpreted as 1.0.
pub fn quartic_in(mut p: f64) -> f64 {
    p = normalized(p);
    p * p * p * p
}

/// Applies EaseQuarticOut function to the input value.
/// Value below 0.0 is interpreted as 0.0, and value above 1.0 is interpreted as 1.0.
pub fn quartic_out(mut p: f64) -> f64 {
    p = normalized(p);
    let f = p - 1.0;
    f * f * f * (1.0 - p) + 1.0
}

/// Applies EaseQuarticInOut function to the input value.
/// Value below 0.0 is interpreted as 0.0, and value above 1.0 is interpreted as 1.0.
pub fn quartic_in_out(mut p: f64) -> f64 {
    p = normalized(p);
    if p < 0.5 {
        8.0 * p * p * p * p
    } else {
        let f = p - 1.0;
        -8.0 * f * f * f * f + 1.0
    }
}


/// Applies EaseQuinticIn function to the input value.
/// Value below 0.0 is interpreted as 0.0, and value above 1.0 is interpreted as 1.0.
pub fn quintic_in(mut p: f64) -> f64 {
    p = normalized(p);
    p * p * p * p * p
}

/// Applies EaseQuinticOut function to the input value.
/// Value below 0.0 is interpreted as 0.0, and value above 1.0 is interpreted as 1.0.
pub fn quintic_out(mut p: f64) -> f64 {
    p = normalized(p);
    let f = p - 1.0;
    f * f * f * f * f + 1.0
}

/// Applies EaseQuinticInOut function to the input value.
/// Value below 0.0 is interpreted as 0.0, and value above 1.0 is interpreted as 1.0.
pub fn quintic_in_out(mut p: f64) -> f64 {
    p = normalized(p);
    if p < 0.5  {
        16.0 * p * p * p * p * p
    } else {
        let f = (2.0 * p) - 2.0;
        0.5 * f * f * f * f * f + 1.0
    }
}


/// Applies EaseSineIn function to the input value.
/// Value below 0.0 is interpreted as 0.0, and value above 1.0 is interpreted as 1.0.
pub fn sine_in(mut p: f64) -> f64 {
    p = normalized(p);
    ((p - 1.0) * PI_2).sin() + 1.0
}

/// Applies EaseSineOut function to the input value.
/// Value below 0.0 is interpreted as 0.0, and value above 1.0 is interpreted as 1.0.
pub fn sine_out(mut p: f64) -> f64 {
    p = normalized(p);
    (p * PI_2).sin()
}

/// Applies EaseSineInOut function to the input value.
/// Value below 0.0 is interpreted as 0.0, and value above 1.0 is interpreted as 1.0.
pub fn sine_in_out(mut p: f64) -> f64 {
    p = normalized(p);
    0.5 * (1.0 - (p * PI).cos())
}


/// Applies EaseCircularIn function to the input value.
/// Value below 0.0 is interpreted as 0.0, and value above 1.0 is interpreted as 1.0.
pub fn circular_in(mut p: f64) -> f64 {
    p = normalized(p);
    1.0 - (1.0 - (p * p)).sqrt()
}

/// Applies EaseCircularOut function to the input value.
/// Value below 0.0 is interpreted as 0.0, and value above 1.0 is interpreted as 1.0.
pub fn circular_out(mut p: f64) -> f64 {
    p = normalized(p);
    ((2.0 - p) * p).sqrt()
}

/// Applies EaseCircularInOut function to the input value.
/// Value below 0.0 is interpreted as 0.0, and value above 1.0 is interpreted as 1.0.
pub fn circular_in_out(mut p: f64) -> f64 {
    p = normalized(p);
    if p < 0.5 {
        0.5 * (1.0 - (1.0 - 4.0 * (p * p)).sqrt())
    } else {
        0.5 * ((-((2.0 * p) - 3.0) * ((2.0 * p) - 1.0)).sqrt() + 1.0)
    }
}


/// Applies EaseExponentialIn function to the input value.
/// Value below 0.0 is interpreted as 0.0, and value above 1.0 is interpreted as 1.0.
pub fn exponential_in(mut p: f64) -> f64 {
    p = normalized(p);
    if p == 0.0 {
        p
    } else {
        2.0f64.powf(10.0 * (p - 1.0))
    }
}

/// Applies EaseExponentialOut function to the input value.
/// Value below 0.0 is interpreted as 0.0, and value above 1.0 is interpreted as 1.0.
pub fn exponential_out(mut p: f64) -> f64 {
    p = normalized(p);
    if p == 1.0 {
        p
    } else {
        1.0 - 2.0f64.powf(-10.0 * p)
    }
}

/// Applies EaseExponentialInOut function to the input value.
/// Value below 0.0 is interpreted as 0.0, and value above 1.0 is interpreted as 1.0.
pub fn exponential_in_out(mut p: f64) -> f64 {
    p = normalized(p);
    if p == 0.0 || p == 1.0 {
        return p;
    }

    if p < 0.5  {
        0.5 * 2.0f64.powf((20.0 * p) - 10.0)
    } else {
        -0.5 * 2.0f64.powf((-20.0 * p) + 10.0) + 1.0
    }
}


/// Applies EaseElasticIn function to the input value.
/// Value below 0.0 is interpreted as 0.0, and value above 1.0 is interpreted as 1.0.
pub fn elastic_in(mut p: f64) -> f64 {
    p = normalized(p);
    (13.0 * PI_2 * p).sin() * 2.0f64.powf(10.0 * (p - 1.0))
}

/// Applies EaseElasticOut function to the input value.
/// Value below 0.0 is interpreted as 0.0, and value above 1.0 is interpreted as 1.0.
pub fn elastic_out(mut p: f64) -> f64 {
    p = normalized(p);
    (-13.0 * PI_2 * (p + 1.0)).sin() * 2.0f64.powf(-10.0 * p) + 1.0
}

/// Applies EaseElasticInOut function to the input value.
/// Value below 0.0 is interpreted as 0.0, and value above 1.0 is interpreted as 1.0.
pub fn elastic_in_out(mut p: f64) -> f64 {
    p = normalized(p);
    if p < 0.5 {
        0.5 * (13.0 * PI_2 * (2.0 * p)).sin() * 2.0f64.powf(10.0 * ((2.0 * p) - 1.0))
    } else {
        0.5 * ((-13.0 * PI_2 * ((2.0 * p - 1.0) + 1.0)).sin() * 2.0f64.powf(-10.0 * (2.0 * p - 1.0)) + 2.0)
    }
}


/// Applies EaseBackIn function to the input value.
/// Value below 0.0 is interpreted as 0.0, and value above 1.0 is interpreted as 1.0.
pub fn back_in(mut p: f64) -> f64 {
    p = normalized(p);
    p * p * p - p * (p * PI).sin()
}

/// Applies EaseBackOut function to the input value.
/// Value below 0.0 is interpreted as 0.0, and value above 1.0 is interpreted as 1.0.
pub fn back_out(mut p: f64) -> f64 {
    p = normalized(p);
    let f = 1.0 - p;
    1.0 - (f * f * f - f * (f * PI).sin())
}

/// Applies EaseBackInOut function to the input value.
/// Value below 0.0 is interpreted as 0.0, and value above 1.0 is interpreted as 1.0.
pub fn back_in_out(mut p: f64) -> f64 {
    p = normalized(p);
    if p < 0.5 {
        let f = 2.0 * p;
        0.5 * (f * f * f - f * (f * PI).sin())
    } else {
        let f = 1.0 - (2.0 * p - 1.0);
        0.5 * (1.0 - (f * f * f - f * (f * PI).sin())) + 0.5
    }
}


/// Applies EaseBounceIn function to the input value.
/// Value below 0.0 is interpreted as 0.0, and value above 1.0 is interpreted as 1.0.
pub fn bounce_in(mut p: f64) -> f64 {
    p = normalized(p);
    1.0 - bounce_out(1.0 - p)
}

/// Applies EaseBounceOut function to the input value.
/// Value below 0.0 is interpreted as 0.0, and value above 1.0 is interpreted as 1.0.
pub fn bounce_out(mut p: f64) -> f64 {
    p = normalized(p);
    if p < 4.0 / 11.0 {
        (121.0 * p * p) / 16.0
    } else if p < 8.0 / 11.0 {
        (363.0 / 40.0 * p * p) - (99.0 / 10.0 * p) + 17.0 / 5.0
    } else if p < 9.0 / 10.0 {
        (4356.0 / 361.0 * p * p) - (35442.0 / 1805.0 * p) + 16061.0 / 1805.0
    } else {
        (54.0 / 5.0 * p * p) - (513.0 / 25.0 * p) + 268.0 / 25.0
    }
}

/// Applies EaseBounceInOut function to the input value.
/// Value below 0.0 is interpreted as 0.0, and value above 1.0 is interpreted as 1.0.
pub fn bounce_in_out(mut p: f64) -> f64 {
    p = normalized(p);
    if p < 0.5 {
        0.5 * bounce_in(p * 2.0)
    } else {
        0.5 * bounce_out(p * 2.0 - 1.0) + 0.5
    }
}

fn normalized(p: f64) -> f64 {
    if p > 1.0 {
        1.0
    } else if p < 0.0 {
        0.0
    } else {
        p
    }
}
