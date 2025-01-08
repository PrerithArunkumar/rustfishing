# Rustfishing

Rustfishing: A terminal-based fishing game written in Rust

## Group Members
- **Prerith Arunkumar** (UIN: prerith2)
- **Evan Wang** (UIN: evan13)

## Project Introduction
Rustfishing is a terminal-based fishing game inspired by the indie game Webfishing. The gameplay loop revolves around catching and selling fish to upgrade your fishing rod, hook, and bait. These upgrades enable players to catch bigger and rarer fish. While there is no definitive end goal, a super rare fish will act as the "endgame" achievement. This project combines fun with the challenge of applying concepts learned in CS 128H.

## Technical Overview
### Major Components
#### Fishing Mechanic
1. **Catching Fish**
   - Players are presented with a sequence of characters that must be retyped before they are obscured by the `▮` character. Successful retyping results in catching a fish.
   - An anti-cheat mechanism incorporates fish emojis (🐟, 🐠, 🐡, 🦈) between characters to prevent copying and pasting.
2. **Upgrades**
   - **Fishing Rod**: Determines the difficulty of catching fish. Higher-quality rods slow down the appearance of `▮` characters.
   - **Bait**: Affects the types of fish available for capture. Better bait enables catching higher-tier fish.
   - **Hooks**: Influences the likelihood of catching certain types of fish. Different hooks increase the chances for specific fish types.

#### UI
1. **Scenes**
   - **Fishing Areas**: 
     - Freshwater
     - Saltwater
   - **Shop**:
     - Fishing rod upgrades
     - Different baits
     - Different hooks

## References
- **Webfishing**: [Webfishing on Steam](https://store.steampowered.com/app/3146520/WEBFISHING)
  - Inspirations: List of fish, loot generation, fishing mechanic

