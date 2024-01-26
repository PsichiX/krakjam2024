#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SpellTag {
    Effect(SpellTagEffect),
    Trajectory(SpellTagTrajectory),
    Size(SpellTagSize),
    Speed(SpellTagSpeed),
    Shape(SpellTagShape),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SpellTagEffect {
    Fire,
    Water,
    Electric,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SpellTagTrajectory {
    #[default]
    Straight,
    Sinus,
    Circle,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SpellTagSize {
    Small,
    #[default]
    Medium,
    Large,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SpellTagSpeed {
    Slow,
    #[default]
    Medium,
    Fast,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SpellTagDuration {
    Quick,
    #[default]
    Medium,
    Long,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SpellTagShape {
    #[default]
    Circle,
    Wall,
    Triangle,
}
