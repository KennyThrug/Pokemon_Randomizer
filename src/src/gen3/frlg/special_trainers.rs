use crate::src::{gen3::{trainers::{Trainer, self, TrainerPokemon, scale_pokemon}, starter_randomization}, settings, pokemon};
use std::collections::VecDeque;
use crate::src::gen3::wild_pokemon;

//Rival Names
const BLUE: usize  = 0;

//Pokemon (assuming he picks bulbasaur)
const BLUE_PIDGEOT: usize   = 0;
const BLUE_ALAKAZAM: usize   = 1;
const BLUE_RHYDON: usize   = 2;
const BLUE_GYARADOS: usize   = 3;
const BLUE_ARCANINE: usize   = 4;
const BLUE_EXECUTOR: usize = 5;
const BLUE_STARTER: usize   = 6;
const BLUE_RATTICATE: usize   = 7;
const BLUE_HERACROSS: usize   = 8;//Post Game
const BLUE_TYRANITAR: usize   = 9;//Post Game

pub fn get_rivals(settings: &mut settings::Settings, pokemon_data: &Vec<pokemon::PokemonStats>) -> Vec<Vec<pokemon::Pokemon>>{
    let mut all : Vec<Vec<pokemon::Pokemon>> = Vec::new();
    all.push(Vec::new());
    //Fake Rival for each pokemon at its highest possible level
    let mut fake_rival = Trainer{
        trainer_full_name: "TRAINER_RIVAL".to_string(),
        trainer_name: "rival".to_string(),
        class: "".to_string(),
        pic: "".to_string(),
        gender: "".to_string(),
        music: "".to_string(),
        items: "".to_string(),
        double_battle: "".to_string(),
        ai: "".to_string(),
        portrait: "".to_string(),
        pokemon: vec![
            TrainerPokemon{
                iv: 150,
                level: 59,
                extra_scripts: "".to_string(),
                species: pokemon::Pokemon::Pidgeot,
                moves: Vec::new(),
                held_items: "".to_string()
            },
            TrainerPokemon{
                iv: 150,
                level: 73,
                extra_scripts: "".to_string(),
                species: pokemon::Pokemon::Alakazam,
                moves: Vec::new(),
                held_items: "".to_string()
            },
            TrainerPokemon{
                iv: 150,
                level: 59,
                extra_scripts: "".to_string(),
                species: pokemon::Pokemon::Rhydon,
                moves: Vec::new(),
                held_items: "".to_string()
            },
            TrainerPokemon{
                iv: 150,
                level: 73,
                extra_scripts: "".to_string(),
                species: pokemon::Pokemon::Gyarados,
                moves: Vec::new(),
                held_items: "".to_string()
            },
            TrainerPokemon{
                iv: 150,
                level: 73,
                extra_scripts: "".to_string(),
                species: pokemon::Pokemon::Arcanine,
                moves: Vec::new(),
                held_items: "".to_string()
            },
            TrainerPokemon{
                iv: 150,
                level: 59,
                extra_scripts: "".to_string(),
                species: pokemon::Pokemon::Exeggutor,
                moves: Vec::new(),
                held_items: "".to_string()
            },
            TrainerPokemon{
                iv: 150,
                level: 75,
                extra_scripts: "".to_string(),
                species: pokemon::Pokemon::Venusaur,
                moves: Vec::new(),
                held_items: "".to_string()
            },
            TrainerPokemon{
                iv: 150,
                level: 16,
                extra_scripts: "".to_string(),
                species: pokemon::Pokemon::Raticate,
                moves: Vec::new(),
                held_items: "".to_string()
            },
            TrainerPokemon{
                iv: 150,
                level: 72,
                extra_scripts: "".to_string(),
                species: pokemon::Pokemon::Heracross,
                moves: Vec::new(),
                held_items: "".to_string()
            },
            TrainerPokemon{
                iv: 150,
                level: 72,
                extra_scripts: "".to_string(),
                species: pokemon::Pokemon::Tyranitar,
                moves: Vec::new(),
                held_items: "".to_string()
            }
        ]
    };
    fake_rival = trainers::get_random_trainer(fake_rival,settings,pokemon_data);

    for i in 0..10{
        all[BLUE].push(fake_rival.pokemon[i].species);
    }
    return all;
}

