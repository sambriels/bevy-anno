use crate::prelude::*;

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CursorPosition::default())
            .add_system(update_cursor.run_in_state(GameState::Playing));
    }
}

#[derive(Default, Debug)]
pub struct CursorPosition {
    pub screen: Vec2,
    pub world: Vec3,
}

pub fn update_cursor(
    windows: Res<Windows>,
    camera_q: Query<(&Transform, &Camera)>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut cursor_position: ResMut<CursorPosition>,
) {
    for cursor_moved in cursor_moved_events.iter() {
        // To get the mouse's world position, we have to transform its window position by
        // any transforms on the camera. This is done by projecting the cursor position into
        // camera space (world space).
        for (cam_t, cam) in camera_q.iter() {
            let window = windows.primary();

            let window_size = Vec2::new(window.width(), window.height());

            // Convert screen position [0..resolution] to ndc [-1..1]
            // (ndc = normalized device coordinates)
            let ndc_to_world = cam_t.compute_matrix() * cam.projection_matrix().inverse();
            let ndc = (cursor_moved.position / window_size) * 2.0 - Vec2::ONE;
            cursor_position.world = ndc_to_world.project_point3(ndc.extend(0.0));
            cursor_position.screen = cursor_moved.position;
        }
    }
}
