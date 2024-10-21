use std::fs;
use crate::src::{settings, pokemon::{PokemonStats, get_pokemon_data}};
use json::{self, JsonValue::Null};

use crate::src::pokemon;
use crate::src::gen3::starter_randomization;

use super::emerald::{static_pokemon, special_trainers::{handle_special_trainer, self}};

#[derive(Clone)]
pub struct Trainer{
    pub trainer_full_name: String, //The Trainer name in code, such as TRAINER_GRUNT_SEAFLOOR_CAVERN_1
    pub trainer_name: String, //Trainer name in game, such as Grunt
    pub class: String,
    pub pic: String,
    pub gender: String,
    pub music: String,
    pub double_battle: String,
    pub ai: String,
    pub portrait: String,
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
        // let has_held_item = cur_trainer.pokemon[0].held_items.len() != 0;
        // let has_custom_moves = cur_trainer.pokemon[0].moves.len() != 0;
        //println!("item: {} moves: {}",has_held_item,has_custom_moves);

        //TODO format this to be in the same format as src/data/trainers.party

        let mut all_pokemon : String = "".to_string();
        for cur_pokemon in cur_trainer.pokemon{
            all_pokemon.push_str(get_pokemon(cur_pokemon,all_stats).as_str());
        }

        // println!("This: {}", all_pokemon);
        file_text += format!("{}",cur_trainer.trainer_full_name).as_str();
        file_text += format!("\nName: {}",cur_trainer.trainer_name).as_str();
        file_text += format!("\nClass: {}",cur_trainer.class).as_str();
        file_text += format!("\nPic: {}",cur_trainer.pic).as_str();
        file_text += format!("\nGender: {}",cur_trainer.gender).as_str();
        file_text += format!("\nMusic: {}",cur_trainer.music).as_str();
        file_text += format!("\nDouble Battle: {}",cur_trainer.double_battle).as_str();
        if cur_trainer.ai != ""{
            file_text += format!("\nAI: {}",cur_trainer.ai).as_str();
        }
        if cur_trainer.portrait != ""{
            file_text += format!("\nPortrait: {}",cur_trainer.portrait).as_str();
        }
        file_text += format!("\n\n{}",all_pokemon).as_str();
    }
    //println!("text: {}",file_text);
    fs::write(filename,file_text).expect("Could not write trainer data");
}

fn read_all_trainers(filename: String,all_stats: &Vec<pokemon::PokemonStats>) -> Vec<Trainer>{
    let data = fs::read_to_string(filename).expect("unable to read file");
    let mut all_trainers: Vec<Trainer> = Vec::new();

    let mut cur_trainer = Trainer{
        trainer_full_name: "".to_string(),
        trainer_name: "".to_string(),
        class: "".to_string(),
        pic: "".to_string(),
        gender: "".to_string(),
        music: "".to_string(),
        double_battle: "".to_string(),
        ai: "".to_string(),
        portrait: "".to_string(),
        pokemon: Vec::new()
    };
    let mut cur_pokemon = TrainerPokemon{
        iv : 0,
        species : pokemon::Pokemon::None,
        level : 5,
        moves : Vec::new(),
        held_items: "".to_string()
    };
    for i in data.lines(){
        //Split against :
        let mut cat : Vec<&str> = i.split(':').collect();
        let mut second_half_temp = "".to_string();
        for i in 0..cat.len(){
            cat[i] = cat[i].trim();
        }
        if i != "" && i.to_string().chars().nth(0).unwrap() == '='{
            if cur_trainer.trainer_full_name != ""{
                all_trainers.push(cur_trainer);
            }
            cur_trainer = Trainer{
                trainer_full_name: "".to_string(),
                trainer_name: "".to_string(),
                class: "".to_string(),
                pic: "".to_string(),
                gender: "".to_string(),
                music: "".to_string(),
                double_battle: "".to_string(),
                ai: "".to_string(),
                portrait: "".to_string(),
                pokemon: Vec::new()
            };
            cur_trainer.trainer_full_name = i.to_string()
        }
        else if cat[0] == "Name"{
            cur_trainer.trainer_name = cat[1].to_string();
        }
        else if cat[0] == "Class"{
            cur_trainer.class = cat[1].to_string();
        }
        else if cat[0] == "Pic"{
            cur_trainer.pic = cat[1].to_string();
        }
        else if cat[0] == "Gender"{
            cur_trainer.gender = cat[1].to_string();
        }
        else if cat[0] == "Music"{
            cur_trainer.music = cat[1].to_string();
        }
        else if cat[0] == "Double Battle"{
            cur_trainer.double_battle = cat[1].to_string();
        }
        else if cat[0] == "AI"{
            cur_trainer.ai = cat[1].to_string();
        }
        else if cat[0] == "Mugshot"{
            cur_trainer.ai.push_str(format!("\n{}",i).as_str());
        }
        else if i == ""{
            //Add pokemon to list of pokemon if needed
            if cur_pokemon.species != pokemon::Pokemon::None{
                cur_trainer.pokemon.push(cur_pokemon);
            }
            //Start new Pokemon
            cur_pokemon = TrainerPokemon{
                iv : 0,
                species : pokemon::Pokemon::None,
                level : 5,
                moves : Vec::new(),
                held_items: "".to_string()
            };
        }
        //Get Pokemon and stuff
        else if cur_pokemon.species == pokemon::Pokemon::None{
            let cur_pokemon_name : Vec<&str> = i.split('@').collect();
            if cur_pokemon_name.len() > 1{
                println!("Name: {} Other: {}",cur_pokemon_name[0],cur_pokemon_name[1]);
                cur_pokemon.held_items = cur_pokemon_name[1].to_string();
            }
            cur_pokemon.species = pokemon::get_pokemon_from_name(cur_pokemon_name[0].trim().to_string(),all_stats)
        }
        else if cat[0] == "Level"{
            cur_pokemon.level = i.to_string()[7..].parse::<i32>().unwrap();
        }
        else if cat[0] == "IVs"{
            cur_pokemon.iv = 0;
        }
        else if i != "" && i.to_string().chars().nth(0).unwrap() == '-'{
            cur_pokemon.moves.push(i.to_string()[2..].to_string());
        }
    }
    //Read JSON file and put data in data
    return all_trainers;
}