pub fn check_if_special_trainer(trainer: Trainer) -> bool{
    println!("Name Before: {}",trainer.trainer_name);
    //This is the name that the game uses for the Rival. Don't ask me why
    if trainer.trainer_name == "TERRY".to_string(){
        return true;
    }
    //Pewter City
    for i in [
        "LIAM",
        "BROCK"
    ]{
        if trainer.trainer_name == i.to_string(){return true;}
    }
    //Cerulean City
    for i in [
        "LUIS",
        "DIANA",
        "MISTY"
    ]{
        if trainer.trainer_name.to_uppercase() == i.to_string(){return true;}
    }
    //Vermilion City
    for i in [
        "DWAYNE",
        "BAILY",
        "TUCKER",
        "LT. SURGE"
    ]{
        if trainer.trainer_name.to_uppercase() == i.to_string(){return true;}
    }
    //Celadon City
    for i in [
        "KAY",
        "BRIDGET",
        "MARY",
        "LISA",
        "TINA",
        "LORI",
        "TAMIA",
        "ERIKA"
    ]{
        if trainer.trainer_name.to_uppercase() == i.to_string(){return true;}
    }
    //Fuchsia City
    for i in [
        "NATE",
        "KAYDEN",
        "KIRK",
        "EDGAR",
        "PHIL",
        "SHAWN",
        "KOGA"
    ]{
        if trainer.trainer_name.to_uppercase() == i.to_string(){return true;}
    }
    //Saffron City
    for i in [
        "CAMERON",
        "TYRON",
        "STACY",
        "PRESTON",
        "AMANDA",
        "TASHA",
        "JOHAN",
        "SABRINA"
    ]{
        if trainer.trainer_name.to_uppercase() == i.to_string(){return true;}
    }
    //Cinnabar Island
    for i in [
        "QUINN",
        "ERIK",
        "AVERY",
        "RAMON",
        "DEREK",
        "DUSTY",
        "ZAC",
        "BLAINE",
    ]{
        if trainer.trainer_name.to_uppercase() == i.to_string(){return true;}
    }
    //Viridian City
    for i in [
        "COLE",
        "KIYO",
        "SAMUEL",
        "YUJI",
        "ATSUSHI",
        "JASON",
        "WARREN",
        "TAKASHI",
        "GIOVANNI"
    ]{
        if trainer.trainer_name.to_uppercase() == i.to_string(){return true;}
    }
    //Pokemon League
    for i in [
        "LORELEI",
        "BRUNO",
        "AGATHA",
        "LANCE"
    ]{
        if trainer.trainer_name.to_uppercase() == i.to_string(){return true;}
    }
    return false;
}
//Pick Torchic -> Rival gets Mudkip
//Pick Treeko -> Rival gets Torchic
//Pick Mudkip -> Rival gets Treeko
pub fn handle_special_trainer(trainer: Trainer, settings: &mut settings::Settings,all_stats: &Vec<pokemon::PokemonStats>,
    starters: &starter_randomization::Starter,rivals: Vec<Vec<pokemon::Pokemon>>,gym_types: Vec<pokemon::Type>,elite_4_types: Vec<pokemon::Type>) -> Trainer{
        println!("Name: {}",trainer.trainer_name);
        match trainer.trainer_name.as_str(){
            //Rustboro City Gym
            "LIAM" => { return handle_gym_trainer(trainer,settings,all_stats,gym_types[0],pokemon::Type::Rock);}
            "BROCK" => {return handle_gym_leader(trainer,settings,all_stats,gym_types[0],pokemon::Type::Rock,0);}
            //Dewford City Gym
            "LUIS"|
            "DIANA" => {return handle_gym_trainer(trainer,settings,all_stats,gym_types[1],pokemon::Type::Fighting)}
            "MISTY" => {return handle_gym_leader(trainer,settings,all_stats,gym_types[1],pokemon::Type::Fighting,1);}
            //Mauville City Gym
            "DWAYNE"|
            "BAILY"|
            "TUCKER" =>  {return handle_gym_trainer(trainer,settings,all_stats,gym_types[2],pokemon::Type::Electric)}
            "LT. SURGE" =>{return handle_gym_leader(trainer,settings,all_stats,gym_types[2],pokemon::Type::Electric,2);}
            //Lavaridge Gym
            "KAY" |
            "BRIDGET" |
            "MARY" |
            "LISA" |
            "TINA" |
            "LORI" |
            "TAMIA" =>{return handle_gym_trainer(trainer,settings,all_stats,gym_types[3],pokemon::Type::Grass)}
            "ERIKA" =>{return handle_gym_leader(trainer,settings,all_stats,gym_types[3],pokemon::Type::Grass,3);}
            //Petalburg Gym
            "NATE" |
            "KAYDEN" |
            "KIRK" |
            "EDGAR" |
            "PHIL" |
            "SHAWN" => {return handle_gym_trainer(trainer,settings,all_stats,gym_types[4],pokemon::Type::Poison)}
            "KOGA"=>{return handle_gym_leader(trainer,settings,all_stats,gym_types[4],pokemon::Type::Poison,4);}
            //Fortree Gym
            "CAMERON" |
            "TYRON" |
            "STACY" |
            "PRESTON" |
            "AMANDA" |
            "TASHA" |
            "JOHAN" => {return handle_gym_trainer(trainer,settings,all_stats,gym_types[5],pokemon::Type::Psychic)}
            "SABRINA" => {return handle_gym_leader(trainer,settings,all_stats,gym_types[5],pokemon::Type::Psychic,5);}
            //Mosdeep Gym
            "QUINN" |
            "ERIK" |
            "AVERY" |
            "RAMON" |
            "DEREK" |
            "DUSTY" |
            "ZAC" => {return handle_gym_trainer(trainer,settings,all_stats,gym_types[6],pokemon::Type::Psychic)}
            "BLAINE" => {return handle_gym_leader(trainer,settings,all_stats,gym_types[6],pokemon::Type::Psychic,6);}
            //Sootopolis Gym
            "COLE" |
            "KIYO" |
            "SAMUEL" |
            "YUJI" |
            "ATSUSHI" |
            "JASON" |
            "WARREN" |
            "TAKASHI" => {return handle_gym_trainer(trainer,settings,all_stats,gym_types[7],pokemon::Type::Water)}
            "GIOVANNI" => {return handle_gym_leader(trainer,settings,all_stats,gym_types[7],pokemon::Type::Water,7);}
            //Pokemon League
            "LORELEI" => {return handle_gym_leader(trainer,settings,all_stats,elite_4_types[0],pokemon::Type::Dark,8);}
            "BRUNO" => {return handle_gym_leader(trainer,settings,all_stats,elite_4_types[1],pokemon::Type::Ice,9);}
            "AGATHA" => {return handle_gym_leader(trainer,settings,all_stats,elite_4_types[2],pokemon::Type::Ghost,10);}
            "LANCE" => {return handle_gym_leader(trainer,settings,all_stats,elite_4_types[3],pokemon::Type::Dragon,11);}
            _ => {
                let new_starter = starter_randomization::Starter{
                    treeko: pokemon::get_pokemon_data(pokemon::get_pokemon_from_name(wild_pokemon::get_random_wild_pokemon(settings, all_stats, 5),all_stats),all_stats),
                    torchic: pokemon::get_pokemon_data(pokemon::get_pokemon_from_name(wild_pokemon::get_random_wild_pokemon(settings, all_stats, 5),all_stats),all_stats),
                    mudkip: pokemon::get_pokemon_data(pokemon::get_pokemon_from_name(wild_pokemon::get_random_wild_pokemon(settings, all_stats, 5),all_stats),all_stats)
                };
                let new_rivals = if settings.rival_consistent_team{
                    rivals[BLUE].clone()
                }
                else{
                    get_rivals(settings,all_stats)[BLUE].clone()
                };
                return handle_blue(trainer,settings,all_stats,new_rivals,if settings.rival_keeps_starter{
                    starters
                }
                else{
                    &new_starter
                }
                );
            }
        }
}

