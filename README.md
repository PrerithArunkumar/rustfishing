# rustfishing
Rustfishing: a terminal-based fishing game in rust
#### Group Members:
Prerith Arunkumar, UIN: prerith2

Evan Wang, UIN: evan13


#### Project Introduction
Rustfishing is a terminal-based fishing game roughly inspired by the indie game Webfishing. The gameplay loop is simple: catch and sell fish to buy upgrades to your fishing rod, hook, and bait, which in turn will let you catch bigger and rarer fish. There is no end goal, although there will be a super rare fish that will serve as the "endgame" point if reached. We wanted to make this because we thought it would be a fun yet challenging way to apply everything we've learned in CS 128H so far. 

Technical Overview
    Please provide a moderate-length technical description of the major components of your project. This should also function as a sort of ‘roadmap’ for tasks you need to complete for your project to be functional.
    Please list what you plan to have finished by each checkpoint. These are meant to be goals to keep your project on track. We will NOT grade your checkpoints on how much you have completed, but on whether or not you’ve made some progress. 

* Major Components:
    - Fishing Mechanic
        - Catching Fish
            - When fishing, you will be given a sequence of characters that you will have to retype. These characters will slowly be hidden by the ▮ character one at a time; fully retype the word before it is hidden to successfully catch a fish.
            - Anticheat will be implemented by putting a fish emoji (🐟, 🐠, 🐡, or 🦈) in between each character to prevent copying and pasting the sequence. 
        - Upgrades
            - Fishing rod
                - determines how hard it is to catch fish
                - a better fishing rod makes the ▮ characters appear slower
            - Bait
                - determines types of fish you can catch
                - better bait lets you catch higher "tiers" of fish
            - Hooks
                - also determines types of fish you can catch
                - different hooks will give you an increased chance to catch certain types of fish
    - UI
        - Scenes
            - Fishing Areas
                - Freshwater
                - Saltwater
            - Shop
                - fishing rod upgrades
                - different baits
                - different hooks
- Checkpoints
    - Checkpoint 1
        - basic UI navigation (shop area, fishing area)
        - Fishing Mechanic with key detection
    - Checkpoint 2
        - fishing rod and bait upgrades
        - UI art, potentially
    

        


### Possible Challenges
- creating a UI in rust using Ratatui
- integrating all of the different mechanics and features of the game and making sure they work together smoothly

### References
- Webfishing: https://store.steampowered.com/app/3146520/WEBFISHING
    - Inspirations: list of fish, loot generation, fishing mechanic
