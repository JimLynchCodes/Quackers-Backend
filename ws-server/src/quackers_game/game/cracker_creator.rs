use rand::Rng;

use crate::quackers_game::types::defaults::{BASE_CRACKER_POINT_VALUE, CRACKER_RADIUS, MAX_X_POS, MAX_Y_POS, MIN_X_POS, MIN_Y_POS, RANDOM_CRACKER_POINT_VALUE};

use super::game_state::CrackerData;

pub fn generate_random_cracker_data() -> CrackerData {
    let mut rng = rand::thread_rng();

    let random_x_pos: f32 = rng.gen_range(MIN_X_POS..MAX_X_POS);
    let random_y_pos: f32 = rng.gen_range(MIN_Y_POS..MAX_Y_POS);

    let random_point_value: u64 =
        BASE_CRACKER_POINT_VALUE + rng.gen_range(0..RANDOM_CRACKER_POINT_VALUE);

    CrackerData {
        points: random_point_value,
        x_pos: random_x_pos,
        y_pos: random_y_pos,
        radius: CRACKER_RADIUS,
    }
}
