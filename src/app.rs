use std::{
    io,
    collections::HashMap,
    time::{Duration, Instant},
};
use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    style::Stylize,
    widgets::Paragraph,
    DefaultTerminal,
    Frame,
};
use rand_distr::{Normal, Distribution};
use rand::{seq::SliceRandom, distributions::Alphanumeric, Rng};

use crate::fish::*;
use crate::fish::FreshwaterFish::*;
use crate::fish::SaltwaterFish::*;
use crate::player::*;

pub struct App {
    pub exit: bool,
    pub scene: CurrentScene,
    pub player: Player,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events(terminal)?;
        }
        Ok(())
    }

    pub fn draw(&mut self, frame: &mut Frame) {
        match self.scene {
            CurrentScene::MainMenu => {
                let text = Paragraph::new("Main Menu (l: Lake, b: Beach, s: Shop, i: Inventory, q: Quit)").white().on_blue();
                frame.render_widget(text, frame.size());
            }
            CurrentScene::Shop(ShopScenes::Base) => {
                let text = Paragraph::new("Shop (x: Buy, y: Sell, m: Main Menu)").white().on_blue();
                frame.render_widget(text, frame.size());
            }
            CurrentScene::Shop(ShopScenes::Buy) => {
                let text = Paragraph::new("Shop - Buy (m: Main Menu)").white().on_blue();
                frame.render_widget(text, frame.size());
            }
            CurrentScene::Shop(ShopScenes::Sell) => {
                let text = Paragraph::new("Shop - Sell (m: Main Menu)").white().on_blue();
                frame.render_widget(text, frame.size());
            }
            CurrentScene::Lake(Lake::Lake) => {
                let text = Paragraph::new("Lake (f: Fish, m: Main Menu)").white().on_blue();
                frame.render_widget(text, frame.size());
            }
            CurrentScene::Lake(Lake::Minigame) => {
                let text = Paragraph::new("Fishing Minigame in Progress (Press 'm' to go back to Main Menu)").white().on_blue();
                frame.render_widget(text, frame.size());
                self.fishing_minigame(frame).expect("Failed to run fishing minigame");
            }
            CurrentScene::Beach(Beach::Beach) => {
                let text = Paragraph::new("Beach (f: Fish, m: Main Menu)").white().on_blue();
                frame.render_widget(text, frame.size());
            }
            CurrentScene::Beach(Beach::Minigame) => {
                let text = Paragraph::new("Fishing Minigame in Progress (Press 'm' to go back to Main Menu)").white().on_blue();
                frame.render_widget(text, frame.size());
                self.fishing_minigame(frame).expect("Failed to run fishing minigame");
            }
            CurrentScene::Inventory => {
                let text = Paragraph::new(format!("Inventory: {} fish (m: Main Menu)", self.player.inventory.len())).white().on_blue();
                frame.render_widget(text, frame.size());
            }
        }
    }

    pub fn handle_events(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        if let event::Event::Key(key) = event::read()? {
            match key.kind {
                KeyEventKind::Press => match key.code {
                    KeyCode::Char('q') => self.exit = true,
                    KeyCode::Char('s') => self.scene = CurrentScene::Shop(ShopScenes::Base),
                    KeyCode::Char('x') => self.scene = CurrentScene::Shop(ShopScenes::Buy),
                    KeyCode::Char('y') => self.scene = CurrentScene::Shop(ShopScenes::Sell),
                    KeyCode::Char('l') => self.scene = CurrentScene::Lake(Lake::Lake),
                    KeyCode::Char('f') => match self.scene {
                        CurrentScene::Lake(Lake::Lake) => self.scene = CurrentScene::Lake(Lake::Minigame),
                        CurrentScene::Beach(Beach::Beach) => self.scene = CurrentScene::Beach(Beach::Minigame),
                        _ => {},
                    },
                    KeyCode::Char('b') => self.scene = CurrentScene::Beach(Beach::Beach),
                    KeyCode::Char('i') => self.scene = CurrentScene::Inventory,
                    KeyCode::Char('m') => self.scene = CurrentScene::MainMenu,
                    _ => {}
                },
                _ => {}
            }
        }
        Ok(())
    }

    pub fn fishing_minigame(&mut self, frame: &mut Frame) -> io::Result<bool> {
        let fish = generate_fish(&self.player, &self.scene).unwrap();
        let difficulty: f64 = fish.get_size_mult() * (fish.get_tier().clone() as f64) * (fish.get_size() / 1000.0).ceil();
        let char_count = 2 * difficulty.round() as i32;
        let emojis = Vec::from(['🐟', '🐡', '🐠', '🦈', '🎣']);
        let challenge_word: String = rand::thread_rng().sample_iter(&Alphanumeric).take(char_count as usize).map(char::from).collect();
        let hidden = '▮';
        let mut hidden_chars = 0;
        let mut player_word = String::from("");
        let start_time = Instant::now();
        let time_per_char = 4 - fish.get_tier().clone();
        let mut fish_caught = false;

        loop {
            hidden_chars = Instant::now().duration_since(start_time).as_secs() as i32 / time_per_char;
            let mut text = String::from("");
            text.push(emojis.choose(&mut rand::thread_rng()).unwrap().clone());
            for i in 0..hidden_chars {
                text.push(hidden);
                text.push(emojis.choose(&mut rand::thread_rng()).unwrap().clone());
            }
            for i in hidden_chars..char_count {
                text.push(challenge_word.chars().nth(i as usize).unwrap().clone());
                text.push(emojis.choose(&mut rand::thread_rng()).unwrap().clone());
            }

            // Draw the minigame state
            self.draw_fishing_minigame(frame, &text, &player_word)?;

            if let Ok(event) = event::poll(Duration::from_millis(100)) {
                if event {
                    if let event::Event::Key(key) = event::read()? {
                        if key.kind == KeyEventKind::Press {
                            match key.code {
                                KeyCode::Char(c) => player_word.push(c),
                                KeyCode::Backspace => { player_word.pop(); },
                                _ => {},
                            }
                        }
                    }
                }
            }

            if player_word == challenge_word {
                fish_caught = true;
                break;
            }
            if hidden_chars == char_count as i32 {
                break;
            }
        }

        if fish_caught {
            self.player.inventory.push(fish);

            if self.player.lure == Lure::Efficient {
                if rand::random::<f64>() > 0.25 {
                    if let Some(count) = self.player.bait.get_mut(&self.player.current_bait) {
                        *count -= 1;
                    }
                }
            } else {
                if let Some(count) = self.player.bait.get_mut(&self.player.current_bait) {
                    *count -= 1;
                }
            }
            return Ok(true);
        }

        Ok(false)
    }

    fn draw_fishing_minigame(&mut self, frame: &mut Frame, text: &str, player_word: &str) -> io::Result<()> {
        let text_widget = Paragraph::new(text)
            .white()
            .on_blue();
        frame.render_widget(text_widget, frame.size());

        let player_widget = Paragraph::new(player_word)
            .white()
            .on_blue();
        frame.render_widget(player_widget, frame.size());
        Ok(())
    }
}

