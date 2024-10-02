use crate::src::{settings, pokemon};
use std::fs;

pub fn randomize_birch_pokemon(settings: &mut settings::Settings, all_stats: &Vec<pokemon::PokemonStats>){
    let pt1 = fs::read_to_string("data/emerald/main_menu_pt1.c").expect("Cannot Read main_menu_pt1.c");
    let pt2 = fs::read_to_string("data/emerald/main_menu_pt2.c").expect("Cannot Read main_menu_pt2.c");
    // fs::write("decomp/pokeemerald-expansion/src/main_menu.c", format!("{}{}{}",pt1,
    // pokemon::format_pokemon_name(all_stats[settings::get_next_seed(0, all_stats.len() as i32-1, settings) as usize].pokemon_name.clone())
    // ,pt2)).expect("Could not Write to main_menu.c");
}