fn handle_blue(trainer: Trainer,settings: &mut settings::Settings,all_stats: &Vec<pokemon::PokemonStats>,rival_team: Vec<pokemon::Pokemon>,starters: &starter_randomization::Starter) -> Trainer{
    let mut pkmn: Vec<TrainerPokemon> = Vec::new();
    let mut new_trainer = trainer.clone();
    for individual_pkmn in trainer.pokemon{
        let new_species = match individual_pkmn.species{
            pokemon::Pokemon::Pidgey | pokemon::Pokemon::Pidgeotto | pokemon::Pokemon::Pidgeot => rival_team[BLUE_PIDGEOT],
            pokemon::Pokemon::Abra | pokemon::Pokemon::Kadabra | pokemon::Pokemon::Alakazam => rival_team[BLUE_ALAKAZAM],
            pokemon::Pokemon::Rhydon | pokemon::Pokemon::Rhyhorn => rival_team[BLUE_RHYDON],
            pokemon::Pokemon::Magikarp | pokemon::Pokemon::Gyarados => rival_team[BLUE_GYARADOS],
            pokemon::Pokemon::Arcanine | pokemon::Pokemon::Growlithe => rival_team[BLUE_ARCANINE],
            pokemon::Pokemon::Exeggcute | pokemon::Pokemon::Exeggutor => rival_team[BLUE_EXECUTOR],
            pokemon::Pokemon::Rattata | pokemon::Pokemon::Raticate => rival_team[BLUE_RATTICATE],
            pokemon::Pokemon::Heracross => rival_team[BLUE_HERACROSS],
            pokemon::Pokemon::Tyranitar => rival_team[BLUE_TYRANITAR],
            pokemon::Pokemon::Bulbasaur | pokemon::Pokemon::Ivysaur | pokemon::Pokemon::Venusaur => starters.treeko.pokemon_id,
            pokemon::Pokemon::Squirtle | pokemon::Pokemon::Wartortle | pokemon::Pokemon::Blastoise => starters.mudkip.pokemon_id,
            pokemon::Pokemon::Charmander | pokemon::Pokemon::Charmeleon | pokemon::Pokemon::Charizard => starters.torchic.pokemon_id,
            _ => pokemon::Pokemon::Arceus
        };
        println!("FFF: {}",pokemon::pokemon_to_formatted_name(new_species,all_stats));
        pkmn.push(TrainerPokemon{
            iv: individual_pkmn.iv,
            species: scale_pokemon(new_species, individual_pkmn.level, all_stats, settings).pokemon_id,
            level: individual_pkmn.level,
            extra_scripts: individual_pkmn.extra_scripts,
            moves: trainers::create_moveset(settings,scale_pokemon(new_species,individual_pkmn.level,all_stats,&mut settings.clone()).pokemon_id,individual_pkmn.level,Vec::new()),
            held_items: trainers::create_held_item(settings, scale_pokemon(new_species,individual_pkmn.level,all_stats,&mut settings.clone()).pokemon_id, individual_pkmn.level, "".to_string(),true,all_stats)
        });
    }
    new_trainer.pokemon = pkmn;
    return new_trainer;
}

