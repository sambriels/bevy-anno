use bevy::math::Vec4Swizzles;

use crate::{components::CursorPos, prelude::*};

pub fn mouse_pos_in_map(cursor_pos: Res<CursorPos>, map_transform: &Transform) -> Vec2 {
    // Extend the cursor_pos vec3 by 1.0
    let cursor_pos = Vec4::from((cursor_pos.0, 1.0));
    let cursor_in_map_pos = map_transform.compute_matrix().inverse() * cursor_pos;
    cursor_in_map_pos.xy()
}
