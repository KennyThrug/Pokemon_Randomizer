use std::fs;

use crate::src::pokemon;
use crate::src::gen3::trainers;
use crate::src::gen3::wild_pokemon;
use crate::src::settings;

pub struct Starter{
    pub treeko: pokemon::PokemonStats,
    pub torchic: pokemon::PokemonStats,
    pub mudkip: pokemon::PokemonStats
}

pub fn randomize_starter_pokemon(settings: &mut settings::Settings,all_stats: &Vec<pokemon::PokemonStats>,file_name_read: String,file_name_read_2: String,file_name_write: String) -> Starter{
    //Settings bullshit to make the preset settings work
    let temp_legend_set = settings.allow_legends_in_wild_pool.clone();
    let temp_scale = settings.scale_wild_pokemon.clone();
    settings.allow_legends_in_wild_pool = settings.allow_starter_legendary.clone();
    settings.scale_wild_pokemon = settings.scale_starter.clone();
    let starters: Starter = 
    if settings.randomize_starter_pokemon{
        Starter{
            treeko: pokemon::get_pokemon_data(pokemon::get_pokemon_from_name(wild_pokemon::get_random_wild_pokemon(settings, all_stats, 5),all_stats),all_stats),
            torchic: pokemon::get_pokemon_data(pokemon::get_pokemon_from_name(wild_pokemon::get_random_wild_pokemon(settings, all_stats, 5),all_stats),all_stats),
            mudkip: pokemon::get_pokemon_data(pokemon::get_pokemon_from_name(wild_pokemon::get_random_wild_pokemon(settings, all_stats, 5),all_stats),all_stats)
        }
    }
    else{
        Starter{
            treeko: pokemon::get_pokemon_data(pokemon::Pokemon::Treecko, all_stats),
            torchic: pokemon::get_pokemon_data(pokemon::Pokemon::Torchic, all_stats),
            mudkip: pokemon::get_pokemon_data(pokemon::Pokemon::Mudkip, all_stats)
        }
    };
    //Setting to file
    let part1 = fs::read_to_string(file_name_read).unwrap();
    let part2 = fs::read_to_string(file_name_read_2).unwrap();


    // // fs::write(file_name_write,format!("{}\n{},\n{},\n{}\n{}",
    // part1,
    // pokemon::format_pokemon_name(starters.treeko.pokemon_name.clone()),
    // pokemon::format_pokemon_name(starters.torchic.pokemon_name.clone()),
    // pokemon::format_pokemon_name(starters.mudkip.pokemon_name.clone()),
    // part2).to_string()).expect("could not write to file starter_choose.c");
    println!("Successfully wrote to file: src/starter_choose.c");
    //Resetting settings so this doesn't mess anything else up
    settings.allow_legends_in_wild_pool = temp_legend_set;
    settings.scale_starter = temp_scale;

    return Starter{
        treeko: trainers::scale_pokemon(starters.treeko.pokemon_id, 34, all_stats, settings),
        torchic: trainers::scale_pokemon(starters.torchic.pokemon_id, 34, all_stats, settings),
        mudkip: trainers::scale_pokemon(starters.mudkip.pokemon_id, 34, all_stats, settings)
    };
}