
use std::f64::consts::{
    PI,
    PI_2,
};

#[allow(missing_doc)]
#[deriving(Clone)]
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
    pub fn calc(self, mut p: f64) -> f64 {
        if p > 1.0 {
            p = 1.0;
        } else if p < 0.0 {
            p = 0.0;
        }

        match self {
            EaseQuadraticIn => {
                p * p
            },
            EaseQuadraticOut => {
                -(p * (p - 2.0))
            },
            EaseQuadraticInOut => {
                if p < 0.5 {
                    2.0 * p * p
                } else {
                    (-2.0 * p * p) + (4.0 * p) - 1.0
                }
            },

            EaseCubicIn => {
                p * p * p
            },
            EaseCubicOut => {
                let f = p - 1.0;
                f * f * f + 1.0
            },
            EaseCubicInOut => {
                if p < 0.5 {
                    4.0 * p * p * p
                } else {
                    let f = (2.0 * p) - 2.0;
                    0.5 * f * f * f + 1.0
                }
            },

            EaseQuarticIn => {
                p * p * p * p
            },
            EaseQuarticOut => {
                let f = p - 1.0;
                f * f * f * (1.0 - p) + 1.0
            },
            EaseQuarticInOut => {
                if p < 0.5 {
                    8.0 * p * p * p * p
                } else {
                    let f = p - 1.0;
                    -8.0 * f * f * f * f + 1.0
                }
            },

            EaseQuinticIn => {
                p * p * p * p *p
            },
            EaseQuinticOut => {
                let f = p - 1.0;
                f * f * f * f * f + 1.0
            },
            EaseQuinticInOut => {
                if p < 0.5  {
                    16.0 * p * p * p * p * p
                } else {
                    let f = (2.0 * p) - 2.0;
                    0.5 * f * f * f * f * f + 1.0
                }
            },

            EaseSineIn => {
                ((p - 1.0) * PI_2).sin() + 1.0
            },
            EaseSineOut => {
                (p * PI_2).sin()
            },
            EaseSineInOut => {
                0.5 * (1.0 - (p * PI).cos())
            },

            EaseCircularIn => {
                1.0 - (1.0 - (p * p)).sqrt()
            },
            EaseCircularOut => {
                ((2.0 - p) * p).sqrt()
            },
            EaseCircularInOut => {
                if p < 0.5 {
                    0.5 * (1.0 - (1.0 - 4.0 * (p * p)).sqrt())
                } else {
                    0.5 * ((-((2.0 * p) - 3.0) * ((2.0 * p) - 1.0)).sqrt() + 1.0)
                }
            },

            EaseExponentialIn => {
                if p == 0.0 {
                    p
                } else {
                    2.0f64.powf(10.0 * (p - 1.0))
                }
            },
            EaseExponentialOut => {
                if p == 1.0 {
                    p
                } else {
                    1.0 - 2.0f64.powf(-10.0 * p)
                }
            },
            EaseExponentialInOut => {
                if p == 0.0 || p == 1.0 {
                    return p;
                }

                if p < 0.5  {
                    0.5 * 2.0f64.powf((20.0 * p) - 10.0)
                } else {
                    -0.5 * 2.0f64.powf((-20.0 * p) + 10.0) + 1.0
                }
            },

            EaseElasticIn => {
                (13.0 * PI_2 * p).sin() * 2.0f64.powf(10.0 * (p - 1.0))
            },
            EaseElasticOut => {
                (-13.0 * PI_2 * (p + 1.0)).sin() * 2.0f64.powf(-10.0 * p) + 1.0
            },
            EaseElasticInOut => {
                if p < 0.5 {
                    0.5 * (13.0 * PI_2 * (2.0 * p)).sin() * 2.0f64.powf(10.0 * ((2.0 * p) - 1.0))
                } else {
                    0.5 * ((-13.0 * PI_2 * ((2.0 * p - 1.0) + 1.0)).sin() * 2.0f64.powf(-10.0 * (2.0 * p - 1.0)) + 2.0)
                }
            },

            EaseBackIn => {
                p * p * p - p * (p * PI).sin()
            },
            EaseBackOut => {
                let f = 1.0 - p;
                1.0 - (f * f * f - f * (f * PI).sin())
            },
            EaseBackInOut => {
                if p < 0.5 {
                    let f = 2.0 * p;
                    0.5 * (f * f * f - f * (f * PI).sin())
                } else {
                    let f = 1.0 - (2.0 * p - 1.0);
                    0.5 * (1.0 - (f * f * f - f * (f * PI).sin())) + 0.5
                }
            },

            EaseBounceIn => {
                1.0 - EaseBounceOut.calc(1.0 - p)
            },
            EaseBounceOut => {
                if p < 4.0 / 11.0 {
                    (121.0 * p * p) / 16.0
                } else if p < 8.0 / 11.0 {
                    (363.0 / 40.0 * p * p) - (99.0 / 10.0 * p) + 17.0 / 5.0
                } else if p < 9.0 / 10.0 {
                    (4356.0 / 361.0 * p * p) - (35442.0 / 1805.0 * p) + 16061.0 / 1805.0
                } else {
                    (54.0 / 5.0 * p * p) - (513.0 / 25.0 * p) + 268.0 / 25.0
                }
            },
            EaseBounceInOut => {
                if p < 0.5 {
                    0.5 * EaseBounceIn.calc(p * 2.0)
                } else {
                    0.5 * EaseBounceOut.calc(p * 2.0 - 1.0) + 0.5
                }
            },
        }
    }
}

