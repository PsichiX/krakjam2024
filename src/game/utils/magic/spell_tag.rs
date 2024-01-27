use micro_games_kit::third_party::{
    rand::{thread_rng, Rng},
    vek::Vec2,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SpellTag {
    Size(SpellTagSize),
    Speed(SpellTagSpeed),
    Effect(SpellTagEffect),
    Shape(SpellTagShape),
    Direction(SpellTagDirection),
    Trajectory(SpellTagTrajectory),
    Duration(SpellTagDuration),
}

impl SpellTag {
    pub fn random() -> Self {
        match thread_rng().gen_range(0..6) {
            0 => Self::Size(SpellTagSize::random()),
            1 => Self::Speed(SpellTagSpeed::random()),
            2 => Self::Effect(SpellTagEffect::random()),
            3 => Self::Shape(SpellTagShape::random()),
            4 => Self::Direction(SpellTagDirection::random()),
            5 => Self::Trajectory(SpellTagTrajectory::random()),
            6 => Self::Duration(SpellTagDuration::random()),
            _ => unreachable!(),
        }
    }

    pub fn as_size(&self) -> Option<SpellTagSize> {
        match self {
            Self::Size(result) => Some(*result),
            _ => None,
        }
    }

    pub fn as_speed(&self) -> Option<SpellTagSpeed> {
        match self {
            Self::Speed(result) => Some(*result),
            _ => None,
        }
    }

    pub fn as_effect(&self) -> Option<SpellTagEffect> {
        match self {
            Self::Effect(result) => Some(*result),
            _ => None,
        }
    }

    pub fn as_shape(&self) -> Option<SpellTagShape> {
        match self {
            Self::Shape(result) => Some(*result),
            _ => None,
        }
    }

    pub fn as_direction(&self) -> Option<SpellTagDirection> {
        match self {
            Self::Direction(result) => Some(*result),
            _ => None,
        }
    }

    pub fn as_trajectory(&self) -> Option<SpellTagTrajectory> {
        match self {
            Self::Trajectory(result) => Some(*result),
            _ => None,
        }
    }

    pub fn as_duration(&self) -> Option<SpellTagDuration> {
        match self {
            Self::Duration(result) => Some(*result),
            _ => None,
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

    pub fn scale(&self) -> Vec2<f32> {
        match self {
            SpellTagSize::Large => Vec2::new(4.0, 4.0).into(),
            SpellTagSize::Medium => Vec2::new(2.0, 2.0).into(),
            SpellTagSize::Small => Vec2::new(1.0, 1.0).into(),
        }
    }

    pub fn radius(&self) -> f32 {
        match self {
            SpellTagSize::Large => 40.0,
            SpellTagSize::Medium => 20.0,
            SpellTagSize::Small => 10.0,
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

    pub fn time(&self) -> f32 {
        match self {
            SpellTagDuration::Long => 8.0,
            SpellTagDuration::Medium => 2.0,
            SpellTagDuration::Quick => 0.5,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SpellTagShape {
    #[default]
    Point,
    Wall,
    Triangle,
}

impl SpellTagShape {
    pub fn random() -> Self {
        match thread_rng().gen_range(0..3) {
            0 => Self::Point,
            1 => Self::Wall,
            2 => Self::Triangle,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SpellTagDirection {
    #[default]
    Forward,
    Backward,
    Down,
}

impl SpellTagDirection {
    pub fn random() -> Self {
        match thread_rng().gen_range(0..3) {
            0 => Self::Forward,
            1 => Self::Backward,
            2 => Self::Down,
            _ => unreachable!(),
        }
    }

    pub fn multiplier(&self) -> f32 {
        match self {
            Self::Forward => 1.0,
            Self::Backward => -1.0,
            Self::Down => 0.0,
        }
    }
}
