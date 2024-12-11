// stores fish data
#[derive(Clone)]
pub struct Fish {
    species: FishSpecies,
    size: f64, // randomly picked from normal distribution
    value:f64
}

impl Fish {
    pub fn new(species: FishSpecies, size: f64, value:f64) -> Self {
        Self {species, size, value}
    }
    pub fn get_species(self) -> FishSpecies {
        self.species
    }
    pub fn get_size(self) -> f64 {
        self.size
    }
    pub fn get_value(self) -> f64 {
        self.value
    }
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

