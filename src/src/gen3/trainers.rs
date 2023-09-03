use std::fs;
use crate::src::{settings, pokemon::PokemonStats};
use json::{self, JsonValue::Null};

use crate::src::pokemon;

pub struct Trainer{
    pub trainer_name: String,
    pub pokemon: Vec<TrainerPokemon>
}
pub struct TrainerPokemon{
    iv: i32,
    species: pokemon::Pokemon,
    level: i32,
    moves: Vec<String>,
    held_items: String,
}
pub fn write_trainers_to_file(filename:String,trainers: Vec<Trainer>,all_stats: &Vec<pokemon::PokemonStats>){
    let mut file_text = "".to_string();
    for cur_trainer in trainers{
        //println!("{}",cur_trainer.trainer_name);
        let has_held_item = cur_trainer.pokemon[0].held_items.len() != 0;
        let has_custom_moves = cur_trainer.pokemon[0].moves.len() != 0;
        //println!("item: {} moves: {}",has_held_item,has_custom_moves);
        if has_custom_moves && has_held_item{
            file_text += &format!("static const struct TrainerMonItemCustomMoves {} [] = {{\n",cur_trainer.trainer_name).to_string();
            for cur_pkmn in cur_trainer.pokemon{
                file_text += &format!("{{\n.iv = {},\n.lvl = {},\n.species = {},\n.heldItem = {},\n.moves = {{{},{},{},{}}}\n}},",
            cur_pkmn.iv,cur_pkmn.level,pokemon::format_pokemon_name(pokemon::get_pokemon_data(cur_pkmn.species,all_stats).pokemon_name),cur_pkmn.held_items,
            cur_pkmn.moves[0],cur_pkmn.moves[1],cur_pkmn.moves[2],cur_pkmn.moves[3])
            }
        }

        else if has_custom_moves && !has_held_item{
            file_text += &format!("static const struct TrainerMonNoItemCustomMoves {} [] = {{\n",cur_trainer.trainer_name).to_string();
            for cur_pkmn in cur_trainer.pokemon{
                file_text += &format!("{{\n.iv = {},\n.lvl = {},\n.species = {},\n.moves = {{{},{},{},{}}}\n}},",
            cur_pkmn.iv,cur_pkmn.level,pokemon::format_pokemon_name(pokemon::get_pokemon_data(cur_pkmn.species, all_stats).pokemon_name),
            cur_pkmn.moves[0],cur_pkmn.moves[1],cur_pkmn.moves[2],cur_pkmn.moves[3])
            }
        }

        else if has_held_item && !has_custom_moves{
            file_text += &format!("static const struct TrainerMonItemDefaultMoves {} [] = {{\n",cur_trainer.trainer_name).to_string();
            for cur_pkmn in cur_trainer.pokemon{
                file_text += &format!("{{\n.iv = {},\n.lvl = {},\n.species = {},\n.heldItem = {}}},",
            cur_pkmn.iv,cur_pkmn.level,pokemon::format_pokemon_name(pokemon::get_pokemon_data(cur_pkmn.species, all_stats).pokemon_name),cur_pkmn.held_items)
            }
        }

        else{
            file_text += &format!("static const struct TrainerMonNoItemDefaultMoves {} [] = {{\n",cur_trainer.trainer_name).to_string();
            for cur_pkmn in cur_trainer.pokemon{
                file_text += &format!("{{\n.iv = {},\n.lvl = {},\n.species = {}}},",
            cur_pkmn.iv,cur_pkmn.level,pokemon::format_pokemon_name(pokemon::get_pokemon_data(cur_pkmn.species, all_stats).pokemon_name))
            }
        }
        file_text += "\n};\n\n"
    }
    //println!("text: {}",file_text);
    fs::write(filename,file_text).expect("Could not write trainer data");
}

fn read_all_trainers(filename: String,all_stats: &Vec<pokemon::PokemonStats>) -> Vec<Trainer>{
    let data = fs::read_to_string(filename).expect("unable to read file");
    let data_json = json::parse(&data).unwrap();
    let mut all_trainers: Vec<Trainer> = Vec::new();
    for i in 0..1000{
        if data_json["trainers"][i] == Null{break;}
        let mut team : Vec<TrainerPokemon> = Vec::new();
        for j in 0..6{
            if(data_json["trainers"][i]["party"][j] == Null){break;}
            let mut moves: Vec<String> = Vec::new();
            if data_json["trainers"][i]["party"][j]["moves"] != Null{
                for k in 1..5{
                    moves.push(data_json["trainers"][i]["party"][j]["moves"][format!("move{}",k)].to_string());
                }
            }

            team.push(TrainerPokemon {
                 iv: data_json["trainers"][i]["party"][j]["iv"].as_i32().unwrap(),
                 species: pokemon::get_pokemon_from_name(data_json["trainers"][i]["party"][j]["species"].to_string(), all_stats),
                 level: data_json["trainers"][i]["party"][j]["lvl"].as_i32().unwrap(),
                 moves:{
                    moves
                 },
                 held_items: if data_json["trainers"][i]["party"][j]["heldItem"] == Null{
                    "".to_string()
                 }
                 else{
                    data_json["trainers"][i]["party"][j]["heldItem"].to_string()
                 }
                }
            )
        }
        all_trainers.push(Trainer { 
            trainer_name: data_json["trainers"][i]["name"].to_string(),
             pokemon: team });
    }
    //println!("len1:{}",all_trainers.len());
    //Read JSON file and put data in data
    return all_trainers;
}
pub fn shuffle_trainers(settings: &mut settings::Settings,all_stats: &Vec<pokemon::PokemonStats>){
    let trainer_data = read_all_trainers("data/emerald/trainer_parties.json".to_string(),all_stats);
    //println!("len: {}",trainer_data.len());
    write_trainers_to_file("decomp/pokeemerald-expansion/src/data/trainer_parties.h".to_string(), trainer_data,all_stats);
}