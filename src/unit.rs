use crate::prelude::*;

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Unit>()
            .add_enter_system(GameState::Playing, spawn_unit)
            .add_system(move_unit.run_in_state(GameState::Playing));
    }
}
