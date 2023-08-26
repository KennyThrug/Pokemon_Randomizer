//An... Exhastive list of every single pokemon
use Type::*; 
use LegendStatus::*;

#[derive(PartialEq)]
pub enum Type{
    Normal,
    Fighting,
    Flying,
    Poison,
    Ground,
    Rock,
    Bug,
    Ghost,
    Steel,
    Fire,
    Water,
    Grass,
    Electric,
    Psychic,
    Ice,
    Dragon,
    Dark,
    Fairy,
    None
}
#[derive(PartialEq)]
pub enum LegendStatus{
    Standard,
    Legendary,
    Mythical,
    Mega,
}
pub struct Pokemon<'a>{
    pub name: &'a str,
    pub type1: Type,
    pub type2: Type,
    pub evo_stage: i16,
    pub total_stages: i16,
    pub status: LegendStatus, //True if Legendary
    pub generation: i16,
}
pub fn convert_name(pkmn: &Pokemon) -> String{
    return format!("SPECIES_{}",pkmn.name.to_uppercase());
}

pub fn get_species(name: &str) -> &Pokemon<'static>{
    for i in 0..LIST_OF_POKEMON.len(){
        if LIST_OF_POKEMON[i].name == name{
            return &LIST_OF_POKEMON[i];
        }
    }
    return &LIST_OF_POKEMON[0];
}

const LEGEND_OFFSET : i32 = 50; //Level above "possible" level where they can start appearing
const MEGA_OFFSET : i32 = 15; //Level above final stage evo where it is possible to find Mega forms in wild
const MYTHICAL_OFFSET: i32 = 55;

