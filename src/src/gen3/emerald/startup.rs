use std::process::Command;

use crate::src::gen3::evolution_changes::setup_evolution_fixes;
use crate::src::gen3::item_randomization;
use crate::src::pokemon::read_all_pokemon;
use crate::src::settings;
use crate::src::gen3::wild_pokemon;
use crate::src::gen3::trainers;
use crate::src::gen3::emerald::other;
use crate::src::gen3::starter_randomization;
use std::env;
//File that contains the functions that will be called by the launcher

//Randomizes every pokemon in the game according to json file
//filename should point to a Json file generated by the launcher with all the settings & Seed
pub fn randomize_pokemon(settings : &mut settings::Settings){
    //let mut settings = create_sample_settings();
    println!("Starting Rom Randomization Process");
    let pkmn_data = read_all_pokemon();
    wild_pokemon::randomize_wild_pokemon(settings,&pkmn_data);
    let starters = starter_randomization::randomize_starter_pokemon(settings, &pkmn_data,"data/emerald/starter_choose.c".to_string(),"data/emerald/starter_choose_2.c".to_string(),"decomp/pokeemerald-expansion/src/starter_choose.c".to_string());
    trainers::shuffle_trainers(settings,&pkmn_data,"data/emerald/trainer_parties.json".to_string(),"decomp/pokeemerald-expansion/src/data/trainer_parties.h".to_string(),starters);
    other::randomize_birch_pokemon(settings, &pkmn_data);
    setup_evolution_fixes(settings);
    item_randomization::randomize_items();
    build_rom(settings);
}

pub fn build_rom(settings : &mut settings::Settings){
    let path = env::current_dir().unwrap();
    Command::new("sh")
    .arg("-C")
    .arg(format!("{}/src/src/gen3/emerald/make_rom.sh",path.display()))
    .arg(settings.seed.clone())
    .spawn()
    .expect("sh command failed to start");
}