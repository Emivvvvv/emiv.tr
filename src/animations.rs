use crate::ui::BG_COLOR;
use tachyonfx::{fx, Effect, EffectTimer, Interpolation, Motion};

pub const CREATE_CONTENT_EFFECT: fn() -> Effect =
    || fx::sequence(&[fx::coalesce((500, Interpolation::SineOut))]);

pub const CREATE_BANNER_EFFECT: fn() -> Effect = || {
    fx::sequence(&[
        // first we "sweep in" the text from the left, before reversing the effect
        fx::ping_pong(fx::sweep_in(
            Motion::LeftToRight,
            10,
            0,
            BG_COLOR,
            EffectTimer::from_ms(750, Interpolation::QuadIn),
        )),
        // then we coalesce the text back to its original state
        fx::coalesce((500, Interpolation::SineOut)),
    ])
};
