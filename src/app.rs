use std::io::SeekFrom::Current;
use std::vec;
use crate::fish::*;
use crate::player::*;

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

pub fn generate_fish(player: Player, scene: CurrentScene) -> Fish {
    let possible_fish:Vec<Fish> = Vec::new();
    for i in 0..4 {
        
    }
    return possible_fish[3].clone();
}