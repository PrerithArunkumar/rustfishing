use core::f64;
use std::io;
use ratatui::{DefaultTerminal, Frame};
use rand::seq::SliceRandom;
use std::collections::HashMap;
use rand_distr::{Normal, Distribution};
use std::cmp::*;
use crate::fish::*;
use crate::fish::FreshwaterFish::*;
use crate::fish::SaltwaterFish::*;

use crate::player::*;

pub struct App {
    exit:bool
}
impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }
    fn draw(&self, frame: &mut Frame) {
        todo!()
    }

    fn handle_events(&mut self) -> io::Result<()> {
        todo!()
    }
}

pub enum CurrentScene {
    Shop(ShopScenes),
    Lake(Lake),
    Beach(Beach),
    Inventory,
}
pub enum ShopScenes {
    Buy,
    Sell,
}
pub enum Lake {
    Lake,
    Minigame,
}
pub enum Beach {
    Beach,
    Minigame,
}

pub fn generate_fish(player: Player, scene: CurrentScene) -> Result<Vec<Fish>, String> {
    let freshwater_tiers:HashMap<i32, Vec<FreshwaterFish>> = HashMap::from([
        (1, Vec::from([Bluegill, LargemouthBass])),
        (2, Vec::from([Turtle, Crab])),
        (3, Vec::from([GoldenBass, Alligator]))
    ]);
    let saltwater_tiers:HashMap<i32, Vec<SaltwaterFish>> = HashMap::from([
        (1, Vec::from([Krill, Herring])),
        (2, Vec::from([MantaRay, Grouper])),
        (3, Vec::from([GoldenRay, Whale])),
    ]);

    let fish_size:HashMap<FishSpecies, f64> = HashMap::from([
        (FishSpecies::Freshwater(Bluegill), 18.75),
        (FishSpecies::Freshwater(LargemouthBass), 56.25),
        (FishSpecies::Freshwater(Turtle),  30.0),
        (FishSpecies::Freshwater(Crab), 10.0),
        (FishSpecies::Freshwater(GoldenBass), 56.25),
        (FishSpecies::Saltwater(Grouper), 1375.0),
        (FishSpecies::Saltwater(Herring), 37.5),
        (FishSpecies::Saltwater(Whale), 27.5),
        (FishSpecies::Saltwater(MantaRay), 7500.0),
        (FishSpecies::Saltwater(GoldenRay), 7500.0),
    ]);
    let size_dist = Normal::new(1.25, 0.55).unwrap();
    let sell_value:HashMap<FishSpecies, f64> = HashMap::from([
        (FishSpecies::Freshwater(Bluegill), 58.0),
        (FishSpecies::Freshwater(LargemouthBass), 50.0),
        (FishSpecies::Freshwater(Turtle), 373.0),
        (FishSpecies::Freshwater(Crab), 196.0),
        (FishSpecies::Freshwater(GoldenBass), 1969.0),
        (FishSpecies::Saltwater(Grouper), 87.0),
        (FishSpecies::Saltwater(Herring), 54.0),
        (FishSpecies::Saltwater(Whale), 1117.0),
        (FishSpecies::Saltwater(MantaRay), 776.0),
        (FishSpecies::Saltwater(GoldenRay), 1969.0),
    ]);

    let mut possible_fish:Vec<Fish> = Vec::new();

    // for special hooks
    let mut biggest_size = 0.0;
    let mut biggest:&Fish;
    let mut smallest_size = f64::MAX;
    let mut smallest:&Fish;
    let mut highest_tier = 0;
    let mut highest:&Fish;

    for i in 0..3 {
        let tier_rand = rand::random::<f64>();
        let tier:i32;
        let size = 0.1_f64.max(size_dist.sample(&mut rand::thread_rng()));
        match tier_rand{
            0.0..0.1 => tier = 3,
            0.1..0.4 => tier = 2,
            _ => tier = 1
        };
        let species:FishSpecies;

        match &scene {
            CurrentScene::Lake(crate::app::Lake::Lake) => species = FishSpecies::Freshwater(freshwater_tiers[&tier].choose(&mut rand::thread_rng()).unwrap().clone()),
            CurrentScene::Beach(crate::app::Beach::Beach) => species = FishSpecies::Saltwater(saltwater_tiers[&tier].choose(&mut rand::thread_rng()).unwrap().clone()),
            _ => return Err("Invalid Scene".to_string()),
        };
        let value = sell_value[&species];
        let price_multiplier = match size {
            0.1..0.25 => 1.75,
            0.25..0.5 => 0.6,
            0.5..1.0 => 0.8,
            1.0..1.5 => 1.0,
            1.5..2.0 => 1.5,
            2.0..3.0 => 2.5,
            _ => 4.25
        };
        let fish = Fish::new(species.clone(), size * fish_size[&species], value * price_multiplier);
        if (size > biggest_size) {
            biggest_size = size;
            biggest = &fish;
        }
        if (size < smallest_size) {
            smallest_size = size;
            smallest = &fish;
        } if (tier > highest_tier) {
            highest_tier = tier;
            highest = &fish;
        }
        possible_fish.push(fish);
        
    }
    return Ok(possible_fish);
}