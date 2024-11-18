use std::collections::HashMap;
use crate::fish::FreshwaterFish::{Alligator, Bluegill, Bowfin, Crab, Frog, GoldenBass, LargemouthBass, Toad, Turtle};
use crate::fish::SaltwaterFish::{Angelfish, GoldenRay, Grouper, Herring, Krill, MantaRay, SeaTurtle, Stingray, Whale};
// stores fish data
#[derive(Clone)]
pub struct Fish {
    species: FishSpecies,
    size: f64, // randomly picked from normal distribution
    quality:i32,
    value:f64
}
#[derive(Clone)]
pub enum FishSpecies{
    Freshwater(FreshwaterFish),
    Saltwater(SaltwaterFish)
}


#[derive(Clone)]
pub enum FreshwaterFish {
    LargemouthBass,
    Bluegill,
    Bowfin,
    GoldenBass,
    Alligator,
    Turtle,
    Toad,
    Frog,
    Crab
}

#[derive(Clone)]
pub enum SaltwaterFish {
    Angelfish,
    Grouper,
    Herring,
    Whale,
    Stingray,
    MantaRay,
    GoldenRay,
    SeaTurtle,
    Krill
}

static AVERAGE_SIZES:HashMap<FishSpecies, f64> = HashMap::from([
    (FishSpecies::Saltwater(GoldenRay), 7500.0),
    (FishSpecies::Freshwater(Crab), 10.0),
]);

static FRESHWATER_TIERS:HashMap<i32, Vec<FreshwaterFish>> = HashMap::from([
    (1, Vec::from([Crab, Frog, Bluegill, LargemouthBass])),
    (2, Vec::from([Turtle, Bowfin, Toad])),
    (3, Vec::from([Alligator, GoldenBass]))
]);

static SALTWATER_TIERS:HashMap<i32, Vec<SaltwaterFish>> = HashMap::from([
    (1, Vec::from([Angelfish, Grouper, Herring, Krill])),
    (2, Vec::from([Stingray, MantaRay, SeaTurtle])),
    (3, Vec::from([Whale, GoldenRay])),
]);