//Gen3 File Example: SPECIES_WURMPLE
//add SPECIES_ and then set name to uppercase
pub const LIST_OF_POKEMON : [Pokemon; 144-50] = [ //Update this number if List Expands (first line number - last line number)
    Pokemon {name: "Bulbasaur",type1: Grass,type2: Poison,evo_stage:1,total_stages: 3,status: Standard,generation: 1},
    Pokemon {name: "Ivysaur",type1: Grass, type2: Poison, evo_stage: 2, total_stages: 3, status: Standard, generation: 1},
    Pokemon {name: "Venusaur", type1: Grass, type2: Poison, evo_stage: 3, total_stages: 3, status: Standard, generation: 1},
    Pokemon {name: "Venusaur_Mega",type1: Grass, type2: Poison, evo_stage: 3, total_stages: 3, status: Mega, generation: 6},
    Pokemon {name: "Charmander",type1: Fire, type2:None,evo_stage:1,total_stages:3,status: Standard, generation: 1},
    Pokemon {name: "Charmeleon",type1: Fire, type2: None, evo_stage: 2, total_stages: 3, status: Standard, generation:1},
    Pokemon {name: "Charizard",type1: Fire, type2: Flying, evo_stage: 3, total_stages: 3, status: Standard, generation:1},
    Pokemon {name: "Charizard_Mega_X",type1: Fire, type2: Dragon,evo_stage:3,total_stages:3,status:Mega,generation:6},
    Pokemon {name: "Charizard_Mega_Y",type1: Fire,type2: Flying, evo_stage:3, total_stages:3,status:Mega,generation:6},
    Pokemon {name: "Squirtle",type1:Water,type2:None,evo_stage:1,total_stages:3,status:Standard,generation:1},
    Pokemon {name: "Wartortle",type1:Water,type2:None,evo_stage:2,total_stages:3,status:Standard,generation:1},
    Pokemon {name: "Blastoise",type1:Water,type2:None,evo_stage:3,total_stages:3,status:Standard,generation:1},
    Pokemon {name: "Blastoise_Mega",type1:Water,type2:None,evo_stage:3,total_stages:3,status:Mega,generation:6},
    Pokemon {name: "Caterpie",type1:Bug,type2:None,evo_stage:1,total_stages:3,status:Standard,generation:1},
    Pokemon {name: "Metapod",type1:Bug,type2:None,evo_stage:2,total_stages:3,status:Standard,generation:1},
    Pokemon {name: "Butterfree",type1:Bug,type2:Flying,evo_stage:3,total_stages:3,status:Standard,generation:1},
    Pokemon {name: "Weedle",type1:Bug,type2:Poison,evo_stage:1,total_stages:3,status:Standard,generation:1},
    Pokemon {name: "Kakuna",type1:Bug,type2:Poison,evo_stage:2,total_stages:3,status:Standard,generation:1},
    Pokemon {name: "Beedrill",type1:Bug,type2:Poison,evo_stage:3,total_stages:3,status:Standard,generation:1},
    Pokemon {name: "Beedrill_Mega",type1:Bug,type2:Poison,evo_stage:3,total_stages:3,status:Mega,generation:6},
    Pokemon {name: "Pidgey",type1:Normal,type2:Flying,evo_stage:1,total_stages:3,status:Standard,generation:1},
    Pokemon {name: "Pidgeotto",type1:Normal,type2:Flying,evo_stage:2,total_stages:3,status:Standard,generation:1},
    Pokemon {name: "Pidgeot",type1: Normal,type2:Flying,evo_stage:3,total_stages:3,status:Standard,generation:1},
    Pokemon {name: "Pidgeot_Mega",type1:Normal,type2:Flying,evo_stage:3,total_stages:3,status:Mega,generation:6},
    Pokemon {name: "Rattata",type1:Normal,type2:None,evo_stage:1,total_stages:2,status:Standard,generation:1},
    Pokemon {name: "Rattata_Alolan",type1:Dark,type2:Normal,evo_stage:1,total_stages:2,status:Standard,generation:7},
    Pokemon {name: "Raticate",type1: Normal,type2:None,evo_stage:2,total_stages:2,status:Standard,generation:1},
    Pokemon {name: "Raticate_Alolan",type1:Dark,type2:Normal,evo_stage:2,total_stages:2,status:Standard,generation:7},
    Pokemon {name: "Spearow",type1:Normal,type2:Flying,evo_stage:1,total_stages:2,status:Standard,generation:1},
    Pokemon {name: "Fearow",type1: Normal,type2:Flying,evo_stage:2,total_stages:2,status:Standard,generation:1},
    Pokemon {name: "Ekans",type1: Poison,type2:None,evo_stage:1,total_stages:2,status:Standard,generation:1},
    Pokemon {name: "Arbok",type1:Poison,type2:None,evo_stage:2,total_stages:2,status:Standard,generation:1},
    Pokemon {name: "Pikachu",type1:Electric,type2:None,evo_stage:1,total_stages:2,status:Standard,generation:1},
    Pokemon {name: "Raichu",type1:Electric,type2:None,evo_stage:2,total_stages:2,status:Standard,generation:1},
    Pokemon {name: "Raichu_Alolan",type1:Electric,type2:Psychic,evo_stage:2,total_stages:2,status:Standard,generation:7},
    Pokemon {name: "Sandshrew",type1: Ground, type2: None, evo_stage: 1, total_stages: 2, status: Standard,generation:1},
    Pokemon {name: "Sandshrew_Alolan",type1:Ice,type2:Steel,evo_stage:1,total_stages:2,status:Standard,generation:7},
    Pokemon {name: "Sandslash",type1:Ground,type2:None,evo_stage:2,total_stages:2,status:Standard,generation:1},
    Pokemon {name: "Sandslash_Alolan",type1:Ice,type2:Steel,evo_stage:2,total_stages:2,status:Standard,generation:7},
    Pokemon {name: "Nidoran_F",type1: Poison,type2: None,evo_stage:1,total_stages:3,status:Standard,generation:1},
    Pokemon {name: "Nidorina",type1:Poison,type2:None,evo_stage:2,total_stages:3,status:Standard,generation:1},
    Pokemon {name: "Nidoqueen", type1:Poison,type2:Ground,evo_stage:3,total_stages:3,status:Standard,generation:1},
    Pokemon {name: "Nidoran_M",type1: Poison,type2:None,evo_stage:1,total_stages:3,status:Standard,generation:1},
    Pokemon {name: "Nidorino",type1:Poison,type2:None,evo_stage:2,total_stages:3,status:Standard,generation:1},
    Pokemon {name: "Nidoking",type1:Poison,type2:Ground,evo_stage:3,total_stages:3,status:Standard,generation:1},
    Pokemon {name: "Clefairy",type1:Fairy,type2:None,evo_stage:1,total_stages:2,status:Standard,generation:1},
    Pokemon {name: "Clefable",type1:Fairy,type2:None,evo_stage:2,total_stages:2,status:Standard,generation:1},
    Pokemon {name: "Vulpix",type1:Fire,type2:None,evo_stage:1,total_stages:2,status:Standard,generation:1},
    Pokemon {name: "Vulpix_Alolan",type1:Ice,type2:None,evo_stage:1,total_stages:2,status:Standard,generation:7},
    Pokemon {name: "Ninetales",type1:Fire,type2:None,evo_stage:2,total_stages:2,status:Standard,generation:1},
    Pokemon {name: "Ninetales_Alolan",type1:Ice,type2:Fairy,evo_stage:2,total_stages:2,status:Standard,generation:7},
    Pokemon {name: "Jigglypuff",type1:Normal,type2:Fairy,evo_stage:1,total_stages:2,status:Standard,generation:1},
    Pokemon {name: "Wigglytuff",type1: Normal,type2:Fairy,evo_stage:2,total_stages:2,status:Standard,generation:1},
    Pokemon {name: "Zubat",type1:Poison,type2:Flying,evo_stage:1,total_stages:3,status:Standard,generation:1},
    Pokemon {name: "Golbat",type1:Poison,type2:Flying,evo_stage:2,total_stages:3,status:Standard,generation:1},
    Pokemon {name: "Oddish",type1:Grass,type2:Poison,evo_stage:1,total_stages:3,status:Standard,generation:1},
    Pokemon {name: "Gloom",type1:Grass,type2:Poison,evo_stage:2,total_stages:3,status:Standard,generation:1},
    Pokemon {name: "Vileplume",type1: Grass,type2: Poison,evo_stage:3,total_stages:3,status:Standard,generation:1},
    Pokemon {name: "Paras",type1:Bug,type2:Grass,evo_stage:1,total_stages:2,status:Standard,generation:1},
    Pokemon {name: "Parasect",type1:Bug,type2:Grass,evo_stage:2,total_stages:2,status:Standard,generation:1},
    Pokemon {name: "Venonat",type1:Bug,type2:Poison,evo_stage:1,total_stages:2,status:Standard,generation:1},
    Pokemon {name: "Venomoth",type1:Bug,type2:Poison,evo_stage:2,total_stages:2,status:Standard,generation:1},
    Pokemon {name: "Diglett",type1:Ground,type2:None,evo_stage:1,total_stages:2,status:Standard,generation:1},
    Pokemon {name: "Diglett_Alolan",type1:Ground,type2:Steel,evo_stage:1,total_stages:2,status:Standard,generation:7},
    Pokemon {name: "Dugtrio",type1:Ground,type2:None,evo_stage:2,total_stages:2,status:Standard,generation:1},
    Pokemon {name: "Dugtrio_Alolan",type1:Ground,type2:Steel,evo_stage:2,total_stages:2,status:Standard,generation:7},
    Pokemon {name: "Meowth",type1:Normal,type2:None,evo_stage:1,total_stages:2,status:Standard,generation:1},
    Pokemon {name: "Meowth_Alolan",type1:Dark,type2:None,evo_stage:1,total_stages:2,status:Standard,generation:7},
    Pokemon {name: "Meowth_Galarian",type1:Steel,type2:None,evo_stage:1,total_stages:2,status:Standard,generation:8},
    Pokemon {name: "Persian",type1:Normal,type2:None,evo_stage:2,total_stages:2,status:Standard,generation:1},
    Pokemon {name: "Persian_Alolan",type1:Dark,type2:None,evo_stage:2,total_stages:2,status:Standard,generation:7},
    Pokemon {name: "Psyduck",type1:Water,type2:None,evo_stage:1,total_stages:2,status:Standard,generation:1},
    Pokemon {name: "Golduck",type1: Water,type2: None, evo_stage:2,total_stages:2,status:Standard,generation:1},
    Pokemon {name: "Mankey",type1:Fighting,type2:None,evo_stage:1,total_stages:2,status:Standard,generation:1},
    Pokemon {name: "Primeape",type1:Fighting,type2:None,evo_stage:2,total_stages:2,status:Standard,generation:1},
    Pokemon {name: "Growlithe",type1:Fire,type2:None,evo_stage:1,total_stages:2,status:Standard,generation:1},
    Pokemon {name: "Growlithe_Hisuian",type1:Fire,type2:Rock,evo_stage:1,total_stages:2,status:Standard,generation:8},
    Pokemon {name: "Arcanine",type1:Fire,type2:None,evo_stage:2,total_stages:2,status:Standard,generation:1},
    Pokemon {name: "Arcanine_Hisuian",type1:Fire,type2:Rock,evo_stage:2,total_stages:2,status:Standard,generation:8},
    Pokemon {name: "Poliwag",type1:Water,type2:None,evo_stage:1,total_stages:3,status:Standard,generation:1},
    Pokemon {name: "Poliwhirl",type1:Water,type2:None,evo_stage:2,total_stages:3,status:Standard,generation:1},
    Pokemon {name: "Poliwrath",type1:Water,type2:Fighting,evo_stage:3,total_stages:3,status:Standard,generation:1},
    Pokemon {name: "Abra",type1:Psychic,type2:None,evo_stage:1,total_stages:3,status:Standard,generation:1},
    Pokemon {name: "Kadabra",type1:Psychic,type2:None,evo_stage:2,total_stages:3,status:Standard,generation:1},
    Pokemon {name: "Alakazam",type1:Psychic,type2:None,evo_stage:3,total_stages:3,status:Standard,generation:1},
    Pokemon {name: "Alakazam_Mega",type1:Psychic,type2:None,evo_stage:3,total_stages:3,status:Mega,generation:6},
    Pokemon {name: "Machop",type1:Fighting,type2:None,evo_stage:1,total_stages:3,status:Standard,generation:1},
    Pokemon {name: "Machoke",type1:Fighting,type2:None,evo_stage:2,total_stages:3,status:Standard,generation:1},
    Pokemon {name: "Machamp",type1:Fighting,type2:None,evo_stage:3,total_stages:3,status:Standard,generation:1},
    Pokemon {name: "Bellsprout",type1:Grass,type2:Poison,evo_stage:1,total_stages:3,status:Standard,generation:1},
    Pokemon {name: "Weepinbell",type1:Grass,type2:Poison,evo_stage:2, total_stages:3,status:Standard,generation:1},
    Pokemon {name: "Victreebel",type1:Grass,type2:Poison,evo_stage:3,total_stages:3,status:Standard,generation:1},
    Pokemon {name: "Tentacool",type1:Water,type2:Poison,evo_stage:1,total_stages:2,status:Standard,generation:1},
    Pokemon {name: "Tentacruel",type1:Water,type2:Poison,evo_stage:2,total_stages:2,status:Standard,generation:1}
];