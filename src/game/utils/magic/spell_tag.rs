use micro_games_kit::third_party::rand::{thread_rng, Rng};
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SpellTag {
    Size(SpellTagSize),
    Speed(SpellTagSpeed),
    Shape(SpellTagShape),
    Effect(SpellTagEffect),
    Trajectory(SpellTagTrajectory),
}

impl SpellTag {
    pub fn random() -> Self {
        match thread_rng().gen_range(0..5) {
            0 => Self::Size(SpellTagSize::random()),
            1 => Self::Speed(SpellTagSpeed::random()),
            2 => Self::Shape(SpellTagShape::random()),
            3 => Self::Effect(SpellTagEffect::random()),
            4 => Self::Trajectory(SpellTagTrajectory::random()),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SpellTagEffect {
    Fire,
    Water,
    Electric,
}

impl SpellTagEffect {
    pub fn random() -> Self {
        match thread_rng().gen_range(0..3) {
            0 => Self::Fire,
            1 => Self::Water,
            2 => Self::Electric,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SpellTagTrajectory {
    #[default]
    Straight,
    Sinus,
    Circle,
}

impl SpellTagTrajectory {
    pub fn random() -> Self {
        match thread_rng().gen_range(0..3) {
            0 => Self::Straight,
            1 => Self::Sinus,
            2 => Self::Circle,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SpellTagSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl SpellTagSize {
    pub fn random() -> Self {
        match thread_rng().gen_range(0..3) {
            0 => Self::Small,
            1 => Self::Medium,
            2 => Self::Large,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SpellTagSpeed {
    Slow,
    #[default]
    Medium,
    Fast,
}

impl SpellTagSpeed {
    pub fn random() -> Self {
        match thread_rng().gen_range(0..3) {
            0 => Self::Slow,
            1 => Self::Medium,
            2 => Self::Fast,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SpellTagDuration {
    Quick,
    #[default]
    Medium,
    Long,
}

impl SpellTagDuration {
    pub fn random() -> Self {
        match thread_rng().gen_range(0..3) {
            0 => Self::Quick,
            1 => Self::Medium,
            2 => Self::Long,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SpellTagShape {
    #[default]
    Circle,
    Wall,
    Triangle,
}

impl SpellTagShape {
    pub fn random() -> Self {
        match thread_rng().gen_range(0..3) {
            0 => Self::Circle,
            1 => Self::Wall,
            2 => Self::Triangle,
            _ => unreachable!(),
        }
    }
}
