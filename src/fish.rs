// stores fish data
#[derive(Debug, Clone)]
pub struct Fish {
    species: FishSpecies,
    size: f64, // randomly picked from normal distribution
    size_mult: f64,
    value:f64,
    tier: i32
}

impl Fish {
    pub fn new(species: FishSpecies, size: f64, size_mult: f64, value:f64, tier:i32) -> Self { Self {species, size, size_mult, value, tier} }
    pub fn get_species(&self) -> &FishSpecies { &self.species }
    pub fn get_size(&self) -> &f64 { &self.size }
    pub fn get_value(&self) -> &f64 { &self.value }
    pub fn get_tier(&self) -> &i32 { &self.tier }
    pub fn get_size_mult(&self) -> &f64 { &self.size_mult }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum FishSpecies{
    Freshwater(FreshwaterFish),
    Saltwater(SaltwaterFish)
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum FreshwaterFish {
    LargemouthBass,
    Bluegill,
    GoldenBass,
    Turtle,
    Crab,
    Alligator
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum SaltwaterFish {
    Grouper,
    Herring,
    Whale,
    MantaRay,
    GoldenRay,
    Krill
}

