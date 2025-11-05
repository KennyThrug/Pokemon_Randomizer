# Pokemon_Randomizer
Launcher (and possibly runner) for a randomizer pokemon

## Features:
- [ ] Launcher to set settings
- [x] Automatic Compilation of Roms
- [x] Up to Generation 9 pokemon
- [x] Physical Special Split Implemented

## Settings:
### Pokemon Randomization
- [x] Randomize Wild Pokemon
- [x] Allow Pokemon from future generations
- [x] Wild pokemon Scale with Routes
- [x] Allow Legendaries to be Randomized
- [x] Force Legendary pokemon to randomize into other Legendaries
- [x] Allow Mega Pokemon in Pool
### Region Randomization
- [ ] Multiple Regions in a game
- [ ] Kanto
- [ ] Johto?
- [ ] 
### Starter Randomization
- [x] Randomize Starter Pokemon
- [x] Allow Legendaries as Starters
- [x] Force Starter to be first stage or baby pokemon
- [x] Have May/Brendan keep other Starter
- [x] Have May/Brendan keep Whole Team
- [x] Have Wally keep Raltz randomization
### Trainer Randomization
- [x] Randomize Trainer Pokemon
- [x] Trainer Pokemon Scale with Routes
- [x] Allow Legendaries on Regular Trainers
- [x] Allow Only 1 Legendary per Team
- [x] Rarity for Legendaries
### Gym Randomization
- [ ] Gym Leader Keys
- [x] Allow Legendaries on Gym Leaders
- [x] Force Gym pokemon to share type
- [x] Randomize Gym Types
- [x] Get Random Pokemon as Gym Reward
- [x] Get pokemon of same type as gym as reward?
- [x] Gimmick Stone Randomization as Gym Reward
- [ ] Gym Location Randomization
### Item Randomization
- [x] Add Extra Rare Candies to Item Pool
- [x] Add Held Items
- [x] Get Items from Trainers
- [x] Get Important Items only from Trainers
- [x] Allow Poke-balls in Pool
- [x] Make Balls always successful
- [x] Limit Ball number in pool
- [x] Put Healing Items in Pool
- [x] Put Revives in Pool
- [x] Randomize Hidden (Itemfinder) Items
- [ ] Randomize Items from talking to NPCs
- [x] Randomize Key Items (Fishing Pole, Bike)
- [x] Randomize Mega Stones
- [x] Randomize Z crystals
- [ ] Randomize Running Shoes
- [x] Randomize TMs
- [ ] Gym Leader Keys
- [ ] Randomize Gym Badges (in Normal pool)
- [ ] Randomize Berries
### Evolution Settings
- [ ]
### Cosmetic Settings
- [ ] Random sprite for PC (Male, Female, Ruby/Sapphire, FRLG)
- [ ] 
### Other Settings
- [ ] Allow HM's to be used without pokemon
- [ ] Change [Evolution](evolution.md) Requirements for weird pokemon
- [ ] Allow [Forms](forms.md) to appear for many pokemon
- [ ] Level Caps on Gyms
- [x] Randomize Professor Birch's example pokemon
- [ ] Randomize Battle Transitions for Legendaries
- [ ] NPC to turn Pokemon to egg
- [x] Follower Pokemon
- [ ] Enforce level cap
- [ ] Silly extras and Troll items (such as a gender change or reversing the name of the lead pokemon, teleporting you somewhere, etc.)



# Technical Information
## Known Bugs
Gives non-randomized items for randomized gyms if "Give Gimmick Stones" is Checked

## Files that are edited by the randomizer
src/data/trainers.party
src/data/wild_encounters
src/main_menu.c

# 2025 update:
Broken files:
src/main_menu.c
src/battle_setup.c
src/data/trainers.party

data/map_events.o
src/maps/*