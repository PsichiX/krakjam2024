use hecs::Entity;

pub struct IgnorePlayer {
    pub ignore_time: f32,
    pub player_entity: Option<Entity>,
}

impl IgnorePlayer {
    pub fn is_player(&self, entity: Entity) -> bool {
        if let Some(player_entity) = self.player_entity {
            if player_entity == entity {
                return true;
            }
        }

        return false;
    }

    pub fn should_be_ignored(&self, entity: Entity) -> bool {
        self.is_player(entity) && self.ignore_time > 0.0
    }
}
