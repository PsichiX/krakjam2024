use micro_games_kit::third_party::rand::{thread_rng, Rng};
use std::ops::RangeInclusive;

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Speed {
    pub value: f32,
}

impl Speed {
    pub fn new(range: RangeInclusive<f32>) -> Self {
        Self {
            value: thread_rng().gen_range(range),
        }
    }
}
