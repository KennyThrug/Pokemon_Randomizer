use crate::src::{settings, pokemon};
use std::fs;

pub fn randomize_birch_pokemon(settings: &mut settings::Settings, all_stats: &Vec<pokemon::PokemonStats>){
    let pt1 = fs::read_to_string("data/gen3/emerald/main_menu_pt1.c").expect("Cannot Read main_menu_pt1.c");
    let pt2 = fs::read_to_string("data/gen3/emerald/main_menu_pt2.c").expect("Cannot Read main_menu_pt2.c");
    let pt3 = fs::read_to_string("data/gen3/emerald/main_menu_pt3.c").expect("Cannot Read main_menu_pt3.c");
    fs::write("decomp/pokeemerald-expansion/src/main_menu.c", format!("{part1}{pkmn}{part2}{pkmn}{part3}"
    ,part1=pt1
    ,part2=pt2
    ,part3=pt3
    ,pkmn = pokemon::format_pokemon_name(all_stats[settings::get_next_seed(0, all_stats.len() as i32-1, settings) as usize].pokemon_name.clone())
    )).expect("Could not Write to main_menu.c");
}