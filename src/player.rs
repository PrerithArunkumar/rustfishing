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
    Basic,
    Fly,
    Lucky,
    Quick,
    Efficient,
    Magnet,
    Large,
    Attractive,
    Double
}

pub enum Bait {
    Worm,
    Leech,
    Minnow,
    Squid,
    Nautilus
}