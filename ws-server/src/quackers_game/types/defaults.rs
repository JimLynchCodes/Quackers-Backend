pub const PLAYER_X_DEFAULT_START_POSTION: f32 = 0.;
pub const PLAYER_Y_DEFAULT_START_POSTION: f32 = 0.;

pub const MIN_X_POS: f32 = -1000.;
pub const MIN_Y_POS: f32 = -1000.;

pub const MAX_X_POS: f32 = 1000.;
pub const MAX_Y_POS: f32 = 1000.;

pub const DUCK_COLORS_LENGTH: usize = 8;
pub const NAMES_LIST_LENGTH: usize = 51;

pub const AVAILABLE_DUCK_COLORS_WEIGHTED: [(&str, u32); DUCK_COLORS_LENGTH] = [
    ("teal", 5),
    ("yellow", 2),
    ("purple", 5),
    ("pink", 2),
    ("light_orange", 2),
    ("baby_blue", 1),
    ("lime_green", 5),
    ("white", 1),
];

pub const AVAILABLE_NAMES: [&str; NAMES_LIST_LENGTH] = [
    "Jimbo",
    "Chip",
    "Francesca",
    "Lucy",
    "Jerome",
    "Phillonius",
    "Faran",
    "Cody",
    "Bob",
    "Ella",
    "Jessica",
    "Scooter",
    "Louie",
    "Cindy",
    "Mary Lou",
    "Raphael",
    "John",
    "Diana",
    "Ernie",
    "Jack",
    "Mike",
    "Roger",
    "Peter",
    "Jiddle",
    "Sergio",
    "Julio",
    "Anne",
    "Alfred",
    "Chuck",
    "Ethan",
    "Fred",
    "Gertrude",
    "Harold",
    "Henrietta",
    "Isabelle",
    "Kyle",
    "Lunky",
    "Marge",
    "Moops",
    "Mims",
    "Mom",
    "Nancy",
    "Quinne",
    "Steve",
    "Sally",
    "Tommy",
    "Ulyses",
    "Victoria",
    "Willy",
    "Xavier",
    "Zoomy",
];

pub const _MIN_QUACK_MULTIPLIER: f32 = 0.33;
pub const _MAX_QUACK_MULTIPLIER: f32 = 2.2;

pub const BASE_CRACKER_POINT_VALUE: u64 = 10;
pub const RANDOM_CRACKER_POINT_VALUE: u64 = 10;

pub const PLAYER_RADIUS: u64 = 50;
pub const CRACKER_RADIUS: u64 = 20;
