use std::fs;
use crate::settings;
use json;

use crate::pokemon::{self, Pokemon};

pub struct Trainer<'a>{
    pub trainer_name: &'a str,
    pub pokemon: Vec<TrainerPokemon<'a>>
}
pub struct TrainerPokemon<'a>{
    iv: i32,
    species: &'a Pokemon<'a>,
    level: i32,
    moves: Vec<String>,
    held_items: &'a str,
}
pub fn write_trainers_to_file(filename:String,trainers: Vec<Trainer>){
    let mut file_text = "".to_string();
    for cur_trainer in trainers{
        let has_held_item = cur_trainer.pokemon[0].held_items.len() == 0;
        let has_custom_moves = cur_trainer.pokemon[0].moves.len() == 0;
        if has_custom_moves && has_held_item{
            file_text += &format!("static const struct TrainerMonItemCustomMoves {} [] = {{\n",cur_trainer.trainer_name).to_string();
            for cur_pkmn in cur_trainer.pokemon{
                file_text += &format!("{{\n.iv = {},\n.lvl = {},\n.species = {},\n.heldItem = {},\n.moves = {{{},{},{},{}}}\n}},",
            cur_pkmn.iv,cur_pkmn.level,pokemon::convert_name(&cur_pkmn.species),cur_pkmn.held_items,
            cur_pkmn.moves[0],cur_pkmn.moves[1],cur_pkmn.moves[2],cur_pkmn.moves[3])
            }
        }

        else if has_custom_moves && !has_held_item{
            file_text += &format!("static const struct TrainerMonNoItemCustomMoves {} [] = {{\n",cur_trainer.trainer_name).to_string();
            for cur_pkmn in cur_trainer.pokemon{
                file_text += &format!("{{\n.iv = {},\n.lvl = {},\n.species = {},\n.moves = {{{},{},{},{}}}\n}},",
            cur_pkmn.iv,cur_pkmn.level,pokemon::convert_name(&cur_pkmn.species),
            cur_pkmn.moves[0],cur_pkmn.moves[1],cur_pkmn.moves[2],cur_pkmn.moves[3])
            }
        }

        else if has_held_item && !has_custom_moves{
            file_text += &format!("static const struct TrainerMonItemDefaultMoves {} [] = {{\n",cur_trainer.trainer_name).to_string();
            for cur_pkmn in cur_trainer.pokemon{
                file_text += &format!("{{\n.iv = {},\n.lvl = {},\n.species = {},\n.heldItem = {}}},",
            cur_pkmn.iv,cur_pkmn.level,pokemon::convert_name(&cur_pkmn.species),cur_pkmn.held_items)
            }
        }

        else{
            file_text += &format!("static const struct TrainerMonNoItemDefaultMoves {} [] = {{\n",cur_trainer.trainer_name).to_string();
            for cur_pkmn in cur_trainer.pokemon{
                file_text += &format!("{{\n.iv = {},\n.lvl = {},\n.species = {}}},",
            cur_pkmn.iv,cur_pkmn.level,pokemon::convert_name(&cur_pkmn.species))
            }
        }
        file_text += "\n}};\n\n"
    }
    fs::write(filename,file_text).expect("Could not write trainer data");
}

fn read_all_trainers(filename: String){
    let data = fs::read_to_string("data/emerald/trainer_parties.json").expect("unable to read file");
    let data_json = json::parse(&data).unwrap();
    println!("{}",data_json["sParty_Sawyer1"]);
    //Read JSON file and put data in data
    
}
pub fn shuffle_trainers(settings: &mut settings::Settings){
    read_all_trainers("data/emerald/trainer_parties.json".to_string());
}