pub fn get_pokemon(pokemon: TrainerPokemon,all_stats: &Vec<pokemon::PokemonStats>) -> String{
    let mut formatted_moves : String = "".to_string();
    for i in pokemon.moves{
        formatted_moves.push_str(format!("- {}\n",i).as_str());
    }
    return format!("{pkmn_name}
Level: {level}
IVs: {iv} HP / {iv} Atk / {iv} Def / {iv} SpA / {iv} SpD / {iv} Spe
{moves}
",pkmn_name=pokemon::pokemon_to_formatted_name(pokemon.species,all_stats),level=pokemon.level,
iv=pokemon.iv,moves=formatted_moves);
}

pub fn shuffle_trainers(settings: &mut settings::Settings,all_stats: &Vec<pokemon::PokemonStats>,trainer_parties_read_filename: String,trainer_parties_write_filename: String,starters: starter_randomization::Starter){
    let mut trainer_data = read_all_trainers(trainer_parties_read_filename,all_stats);
    let (rival_team,wally_team) = create_rival_teams(settings, all_stats);
    static_pokemon::randomize_static_pokemon(settings, all_stats, &rival_team, &wally_team);
    let gym_types = special_trainers::randomize_gym_types(12,settings);
    for i in 0..trainer_data.len(){
        if special_trainers::check_if_special_trainer(trainer_data[i].clone()){
            trainer_data[i] = handle_special_trainer(trainer_data[i].clone(), settings, all_stats,&starters,&rival_team,&wally_team,gym_types.clone());
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
            println!("LL: {}",trainer_pkmn[i].species as i32);
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
        trainer_full_name: trainer.trainer_full_name,
        trainer_name: trainer.trainer_name,
        class: trainer.class,
        pic: trainer.pic,
        gender: trainer.gender,
        music: trainer.music,
        double_battle: trainer.double_battle,
        ai: trainer.ai,
        portrait: "".to_string(),
        pokemon: trainer_pkmn
    };
}

pub fn get_random_pokemon_for_trainer(trainer_name: String, pokemon: &TrainerPokemon,pokemon_data: &Vec<pokemon::PokemonStats>,settings: &mut settings::Settings,can_be_legend: bool) -> TrainerPokemon{
    if !settings.randomize_trainer_pokemon{
        return pokemon.clone();
    }
    let rand_val = settings::get_next_seed(0, (pokemon_data.len() as i32), settings);
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
        held_items: create_held_item(settings,new_pokemon.pokemon_id,pokemon.level,pokemon.held_items.clone()),
        // gimmick: None,
        // gimmick_string: "".to_string()
    }
}

pub fn scale_pokemon(pokemon: pokemon::Pokemon,level: i32,all_stats: &Vec<pokemon::PokemonStats>,settings: &mut settings::Settings) -> PokemonStats{
    let stats = get_pokemon_data(pokemon, all_stats).clone();
    if !settings.trainers_scale{
        return stats;
    }
    if get_pokemon_data(pokemon, all_stats).min_level > level as i16{
        if get_pokemon_data(pokemon,all_stats).evolve_from == pokemon::Pokemon::None{
            // println!("Failsafe triggered 1, level = {}",level);
            return get_pokemon_data(pokemon,all_stats);
        }
        return scale_pokemon(get_pokemon_data(pokemon,all_stats).evolve_from, level, all_stats, settings);
    }
    for cur_evolution in randomize_next_evolutions(get_pokemon_data(pokemon, all_stats).evolve_into.clone(),settings){
        if cur_evolution == pokemon::Pokemon::None{
            // println!("Failsafe triggered 2, level = {}",level);
            return get_pokemon_data(pokemon,all_stats);
        }
        if get_pokemon_data(cur_evolution, all_stats).min_level <= level as i16{
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
    return Vec::new();
}
pub fn create_held_item(settings: &mut settings::Settings,pokemon: pokemon::Pokemon,level: i32,old_item: String) -> String{
    //Placeholder for now, functionality will be added later
    old_item
}

fn create_rival_teams(settings: &mut settings::Settings,pokemon_data: &Vec<pokemon::PokemonStats>) -> (MayBrendanTeam,WallyTeam){
    let mut fake_rival = Trainer{
        trainer_full_name: "TRAINER_RIVAL".to_string(),
        trainer_name: "rival".to_string(),
        class: "".to_string(),
        pic: "".to_string(),
        gender: "".to_string(),
        music: "".to_string(),
        double_battle: "".to_string(),
        ai: "".to_string(),
        portrait: "".to_string(),
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
        trainer_full_name: "TRAINER_WALLY".to_string(),
        trainer_name: "wally".to_string(),
        class: "".to_string(),
        pic: "".to_string(),
        gender: "".to_string(),
        music: "".to_string(),
        double_battle: "".to_string(),
        ai: "".to_string(),
        portrait: "".to_string(),
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