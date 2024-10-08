use serde_json::Value;

use super::msg::{GenericIncomingRequest, IncomingGameActionType};

pub const PLAYER_X_DEFAULT_START_POSTION: f32 = 0.;
pub const PLAYER_Y_DEFAULT_START_POSTION: f32 = 0.;

pub const MIN_X_POS: f32 = -1000.;
pub const MIN_Y_POS: f32 = -1000.;

pub const MAX_X_POS: f32 = 0.;
pub const MAX_Y_POS: f32 = 0.;

pub const DUCK_COLORS_LENGTH: usize = 6;

pub const available_duck_colors: [&str; DUCK_COLORS_LENGTH] = [
    "red",
    "blue",
    "green",
    "purple",
    "yellow",
    "pink"
];

pub const MIN_QUACK_MULTIPLIER: f32 = 0.33;
pub const MAX_QUACK_MULTIPLIER: f32 = 2.2;

pub const BASE_CRACKER_POINT_VALUE: u64 = 10;
pub const RANDOM_CRACKER_POINT_VALUE: u64 = 10;

pub const CRACKER_RADIUS: u64 = 5;
