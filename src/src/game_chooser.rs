use crate::src::gen3::emerald;
use crate::src::settings;
use crate::src::pokemon;
use crate::src::gen3::trainers;
use crate::src::gen3::starter_randomization;
use crate::src::gen3::startup;


//Things that should be handled through this file
/*
    battle_setup.c
    battle_setup2.c
    item_locations.csv -------
    main_menu_ptx.c  ---------
    starter_choose.c
    trainer_parties.json
    trainers.txt
    wild_encounters.json
    other.rs
    special_trainers.rs
    startup_stuff.rs
    static_pokemon.rs
*/
/*
Actual in game files that need to be handled by this file
    data/maps/**/*.json
    data/scripts/randomizer_scripts.inc
    src/data/trainers.h
    src/data/trainers.party
    src/data/wild_encounters.json
    src/main_menu.c
    src/starter_choose.c
    src/battle_setup.c
    data/maps/map_groups.json
*/
/*
Possibly can be moved out
    evolution_unchanged
    items.json
    pokemon_names.csv
    pokemon.csv
    randomizer_scripts_common
    startup.rs
*/






//-------------------------------------------------------------------------Function Forwarders---------------------------------------------------
pub fn do_starter_randomization(settings: &mut settings::Settings, pkmn_data: &Vec<pokemon::PokemonStats>) -> starter_randomization::Starter{
    return match settings.game{
        settings::Game::Emerald => starter_randomization::randomize_starter_pokemon(settings,pkmn_data,"data/gen3/emerald/starter_choose.c".to_string(),"data/gen3/emerald/starter_choose_2.c".to_string(),"decomp/pokeemerald-expansion/src/starter_choose.c".to_string())
    }
}

pub fn do_trainer_randomization(settings: &mut settings::Settings,pkmn_data: &Vec<pokemon::PokemonStats>,starters: starter_randomization::Starter) -> Vec<pokemon::Type>{
    return match settings.game{
        settings::Game::Emerald => trainers::shuffle_trainers(settings,&pkmn_data,"data/gen3/emerald/trainers.txt".to_string(),"decomp/pokeemerald-expansion/src/data/trainers.party".to_string(),starters)
    }
}

pub fn main_randomizer_script(settings: &mut settings::Settings){
    match settings.game{
        settings::Game::Emerald => startup::randomize_pokemon(settings)
    }
}

pub fn randomize_professor_pokemon(settings: &mut settings::Settings, pokemon_data: &Vec<pokemon::PokemonStats>){
    match settings.game{
        settings::Game::Emerald => emerald::other::randomize_birch_pokemon(settings,pokemon_data)
    }
}

pub fn randomize_static_pokemon(settings: &mut settings::Settings,pokemon_data: &Vec<pokemon::PokemonStats>,rival: &trainers::MayBrendanTeam,wally: &trainers::WallyTeam){
    match settings.game{
        settings::Game::Emerald => emerald::static_pokemon::randomize_static_pokemon(settings,pokemon_data,rival,wally)
    }
}

pub fn check_if_special_trainer(settings: &mut settings::Settings,trainer: trainers::Trainer) -> bool{
    return match settings.game{
        settings::Game::Emerald => emerald::special_trainers::check_if_special_trainer(trainer)
    };
}

pub fn handle_special_trainer(trainer: trainers::Trainer, settings: &mut settings::Settings,all_stats: &Vec<pokemon::PokemonStats>,
    starters: &starter_randomization::Starter,rival: &trainers::MayBrendanTeam,rival_2: &trainers::WallyTeam,gym_types: Vec<pokemon::Type>,elite_4_types: Vec<pokemon::Type>) -> trainers::Trainer{
    return match settings.game{
        settings::Game::Emerald => emerald::special_trainers::handle_special_trainer(trainer,settings,all_stats,starters,rival,rival_2,gym_types,elite_4_types)
    };
}

pub fn startup_stuff(settings: &mut settings::Settings) -> String{
    return match settings.game{
    settings::Game::Emerald => emerald::startup_stuff::get_startup_stuff(settings)
    }
}


//-------------------------------------------------------------------------Strings and Filenames-------------------------------------------------
pub fn get_pokemon_data_file(settings: &settings::Settings) -> String{
    return match settings.game{
        settings::Game::Emerald => "data/gen3/pokemon.csv".to_string()
    };
}

pub fn get_wild_pokemon_data_file(settings: &settings::Settings) -> String{
    return match settings.game{
        settings::Game::Emerald => "data/gen3/emerald/wild_encounters.json".to_string()
    }
}

pub fn get_wild_pokemon_file(settings: &settings::Settings) -> String{
    return match settings.game{
        settings::Game::Emerald => "decomp/pokeemerald-expansion/src/data/wild_encounters.json".to_string()
    }
}

pub fn get_item_locations(settings: &settings::Settings) -> String{
    return match settings.game{
        settings::Game::Emerald => "data/gen3/emerald/item_locations.csv".to_string()
    }
}

pub fn get_items(settings: &settings::Settings) -> String{
    return match settings.game{
        settings::Game::Emerald => "data/gen3/items.json".to_string()
    }
}

pub fn randomizer_scripts(settings: &settings::Settings) -> String{
    return match settings.game{
        settings::Game::Emerald => "decomp/pokeemerald-expansion/data/scripts/randomizer_scripts.inc".to_string()
    }
}

pub fn build_rom(settings: &settings::Settings) -> String{
    return match settings.game{
        settings::Game::Emerald => "src/src/gen3/emerald/make_rom.sh".to_string()
    }
}
pub fn get_randomizer_script_filename(settings: &mut settings::Settings) -> String{
    return match settings.game{
        settings::Game::Emerald => "decomp/pokeemerald-expansion/data/scripts/randomizer_scripts.inc".to_string()
    }
}

pub fn get_map_json_files(settings: &settings::Settings) -> String{
    return match settings.game{
        settings::Game::Emerald => "decomp/pokeemerald-expansion/data/maps/**/*.json".to_string()
    }
}

//------------------------------------------------------------------Just Numbers here-----------------------------------------------------------------

//Generally should be 8, but Johto exists so we are going to make this a function for future proofing
pub fn num_gym_leaders(settings: &settings::Settings) -> i16{
    return match settings.game{
        settings::Game::Emerald => 8
    }
}

//Elite 4 types should also include Champion, generally this should be 5. Probably a reason to make it not 5, but I have no clue what that could be.
pub fn num_elite_4(settings: &settings::Settings) -> i16{
    return match settings.game{
        settings::Game::Emerald => 5
    }
}

pub fn get_gym_ace_level(settings: &settings::Settings,num_gym: i16) -> i16{
    return match settings.game{
        settings::Game::Emerald => vec![15,19,24,29,31,33,42,46][num_gym as usize]
    }
}