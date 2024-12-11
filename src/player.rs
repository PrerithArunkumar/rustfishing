use std::collections::HashMap;

use crate::fish::*;
pub struct Player {
    lure: Lure,
    rod_tier: i32,
    catch_chance: f64,
    bait_capacity: i32,
    bait: HashMap<Bait, i32>,
    currency: i32,
    inventory: Vec<Fish>
}

pub enum Lure {
    Basic, // no special effects
    Fly, // picks smallest fish possible out of the 3 rolls
    Large, // picks largest fish possible out of the 3 rolls
    Efficient, // 25% chance to not subtract bait when fishing
    Sparkling, // picks highest tier fish out of the 3 rolls, 25% chance to consume extra bait
}

pub enum Bait {
    Worm,
    Leech,
    Minnow,
}