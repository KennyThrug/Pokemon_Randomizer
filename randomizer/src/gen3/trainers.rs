use std::fs;

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
pub fn get_unaltered_trainers() -> Vec<Trainer<'static>>{
    let all_trainers: Vec<Trainer> = vec![
        Trainer {trainer_name: "sParty_Sawyer1",pokemon: vec![TrainerPokemon{
            iv: 0,
            level: 21,
            species: pokemon::get_species("Geodude"),
            moves: Vec::new(),
            held_items: ""
    }]},
    Trainer {trainer_name: "sParty_GruntAquaHideout1",pokemon: vec![TrainerPokemon{
            iv: 0,
            level: 32,
            species: pokemon::get_species("Poochyena"),
            moves: Vec::new(),
            held_items: ""
    }]},
    Trainer {trainer_name: "sParty_GruntAquaHideout2",pokemon: vec![TrainerPokemon{
            iv: 0,
            level: 31,
            species: pokemon::get_species("Zubat"),
            moves: Vec::new(),
            held_items: ""
    },
    TrainerPokemon{
        iv: 0,
        level: 31,
        species: pokemon::get_species("Carvanha"),
        moves: Vec::new(),
        held_items: ""
    }]},
    Trainer {trainer_name: "sParty_GruntAquaHideout4",pokemon: vec![TrainerPokemon{
            iv: 0,
            level: 32,
            species: pokemon::get_species("Carvahna"),
            moves: Vec::new(),
            held_items: ""
    }]},
    Trainer {trainer_name: "sParty_GruntSeafloorCavern1",pokemon: vec![TrainerPokemon{
            iv: 0,
            level: 36,
            species: pokemon::get_species("Poochyena"),
            moves: Vec::new(),
            held_items: ""
    }]},
    Trainer {trainer_name: "sParty_GruntSeafloorCavern2",pokemon: vec![TrainerPokemon{
            iv: 0,
            level: 36,
            species: pokemon::get_species("Carvahna"),
            moves: Vec::new(),
            held_items: ""
    }]},
    Trainer {trainer_name: "sParty_GruntSeafloorCavern3",pokemon: vec![TrainerPokemon{
            iv: 0,
            level: 36,
            species: pokemon::get_species("Zubat"),
            moves: Vec::new(),
            held_items: ""
    }]}
    ];
    all_trainers
}

fn read_all_trainers(filename: String){
    let data = fs::read_to_string("decomp/pokeemerald-expansion/src/data/trainer_parties.h").expect("unable to read file");
    let mut char_data = data.chars();
    let mut results: Vec<Trainer> = Vec::new();
    let mut cur_num_trainer = 0;
    let mut cur_num_pokemon = 0;
    let mut cur_word: String = "".to_string();
    let mut move_cur = 0;
    let mut iv_cur = false; let mut lvl_cur = false; let mut species_cur = false; let mut item_cur = false;
    let cur_trainer = Trainer {
        trainer_name: "",
        pokemon: Vec::new()
    };
    for i in 0..data.len(){
        if char_data.nth(i).unwrap() == ' ' || char_data.nth(i).unwrap() == '.'{
            cur_word = "".to_string();
        }
        else if char_data.nth(i).unwrap() == ',' {
            if iv_cur{
                results[cur_num_trainer].pokemon.push(TrainerPokemon{
                    iv: str::parse(&cur_word).unwrap(),
                    species: &Pokemon { name:"", type1: pokemon::Type::None, type2: pokemon::Type::None, evo_stage: 0, total_stages: 0, status: pokemon::LegendStatus::Standard, generation: 0 },
                    level: 0,
                    moves: Vec::new(),
                    held_items: ""
                });
            }
            else if lvl_cur{
                results[cur_num_trainer].pokemon[cur_num_pokemon].level = str::parse(&cur_word).unwrap();
            }
            else if species_cur{
                results[cur_num_trainer].pokemon[cur_num_pokemon].species = pokemon::get_species(&cur_word);
            }
            else if item_cur{
                results[cur_num_trainer].pokemon[cur_num_pokemon].held_items = &cur_word;
            }
            else if move_cur != 0{
                results[cur_num_trainer].pokemon[cur_num_pokemon].moves[4 - move_cur] = cur_word.clone();
                move_cur -= 1;
            }
            else if &cur_word == "iv"{
                iv_cur = true;
            }
            else if &cur_word == "lvl"{
                lvl_cur = true;
            }
            else if &cur_word == "species"{
                species_cur = true;
            }
            else if &cur_word == "held_item"{
                item_cur = true;
            }
            else if &cur_word == "moves"{
                move_cur = 4;
            }
            cur_word = "".to_string();
        }
        else if char_data.nth(i).unwrap() == '['{

        }
        else if char_data.nth(i).unwrap() == ';'{
            cur_num_pokemon = 0;
        }
        else{
            cur_word = cur_word + &char_data.nth(i).unwrap().to_string();
        }
    }
}