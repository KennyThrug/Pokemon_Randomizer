use crate::settings;
use crate::pokemon;
use std::fs;

const NUMBER_OF_ROUTES: usize = 124;
pub fn randomize_wild_pokemon(settings: &mut settings::Settings,pokemon_data: &Vec<pokemon::PokemonStats>){
    let data = fs::read_to_string("decomp/pokeemerald-expansion/src/data/wild_encounters.json").expect("unable to read file");
    let mut parsed_data = json::parse(&data).unwrap();
    for i in 0..NUMBER_OF_ROUTES{
        for place in ["land_mons","water_mons","fishing_mons"]{
            for j in 0..30{ //Land Mons
                if parsed_data["wild_encounter_groups"][0]["encounters"]
                [i][place]["mons"][j]["species"] == json::JsonValue::Null{
                    break;
                }
                parsed_data["wild_encounter_groups"][0]["encounters"]
                [i][place]["mons"][j]["species"] = get_random_wild_pokemon(settings,pokemon_data).into();
            }
        }
    }
    //write to file
    fs::write("decomp/pokeemerald-expansion/src/data/wild_encounters.json",
    parsed_data.to_string()).expect("couldn't write to file");
}

//Gets a pokemon, checks if it can be put there, and returns the value
fn get_random_wild_pokemon(settings: &mut settings::Settings,pokemon_data: &Vec<pokemon::PokemonStats>) -> String{
    //Make Random Pokemon
    let rand_val = settings::get_next_seed(0, pokemon_data.len() as i32, settings);
    let pokemon = pokemon_data[rand_val as usize].clone();
    //Check if it is a valid pokemon
    if !settings.allow_pokemon_past_generation && pokemon.generation > 3{
        return get_random_wild_pokemon(settings,pokemon_data);
    }
    if !settings.allow_legends_in_wild_pool && pokemon.status == pokemon::LegendStatus::Legendary{
        return get_random_wild_pokemon(settings,pokemon_data);
    }
    if !settings.allow_megas_in_wild_pool && pokemon.status == pokemon::LegendStatus::Mega{
        return get_random_wild_pokemon(settings,pokemon_data);
    }

    //Return if it passes all checks
    return pokemon::format_pokemon_name(pokemon.pokemon_name);
}