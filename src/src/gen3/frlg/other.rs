use crate::src::{settings, pokemon};
use std::fs;

pub fn randomize_oak_pokemon(settings: &mut settings::Settings, all_stats: &Vec<pokemon::PokemonStats>){
    let pt1 = fs::read_to_string("data/gen3/firered/oak_speech.c").expect("Cannot Read main_menu_pt1.c");
    let pt2 = fs::read_to_string("data/gen3/firered/oak_speech2.c").expect("Cannot Read main_menu_pt2.c");
    fs::write("decomp/pokefirered/src/oak_speech.c", format!("{part1}{pkmn}{part2}"
    ,part1=pt1
    ,part2=pt2
    ,pkmn = pokemon::format_pokemon_name(all_stats[settings::get_next_seed(0, all_stats.len() as i32-1, settings) as usize].pokemon_name.clone())
    )).expect("Could not Write to oak_speech.c");
}