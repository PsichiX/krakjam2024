use crate::game::components::immobility::Immobility;
use hecs::World;

pub struct ImmobilityController;

impl ImmobilityController {
    pub fn run(world: &World, delta_time: f32) {
        for (_, (immobility,)) in world.query::<(&mut Immobility,)>().iter() {
            immobility.time_left = (immobility.time_left - delta_time).max(0.0);
        }
    }
}
