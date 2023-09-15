use std::fs;
use crate::src::{settings, pokemon::{PokemonStats, get_pokemon_data}};
use json::{self, JsonValue::Null};

use crate::src::pokemon;
use crate::src::gen3::starter_randomization;

use super::emerald::{static_pokemon, special_trainers::{handle_special_trainer, self}};

#[derive(Clone)]
pub struct Trainer{
    pub trainer_name: String,
    pub pokemon: Vec<TrainerPokemon>
}
#[derive(Clone)]
pub struct TrainerPokemon{
    pub iv: i32,
    pub species: pokemon::Pokemon,
    pub level: i32,
    pub moves: Vec<String>,
    pub held_items: String,
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
             pokemon: team});
    }
    //println!("len1:{}",all_trainers.len());
    //Read JSON file and put data in data
    return all_trainers;
}
pub fn shuffle_trainers(settings: &mut settings::Settings,all_stats: &Vec<pokemon::PokemonStats>,trainer_parties_read_filename: String,trainer_parties_write_filename: String,starters: starter_randomization::Starter){
    let mut trainer_data = read_all_trainers(trainer_parties_read_filename,all_stats);
    let (rival_team,wally_team) = create_rival_teams(settings, all_stats);
    //println!("rival team member 1: {}",pokemon::format_pokemon_name(get_pokemon_data(rival_team.pokemon2,all_stats).pokemon_name));
    //println!("Test2");
    static_pokemon::randomize_static_pokemon(settings, all_stats, &rival_team, &wally_team);
    for i in 0..trainer_data.len(){
        if special_trainers::check_if_special_trainer(trainer_data[i].clone()){
            trainer_data[i] = handle_special_trainer(trainer_data[i].clone(), settings, all_stats,&starters,&rival_team,&wally_team);
        }
        else{//Regular Trainers
            trainer_data[i] = get_random_trainer(trainer_data[i].clone(), settings, all_stats)
        }
    }
    //println!("len: {}",trainer_data.len());
    write_trainers_to_file(trainer_parties_write_filename, trainer_data,all_stats);
}

pub fn get_random_trainer(trainer: Trainer, settings: &mut settings::Settings,all_stats: &Vec<pokemon::PokemonStats>) -> Trainer{
    let mut trainer_pkmn: Vec<TrainerPokemon> = Vec::new();
    let mut has_legend = false;
    for cur_pkmn in trainer.pokemon{
        let next_pkmn = get_random_pokemon_for_trainer(trainer.trainer_name.clone(), &cur_pkmn,all_stats,settings,
    if settings.allow_trainer_legendaries == settings::AllowLegendaries::NoLegends ||
                     (settings.allow_trainer_legendaries == settings::AllowLegendaries::OneLegend && has_legend ||
                    settings.allow_trainer_legendaries == settings::AllowLegendaries::AceLegend && has_legend)
                        {false}else{true});
        if get_pokemon_data(next_pkmn.species, all_stats).status == pokemon::LegendStatus::Legendary || get_pokemon_data(next_pkmn.species, all_stats).status == pokemon::LegendStatus::LegendMega{
            has_legend = true;
        }
        trainer_pkmn.push(next_pkmn)
    }
    if settings.allow_trainer_legendaries == settings::AllowLegendaries::AceLegend && has_legend{
        for i in 0..trainer_pkmn.len(){
            if get_pokemon_data(trainer_pkmn[i].species,all_stats).status == pokemon::LegendStatus::Legendary || 
            get_pokemon_data(trainer_pkmn[i].species,all_stats).status == pokemon::LegendStatus::Legendary{
                //Switch Legend with Ace Pokemon
                let temp = trainer_pkmn[i].species;
                let last_pos = trainer_pkmn.len()-1;
                trainer_pkmn[i].species = trainer_pkmn[last_pos].species;
                trainer_pkmn[last_pos].species = temp;
            }
        }
    }
    return Trainer{
        trainer_name: trainer.trainer_name,
        pokemon: trainer_pkmn
    };
}

fn get_random_pokemon_for_trainer(trainer_name: String, pokemon: &TrainerPokemon,pokemon_data: &Vec<pokemon::PokemonStats>,settings: &mut settings::Settings,can_be_legend: bool) -> TrainerPokemon{
    if !settings.randomize_trainer_pokemon{
        return pokemon.clone();
    }
    let rand_val = settings::get_next_seed(0, pokemon_data.len() as i32, settings);
    let new_pokemon = scale_pokemon(pokemon_data[rand_val as usize].clone().pokemon_id,pokemon.level,pokemon_data,settings);
    
    if !can_be_legend && (new_pokemon.status == pokemon::LegendStatus::Legendary || new_pokemon.status == pokemon::LegendStatus::LegendMega){
        return get_random_pokemon_for_trainer(trainer_name, pokemon, pokemon_data, settings,can_be_legend);
    }
    if settings.trainer_legendaries_rare && (new_pokemon.status == pokemon::LegendStatus::Legendary || new_pokemon.status == pokemon::LegendStatus::LegendMega){
        if settings::get_next_seed(0, 20, settings) != 0{
            return get_random_pokemon_for_trainer(trainer_name, pokemon, pokemon_data, settings, can_be_legend)
        }
    }

    TrainerPokemon {
        iv: pokemon.iv,
        species: new_pokemon.pokemon_id.clone(),
        level: pokemon.level,
        moves: create_moveset(settings,new_pokemon.pokemon_id,pokemon.level,pokemon.moves.clone()),
        held_items: create_held_item(settings,new_pokemon.pokemon_id,pokemon.level,pokemon.held_items.clone())
    }
}

