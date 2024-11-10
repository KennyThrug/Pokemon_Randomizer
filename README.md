# Pokemon_Randomizer
Launcher (and possibly runner) for a randomizer pokemon

## Features:

- [x] Launcher to set settings
- [x] Automatic Compilation of Roms
- [x] Up to Generation 8 pokemon
- [x] Physical Special Split Implemented

## Settings:
### Pokemon Randomization
- [x] Randomize Wild Pokemon
- [x] Allow Pokemon from future generations
- [x] Wild pokemon Scale with Routes
- [x] Allow Legendaries to be Randomized
- [ ] Force Legendary pokemon to randomize into other Legendaries
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
- [ ] Allow Legendaries on Gym Leaders
- [ ] Force Gym pokemon to share type
- [ ] Randomize Gym Types
- [ ] Get Random Pokemon as Gym Reward
- [ ] Gimmick Stone Randomization as Gym Reward
- [ ] Gym Location Randomization
### Item Randomization
- [ ] Add Extra Rare Candies to Item Pool
- [ ] Add Held Items
- [ ] Get Items from Trainers
- [ ] Get Important Items only from Trainers
- [ ] Allow Poke-balls in Pool
- [ ] Make Balls Reusable if unsuccessful
- [ ] Limit Ball number in pool
- [ ] Put Healing Items in Pool
- [ ] Put Revives in Pool
- [ ] Randomize Hidden (Itemfinder) Items
- [ ] Randomize Items from talking to NPCs
- [ ] Randomize Key Items (Fishing Pole, Bike)
- [ ] Randomize Mega Stones
- [ ] Randomize Z crystals
- [ ] Randomize Running Shoes
- [ ] Randomize TMs
- [ ] Gym Leader Keys
- [ ] Randomize Gym Badges (in Normal pool)
- [ ] Randomize Berries
### Evolution Settings
- [ ]
### Cosmetic Settings
- [ ] Random sprite for PC (Male, Female, Ruby/Sapphire, FRLG)
- [ ] 
### Other Settings
- [x] Allow HM's to be used without pokemon
- [ ] Change [Evolution](evolution.md) Requirements for weird pokemon
- [ ] Allow [Forms](forms.md) to appear for many pokemon
- [ ] Level Caps on Gyms
- [x] Randomize Professor Birch's example pokemon
- [ ] Randomize Battle Transitions for Legendaries
- [ ] NPC to turn Pokemon to egg
- [ ] Follower Pokemon
- [ ] Enforce level cap
- [ ] Silly extras and Troll items (such as a gender change or reversing the name of the lead pokemon, teleporting you somewhere, etc.)



# Technical Information

## Files that are edited by the randomizer
src/data/trainers.party
src/data/wild_encounters
src/main_menu.c