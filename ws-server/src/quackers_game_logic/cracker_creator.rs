use rand::Rng;

use super::types::{
    defaults::{
        available_duck_colors, BASE_CRACKER_POINT_VALUE, CRACKER_RADIUS, MAX_QUACK_MULTIPLIER,
        MAX_X_POS, MAX_Y_POS, MIN_QUACK_MULTIPLIER, MIN_X_POS, MIN_Y_POS,
        RANDOM_CRACKER_POINT_VALUE,
    },
    game_state::CrackerData,
};

pub fn generate_random_cracker_data() -> CrackerData {
    let mut rng = rand::thread_rng();

    // let colors_length: u64 = u64::try_from(available_duck_colors.len()).expect("error converting colors index")
    // let colors_length: u64 = available_duck_colors.len() as u64;

    // let random_color_index: u64 = rng.gen_range(0..=(colors_length - 1));
    // let random_quack_pitch: f64 = rng.gen_range(MIN_QUACK_MULTIPLIER..MAX_QUACK_MULTIPLIER);

    let random_x_pos: u64 = rng.gen_range(MIN_X_POS..MAX_X_POS);
    let random_y_pos: u64 = rng.gen_range(MIN_Y_POS..MAX_Y_POS);

    let random_point_value: u64 =
        BASE_CRACKER_POINT_VALUE + rng.gen_range(0..RANDOM_CRACKER_POINT_VALUE);

    CrackerData {
        points: random_point_value,
        x_pos: random_x_pos,
        y_pos: random_y_pos,
        radius: CRACKER_RADIUS,
    }
}
