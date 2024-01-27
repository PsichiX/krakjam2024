use std::borrow::Cow;

pub struct ParticleGenerator {
    pub texture: Cow<'static, str>,
    pub emmission_accumulator: f32,
    pub emmission_time: f32,
    pub batch_size: usize,
}