pub enum CurrentScene {
    MainMenu,
    Shop(ShopScenes),
    Lake(Lake),
    Beach(Beach),
    Inventory,
}

pub enum ShopScenes {
    Base,
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

pub fn generate_fish(player: &Player, scene: &CurrentScene) -> Result<Fish, String> {
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
    let mut biggest_index = 0;
    let mut smallest_size = f64::MAX;
    let mut smallest_index = 0;
    let mut best_tier = 0;
    let mut best_index = 0;

    for i in 0..3 {
        let tier_rand = rand::random::<f64>();
        let tier:i32;
        let size = 0.1_f64.max(size_dist.sample(&mut rand::thread_rng()));
        match player.current_bait {
            Bait::Worm => match tier_rand{
                0.0..0.1 => tier = 3,
                0.1..0.4 => tier = 2,
                _ => tier = 1
            },
            Bait::Leech => match tier_rand{
                0.0..0.15 => tier = 3,
                0.15..0.55 => tier = 2,
                _ => tier = 1
            },
            Bait::Minnow => match tier_rand{
                0.0..0.2 => tier = 3,
                0.2..0.6 => tier = 2,
                _ => tier = 1
            },
        }
        
        let species:FishSpecies;

        match &scene {
            CurrentScene::Lake(crate::app::Lake::Minigame) => species = FishSpecies::Freshwater(freshwater_tiers[&tier].choose(&mut rand::thread_rng()).unwrap().clone()),
            CurrentScene::Beach(crate::app::Beach::Minigame) => species = FishSpecies::Saltwater(saltwater_tiers[&tier].choose(&mut rand::thread_rng()).unwrap().clone()),
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
        let fish = Fish::new(species.clone(), size * fish_size[&species], size, value * price_multiplier, tier);
        if size > biggest_size {
            biggest_index = i;
            biggest_size = size;
        }
        if size < smallest_size {
            smallest_index = i;
            smallest_size = size;
        } if tier > best_tier {
            best_index = i;
            best_tier = tier;
        }
        possible_fish.push(fish);
        
    }
    if player.lure == Lure::Fly {
        return Ok(possible_fish.remove(smallest_index));
    }
    if player.lure == Lure::Large {
        return Ok(possible_fish.remove(biggest_index));
    }
    if player.lure == Lure::Sparkling {
        return Ok(possible_fish.remove(best_index));
    }
    return Ok(possible_fish.remove(2));
}