pub fn scale_pokemon(pokemon: pokemon::Pokemon,level: i32,all_stats: &Vec<pokemon::PokemonStats>,settings: &mut settings::Settings) -> PokemonStats{
    let stats = get_pokemon_data(pokemon, all_stats).clone();
    if !settings.trainers_scale{
        return stats;
    }
    if get_pokemon_data(pokemon, all_stats).min_level > level as i16{
        //println!("Test Going Down, min lvl: {}.{}",get_pokemon_data(pokemon,all_stats).pokemon_name,level);
        return scale_pokemon(get_pokemon_data(pokemon,all_stats).evolve_from, level, all_stats, settings)
    }
    for cur_evolution in randomize_next_evolutions(get_pokemon_data(pokemon, all_stats).evolve_into.clone(),settings){
        if get_pokemon_data(cur_evolution, all_stats).min_level <= level as i16{
            //println!("Test Going Up: {}, min lvl: {}",get_pokemon_data(cur_evolution, all_stats).pokemon_name,level);
            return scale_pokemon(cur_evolution, level, all_stats, settings);
        }
    }
    return stats;
}
//Warning: Do not use this on a non-copied Vector
fn randomize_next_evolutions(mut next_evolutions: Vec<pokemon::Pokemon>,settings: &mut settings::Settings) -> Vec<pokemon::Pokemon>{
    let mut return_values: Vec<pokemon::Pokemon> = Vec::new();
    while next_evolutions.len() > 0 {
        return_values.push(next_evolutions.remove(settings::get_next_seed(0, next_evolutions.len() as i32, settings) as usize));
    }
    return return_values;
}

pub fn create_moveset(settings: &mut settings::Settings,pokemon: pokemon::Pokemon,level: i32,old_moveset: Vec<String>) -> Vec<String>{
    //Placeholder for now, functionality will be added later
    old_moveset
}
pub fn create_held_item(settings: &mut settings::Settings,pokemon: pokemon::Pokemon,level: i32,old_item: String) -> String{
    //Placeholder for now, functionality will be added later
    old_item
}

fn create_rival_teams(settings: &mut settings::Settings,pokemon_data: &Vec<pokemon::PokemonStats>) -> (MayBrendanTeam,WallyTeam){
    let mut fake_rival = Trainer{
        trainer_name: "rival".to_string(),
        pokemon: vec![
            TrainerPokemon{
                iv: 150,
                level: 31,
                species: pokemon::Pokemon::Tropius,
                moves: Vec::new(),
                held_items: "".to_string()
            },
            TrainerPokemon{
                iv: 150,
                level: 32,
                species: pokemon::Pokemon::Ludicolo,
                moves: Vec::new(),
                held_items: "".to_string()
            },
            TrainerPokemon{
                iv: 150,
                level: 32,
                species: pokemon::Pokemon::Slugma,
                moves: Vec::new(),
                held_items: "".to_string()
            }
        ]
    };
    fake_rival = get_random_trainer(fake_rival,settings,pokemon_data);
    let may_team = MayBrendanTeam{
        pokemon2: fake_rival.pokemon[0].species,
        pokemon3: fake_rival.pokemon[1].species,
        pokemon4: fake_rival.pokemon[2].species,
    };
    let mut fake_wally = Trainer{
        trainer_name: "wally".to_string(),
        pokemon: vec![
            TrainerPokemon{
                iv: 150,
                level: 44,
                species: pokemon::Pokemon::Altaria,
                moves: Vec::new(),
                held_items: "".to_string()
            },
            TrainerPokemon{
                    iv: 150,
                    level: 43,
                    species: pokemon::Pokemon::Delcatty,
                    moves: Vec::new(),
                    held_items: "".to_string()
                },
            TrainerPokemon{
                iv: 150,
                level: 44,
                species: pokemon::Pokemon::Roselia,
                moves: Vec::new(),
                held_items: "".to_string()
            },
            TrainerPokemon{
                iv: 150,
                level: 41,
                species: pokemon::Pokemon::Magneton,
                moves: Vec::new(),
                held_items: "".to_string()
            },
            TrainerPokemon{
                iv: 150,
                level: 45,
                species: pokemon::Pokemon::Gardevoir,
                moves: Vec::new(),
                held_items: "".to_string()
            }
        ]
    };
    fake_wally = get_random_trainer(fake_wally, settings, pokemon_data);
    let wally_team = WallyTeam{
        ralt_substitute: fake_wally.pokemon[4].species,
        pokemon2: fake_wally.pokemon[0].species,
        pokemon3: fake_wally.pokemon[1].species,
        pokemon4: fake_wally.pokemon[2].species,
        pokemon5: fake_wally.pokemon[3].species
    };
    (may_team,wally_team)
}

pub struct MayBrendanTeam{
    pub pokemon2: pokemon::Pokemon,
    pub pokemon3: pokemon::Pokemon,
    pub pokemon4: pokemon::Pokemon,
}
pub struct WallyTeam{
    pub ralt_substitute: pokemon::Pokemon,
    pub pokemon2: pokemon::Pokemon,
    pub pokemon3: pokemon::Pokemon,
    pub pokemon4: pokemon::Pokemon,
    pub pokemon5: pokemon::Pokemon,
}