//Entrance point to handling gym trainer, calls handle_gym_normal_trainer and handle_gym_leader if needed
fn handle_gym_trainer(trainer: Trainer, settings: &mut settings::Settings,all_stats: &Vec<pokemon::PokemonStats>,pkmn_type: pokemon::Type,standard_type: pokemon::Type) -> Trainer{
    if settings.gym_type == settings::GymType::CompletelyRandom{
        println!("Completely random");
        return trainers::get_random_trainer(trainer, settings, all_stats);
    }
    else if settings.gym_type == settings::GymType::RandomType{
        println!("Random Type");
        return get_gym_trainer_pokemon(trainer,settings,all_stats,pkmn_type,settings.allow_trainer_legendaries.clone());
    }
    else{
        println!("Other");
        return get_gym_trainer_pokemon(trainer,settings,all_stats,standard_type,settings.allow_trainer_legendaries.clone());
    }
}

fn handle_gym_leader(trainer: Trainer, settings: &mut settings::Settings,all_stats: &Vec<pokemon::PokemonStats>,pkmn_type: pokemon::Type,standard_type: pokemon::Type,numGym: i16) -> Trainer{
    if !settings.randomize_trainer_pokemon {return trainer;}
    if numGym <= 1{
        return handle_gym_trainer(trainer,settings,all_stats,pkmn_type,standard_type);
    }
    let mut num_gimmick = 0;
    let mut has_mega = false;
    let mut has_legend = false;
    let mut has_z_crystal = false;
    let mut has_dynamax = false;
    let mut has_terra = false;
    for i in 0..numGym{
        //Escape condition (too many gimmicks)
        if numGym <= 7 && num_gimmick >= 1{break;}
        if numGym <= 11 && num_gimmick >= 2{break;}
        if num_gimmick >= 3{break;}
        //Pick a random number and test it against a gimmick (purposfully have more options than gimmicks so it will fail 50% of time)
        match settings::get_next_seed(0, 10 as i32, settings){
            0 => {
                if !has_mega{
                    has_mega = settings.gym_leader_megas;
                    num_gimmick += 1;
                }
            }
            1 => {
                if !has_legend{
                    has_legend = settings.gym_leader_legends;
                    num_gimmick += 1;
                }
            }
            2 => {
                if !has_z_crystal{
                    has_z_crystal = settings.gym_leader_z_crystal;
                    num_gimmick += 1;
                }
            }
            3 => {
                if !has_dynamax{
                    has_dynamax = settings.gym_leader_dynamax;
                    num_gimmick += 1;
                }
            }
            4 => {
                if !has_terra{
                    has_terra = settings.gym_leader_terra;
                    num_gimmick += 1;
                }
            }
            _ => {}
        }
    }

    //Get Mega Pokemon or Legendary pokemon
    let mut added_pokemon : Vec<TrainerPokemon> = Vec::new();
    let mut num_tries = 0;
    while num_tries < 80 && (has_mega || has_legend){
        let mut temp_mon = trainers::get_random_pokemon_for_trainer(trainer.clone().trainer_name, &trainer.pokemon[trainer.pokemon.len()-(1 + added_pokemon.len())],all_stats,settings,true);
        while (pokemon::get_pokemon_data(temp_mon.species,all_stats).type1 != pkmn_type && pokemon::get_pokemon_data(temp_mon.species,all_stats).type2 != pkmn_type){
            println!("Adding Loop");
            temp_mon = trainers::get_random_pokemon_for_trainer(trainer.clone().trainer_name, &trainer.pokemon[trainer.pokemon.len()-(1 + added_pokemon.len())],all_stats,settings,true);
        }
        match pokemon::get_pokemon_data(temp_mon.species,all_stats).status{
            pokemon::LegendStatus::Standard => {}
            pokemon::LegendStatus::Legendary | pokemon::LegendStatus::Mythical => {
                if has_legend{
                    added_pokemon.push(temp_mon);
                    has_legend = false;
                }
            }
            pokemon::LegendStatus::Mega => {
                if has_mega{
                    added_pokemon.push(temp_mon);
                    has_mega = false;
                }
            }
            pokemon::LegendStatus::LegendMega => {
                if (has_mega || has_legend) && added_pokemon.len() == 0{
                    added_pokemon.push(temp_mon);
                    has_legend = false;
                    has_mega = false;
                }
            }
        }
        println!("loop 1 {}",num_tries);
        num_tries += 1;
    }
    let mut cur_party_member = trainer.pokemon.len() - 1;
    //Combine it with the original party
    let mut new_party: VecDeque<TrainerPokemon> = VecDeque::new();
    for cur_mon in added_pokemon{
        new_party.push_front(cur_mon);
        cur_party_member -= 1;
    }

    if has_dynamax{
        let mut temp_mon = trainers::get_random_pokemon_for_trainer(trainer.clone().trainer_name, &trainer.pokemon[cur_party_member],all_stats,settings,false);
        while (pokemon::get_pokemon_data(temp_mon.species,all_stats).type1 != pkmn_type && pokemon::get_pokemon_data(temp_mon.species,all_stats).type2 != pkmn_type){
            println!("Dynamax Loop");
            temp_mon = trainers::get_random_pokemon_for_trainer(trainer.clone().trainer_name, &trainer.pokemon[cur_party_member],all_stats,settings,false);
        }
        temp_mon.extra_scripts = format!("\nDynamax Level: {}\nGigantamax: Yes",numGym - 2);
        new_party.push_front(temp_mon);
        cur_party_member -= 1;
    }
    if has_terra{
        let mut temp_mon = trainers::get_random_pokemon_for_trainer(trainer.clone().trainer_name, &trainer.pokemon[cur_party_member],all_stats,settings,false);
        let mut counter_failed = 5;
        while (pokemon::get_pokemon_data(temp_mon.species,all_stats).type1 != pkmn_type && pokemon::get_pokemon_data(temp_mon.species,all_stats).type2 != pkmn_type) || counter_failed > 0{
            println!("Terra Loop");
            temp_mon = trainers::get_random_pokemon_for_trainer(trainer.clone().trainer_name, &trainer.pokemon[cur_party_member],all_stats,settings,false);
            counter_failed -= 1;
        }
        if counter_failed == 0{
            //Same type as gym
            temp_mon.extra_scripts = format!("\nTera Type: {}",pokemon::type_to_string(pkmn_type));
        }
        else{
            temp_mon.extra_scripts = format!("\nTera Type: {}",pokemon::type_to_string(trainers::get_random_type(settings)));
        }
        new_party.push_front(temp_mon);
        cur_party_member -= 1;
    }
    if has_z_crystal{
        let mut temp_mon = trainers::get_random_pokemon_for_trainer(trainer.clone().trainer_name, &trainer.pokemon[cur_party_member],all_stats,settings,false);
        while (pokemon::get_pokemon_data(temp_mon.species,all_stats).type1 != pkmn_type && pokemon::get_pokemon_data(temp_mon.species,all_stats).type2 != pkmn_type){
            println!("Z crystal Loop");
            temp_mon = trainers::get_random_pokemon_for_trainer(trainer.clone().trainer_name, &trainer.pokemon[cur_party_member],all_stats,settings,false);
        }
        temp_mon.held_items = trainers::get_z_crystal(temp_mon.species,pkmn_type);
        new_party.push_front(temp_mon);
        cur_party_member -= 1;
    }

    while new_party.len() < trainer.pokemon.len(){
        let mut temp_mon = trainers::get_random_pokemon_for_trainer(trainer.clone().trainer_name, &trainer.pokemon[cur_party_member],all_stats,settings,false);
        while (pokemon::get_pokemon_data(temp_mon.species,all_stats).type1 != pkmn_type && pokemon::get_pokemon_data(temp_mon.species,all_stats).type2 != pkmn_type){
            println!("Adding Loop");
            temp_mon = trainers::get_random_pokemon_for_trainer(trainer.clone().trainer_name, &trainer.pokemon[cur_party_member],all_stats,settings,false);
            temp_mon.held_items = trainers::create_held_item(settings,temp_mon.species,temp_mon.level,temp_mon.held_items,true,all_stats);
        }
        println!("Number: {}, trainer: {}, Counter: {}",new_party.len(),trainer.pokemon.len(),cur_party_member);
        new_party.push_front(temp_mon);
        if cur_party_member == 0{break;}//Fix for an underflow. Bandaid fix, yes I know, but frankly if it works it works and I was confounded by this one
        cur_party_member -= 1;
    }
    let mut new_trainer = trainer.clone();
    new_trainer.pokemon = Vec::from(new_party);
    return new_trainer;
}

