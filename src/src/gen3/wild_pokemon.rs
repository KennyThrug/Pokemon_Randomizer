use crate::src::pokemon::LegendStatus;
use crate::src::settings;
use crate::src::pokemon;
use std::fs;
use super::trainers;

//If you want the starters, they are going to be in the Trainers file
const NUMBER_OF_ROUTES: usize = 124;
pub fn randomize_wild_pokemon(settings: &mut settings::Settings,pokemon_data: &Vec<pokemon::PokemonStats>){
    let data = fs::read_to_string("data/emerald/wild_encounters.json").expect("unable to read file");
    let mut parsed_data = json::parse(&data).unwrap();
    if settings.randomize_wild_pokemon{
        for i in 0..NUMBER_OF_ROUTES{
            for place in ["land_mons","water_mons","fishing_mons","rock_smash_mons","fishing_mons"]{
                for j in 0..30{ //Land Mons
                    if parsed_data["wild_encounter_groups"][0]["encounters"]
                    [i][place]["mons"][j]["species"] == json::JsonValue::Null{
                        break;
                    }
                    parsed_data["wild_encounter_groups"][0]["encounters"]
                    [i][place]["mons"][j]["species"] = get_random_wild_pokemon(settings,pokemon_data,
                    parsed_data["wild_encounter_groups"][0]["encounters"][i][place]["mons"][j]["min_level"].as_i16().unwrap()).into();
                }
            }
        }
    }
    //write to file
    fs::write("decomp/pokeemerald-expansion/src/data/wild_encounters.json",
    parsed_data.to_string()).expect("couldn't write to file");
    println!("Successfully wrote to file: src/data/wild_encounters.json");
}

//Gets a pokemon, checks if it can be put there, and returns the value
pub fn get_random_wild_pokemon(settings: &mut settings::Settings,pokemon_data: &Vec<pokemon::PokemonStats>,level: i16) -> String{
    //Make Random Pokemon
    let rand_val = settings::get_next_seed(0, pokemon_data.len() as i32, settings);
    let pokemon = pokemon_data[rand_val as usize].clone();
    //Check if it is a valid pokemon
    if !settings.allow_pokemon_future_generation && pokemon.generation >= 3{
        return get_random_wild_pokemon(settings, pokemon_data,level);
    }
    if settings.scale_wild_pokemon && pokemon.min_level > level{
        return get_random_wild_pokemon(settings, pokemon_data, level);
    }
    if settings.allow_legends_in_wild_pool == settings::WildLegends::NoLegends && (pokemon.status == LegendStatus::Legendary || pokemon.status == LegendStatus::LegendMega){
        return  get_random_wild_pokemon(settings, pokemon_data, level);
    }
    if settings.allow_legends_in_wild_pool == settings::WildLegends::SometimesLegends && (pokemon.status == LegendStatus::Legendary || pokemon.status == LegendStatus::LegendMega){
        if settings::get_next_seed(0, 25, settings) != 0{
            return get_random_wild_pokemon(settings, pokemon_data, level);
        }
    }
    if settings.allow_megas_in_wild_pool == settings::WildLegends::NoLegends && (pokemon.status == LegendStatus::Mega || pokemon.status == LegendStatus::LegendMega){
         return get_random_wild_pokemon(settings, pokemon_data, level)
    }
    if settings.allow_megas_in_wild_pool == settings::WildLegends::SometimesLegends && (pokemon.status == LegendStatus::Mega || pokemon.status == LegendStatus::LegendMega){
        if settings::get_next_seed(0, 25, settings) != 0{
            return get_random_wild_pokemon(settings, pokemon_data, level);
        }
    }

    //Return if it passes all checks
    return pokemon::format_pokemon_name(pokemon.pokemon_name);
}

//Gets a pokemon thats guarenteed to be a legendary
pub fn get_legendary_pokemon(settings: &mut settings::Settings,pokemon_data: &Vec<pokemon::PokemonStats>,level: i32) -> pokemon::PokemonStats{
    let rand_val = settings::get_next_seed(0, pokemon_data.len() as i32, settings);
    let pokemon = pokemon_data[rand_val as usize].clone();
    if settings.force_legendaries_to_legendaries == settings::LegendRarity::AlwaysLegendary && !(pokemon.status == LegendStatus::LegendMega || pokemon.status == LegendStatus::Legendary){
        return get_legendary_pokemon(settings, pokemon_data, level);
    }
    else if settings.force_legendaries_to_legendaries == settings::LegendRarity::LikelyLegendary && !(pokemon.status == LegendStatus::LegendMega ||pokemon.status == LegendStatus::Legendary){
        if !settings::get_next_seed(0, 200, settings) == 0{
            return get_legendary_pokemon(settings, pokemon_data, level);
        }
    }
    else if settings.force_legendaries_to_legendaries == settings::LegendRarity::NotLegendary && (pokemon.status == LegendStatus::LegendMega ||pokemon.status == LegendStatus::Legendary){
        return get_legendary_pokemon(settings, pokemon_data, level);
    }
    return trainers::scale_pokemon(pokemon.pokemon_id, level, pokemon_data, settings);
}