fn get_gym_trainer_pokemon(trainer: Trainer, settings: &mut settings::Settings,all_stats: &Vec<pokemon::PokemonStats>,pkmn_type: pokemon::Type,legend_rule: settings::AllowLegendaries) -> Trainer{
    if !settings.randomize_trainer_pokemon {return trainer;}
    let mut has_legend = false;
    println!("Hello There");
    let mut trainer_pokemon = Vec::new();
    for cur_pokemon in &trainer.pokemon{
        let mut is_correct_type = false;
        while !is_correct_type{
            let pokemon = trainers::get_random_pokemon_for_trainer(trainer.clone().trainer_name, &cur_pokemon,all_stats,settings,if settings.allow_trainer_legendaries.clone() == settings::AllowLegendaries::NoLegends ||
                (legend_rule == settings::AllowLegendaries::OneLegend && has_legend ||
                legend_rule == settings::AllowLegendaries::AceLegend && has_legend)
                   {false}else{true});
            if pokemon::get_pokemon_data(pokemon.species, all_stats).status == pokemon::LegendStatus::Legendary || pokemon::get_pokemon_data(pokemon.species, all_stats).status == pokemon::LegendStatus::LegendMega{
                has_legend = true;
            }
            if pkmn_type == pokemon::Type::Stellar || pokemon::get_pokemon_data(pokemon.species,all_stats).type1 == pkmn_type || pokemon::get_pokemon_data(pokemon.species,all_stats).type2 == pkmn_type{
                println!("Testing just in case");
                is_correct_type = true;
                trainer_pokemon.push(pokemon.clone());
            }
            println!("repeat stuff type= {}, pokemon types: {},{}",pkmn_type as i32,pokemon::get_pokemon_data(pokemon.clone().species,all_stats).type1 as i32, pokemon::get_pokemon_data(pokemon.clone().species,all_stats).type2 as i32);
        }
    }
    let mut new_trainer = trainer.clone();
    new_trainer.pokemon = trainer_pokemon;
    return new_trainer;
}

fn get_random_starter(settings: &mut settings::Settings,all_stats: &Vec<pokemon::PokemonStats>) -> pokemon::Pokemon{
    let mut fake_starter_trainer = Trainer{
        trainer_full_name: "r".to_string(),
        trainer_name: "r".to_string(),
        class: "".to_string(),
        pic: "".to_string(),
        gender: "".to_string(),
        music: "".to_string(),
        double_battle: "".to_string(),
        items: "".to_string(),
        ai: "".to_string(),
        portrait: "".to_string(),
        pokemon: vec![
            TrainerPokemon{
                iv: 200,
                level: 34,
                extra_scripts: "".to_string(),
                species: pokemon::Pokemon::Mudkip,
                moves: Vec::new(),
                held_items: "".to_string()
            }
        ]
    };
    trainers::get_random_trainer(fake_starter_trainer, settings, all_stats).pokemon[0].species
}