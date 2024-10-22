use crate::src::{gen3::{trainers::{Trainer, self, TrainerPokemon, scale_pokemon, MayBrendanTeam}, starter_randomization}, settings, pokemon};
use std::collections::VecDeque;


pub fn check_if_special_trainer(trainer: Trainer) -> bool{
    println!("Name Before: {}",trainer.trainer_name);
    for i in [
        "WALLY","BRENDAN","MAY"
        ]{
        if trainer.trainer_name == i.to_string(){
            return true;
        }
    }
    //Rustboro City
    for i in [
        "JOSH",
        "TOMMY",
        "MARC",
        "ROXANNE"
    ]{
        if trainer.trainer_name == i.to_string(){return true;}
    }
    //Dewford City
    for i in [
        "LAURA",
        "LILITH",
        "BRENDEN",
        "CRISTIAN",
        "JOCELYN",
        "BRAWLY"
    ]{
        if trainer.trainer_name.to_uppercase() == i.to_string(){return true;}
    }
    //Mauville City
    for i in [
        "VIVIAN",
        "KIRK",
        "BEN",
        "ANGELO",
        "SHAWN",
        "WATTSON"
    ]{
        if trainer.trainer_name.to_uppercase() == i.to_string(){return true;}
    }
    //Lavaridge City
    for i in [
        "JEFF",
        "JACE",
        "ELI",
        "COLE",
        "GERALD",
        "AXLE",
        "KEEGAN",
        "DANIELLE",
        "FLANNERY"
    ]{
        if trainer.trainer_name.to_uppercase() == i.to_string(){return true;}
    }
    //Petalburg City
    for i in [
        "RANDALL",
        "MARY",
        "PARKER",
        "ALEXIA",
        "GEORGE",
        "JODY",
        "BERKE",
        "NORMAN"
    ]{
        if trainer.trainer_name.to_uppercase() == i.to_string(){return true;}
    }
    //Fortree City
    for i in [
        "JARED",
        "HUMBERTO",
        "ASHLEY",
        "FLINT",
        "EDWARDO",
        "DARIUS",
        "WINONA"
    ]{
        if trainer.trainer_name.to_uppercase() == i.to_string(){return true;}
    }
    //Mosdeep City
    for i in [
        "PRESTON",
        "MAURA",
        "SAMANTHA",
        "BLAKE",
        "MACEY",
        "CLIFFORD",
        "KATHLEEN",
        "NICHOLAS",
        "NATE",
        "VIRGIL",
        "SYLVIA",
        "HANNAH",
        "TATE_AND_LIZA"
    ]{
        if trainer.trainer_name.to_uppercase() == i.to_string(){return true;}
    }
    //Sootopolis
    for i in [
        "CONNIE",
        "ANDREA",
        "DAPHNE",
        "ANNIKA",
        "TIFFANY",
        "CRISSY",
        "BETHANY",
        "OLIVIA",
        "BRIANNA",
        "BRIDGET",
        "JUAN"
    ]{
        if trainer.trainer_name.to_uppercase() == i.to_string(){return true;}
    }
    //Pokemon League
    for i in [
        "DRAKE",
        "PHOEBE",
        "GLACIA",
        "SIDNEY",
        "WALLACE"
    ]{
        if trainer.trainer_name.to_uppercase() == i.to_string(){return true;}
    }
    return false;
}
//Pick Torchic -> Rival gets Mudkip
//Pick Treeko -> Rival gets Torchic
//Pick Mudkip -> Rival gets Treeko
pub fn handle_special_trainer(trainer: Trainer, settings: &mut settings::Settings,all_stats: &Vec<pokemon::PokemonStats>,
    starters: &starter_randomization::Starter,rival: &trainers::MayBrendanTeam,wally: &trainers::WallyTeam,gym_types: Vec<pokemon::Type>) -> Trainer{
        println!("Name: {}",trainer.trainer_name);
        match trainer.trainer_name.as_str(){
            //Rustboro City Gym
            "JOSH"|
            "TOMMY"|
            "MARC" => { return handle_gym_trainer(trainer,settings,all_stats,gym_types[0],pokemon::Type::Rock);}
            "ROXANNE" => {return handle_gym_leader(trainer,settings,all_stats,gym_types[0],pokemon::Type::Rock,0);}
            //Dewford City Gym
            "LAURA"|
            "LILITH"|
            "BRENDEN"|
            "CRISTIAN"|
            "JOCELYN" => {return handle_gym_trainer(trainer,settings,all_stats,gym_types[1],pokemon::Type::Fighting)}
            "BRAWLY" => {return handle_gym_leader(trainer,settings,all_stats,gym_types[1],pokemon::Type::Fighting,1);}
            //Mauville City Gym
            "VIVIAN"|
            "KIRK"|
            "BEN"|
            "ANGELO"|
            "SHAWN" =>  {return handle_gym_trainer(trainer,settings,all_stats,gym_types[2],pokemon::Type::Electric)}
            "WATTSON" =>{return handle_gym_leader(trainer,settings,all_stats,gym_types[2],pokemon::Type::Electric,2);}
            //Lavaridge Gym
            "JEFF" |
            "JACE" |
            "ELI" |
            "COLE" |
            "GERALD" |
            "AXLE" |
            "KEEGAN" |
            "DANIELLE" =>{return handle_gym_trainer(trainer,settings,all_stats,gym_types[3],pokemon::Type::Fire)}
            "FLANNERY" =>{return handle_gym_leader(trainer,settings,all_stats,gym_types[3],pokemon::Type::Fire,3);}
            //Petalburg Gym
            "RANDALL" |
            "MARY" |
            "PARKER" |
            "ALEXIA" |
            "GEORGE" |
            "JODY" |
            "BERKE" => {return handle_gym_trainer(trainer,settings,all_stats,gym_types[4],pokemon::Type::Normal)}
            "NORMAN"=>{return handle_gym_leader(trainer,settings,all_stats,gym_types[4],pokemon::Type::Normal,4);}
            //Fortree Gym
            "JARED" |
            "HUMBERTO" |
            "ASHLEY" |
            "FLINT" |
            "EDWARDO" |
            "DARIUS" => {return handle_gym_trainer(trainer,settings,all_stats,gym_types[5],pokemon::Type::Flying)}
            "WINONA" => {return handle_gym_leader(trainer,settings,all_stats,gym_types[5],pokemon::Type::Flying,5);}
            //Mosdeep Gym
            "PRESTON" |
            "MAURA" |
            "SAMANTHA" |
            "BLAKE" |
            "MACEY" |
            "CLIFFORD" |
            "KATHLEEN" |
            "NICHOLAS" |
            "NATE" |
            "VIRGIL" |
            "SYLVIA" |
            "HANNAH" => {return handle_gym_trainer(trainer,settings,all_stats,gym_types[6],pokemon::Type::Psychic)}
            "TATE_AND_LIZA" => {return handle_gym_leader(trainer,settings,all_stats,gym_types[6],pokemon::Type::Psychic,6);}
            //Sootopolis Gym
            "CONNIE" |
            "ANDREA" |
            "DAPHNE" |
            "ANNIKA" |
            "TIFFANY" |
            "CRISSY" |
            "BETHANY" |
            "OLIVIA" |
            "BRIANNA" |
            "BRIDGET" => {return handle_gym_trainer(trainer,settings,all_stats,gym_types[7],pokemon::Type::Water)}
            "JUAN" => {return handle_gym_leader(trainer,settings,all_stats,gym_types[7],pokemon::Type::Water,7);}
            //Pokemon League
            "SIDNEY" => {return handle_gym_leader(trainer,settings,all_stats,gym_types[8],pokemon::Type::Dark,8);}
            "GLACIA" => {return handle_gym_leader(trainer,settings,all_stats,gym_types[9],pokemon::Type::Ice,9);}
            "PHOEBE" => {return handle_gym_leader(trainer,settings,all_stats,gym_types[10],pokemon::Type::Ghost,10);}
            "DRAKE" => {return handle_gym_leader(trainer,settings,all_stats,gym_types[11],pokemon::Type::Dragon,11);}
            "WALLACE" => {return handle_gym_leader(trainer,settings,all_stats,gym_types[12],pokemon::Type::Water,12);}
            //Wally's
            "WALLY" =>{
                if trainer.trainer_full_name == "=== TRAINER_WALLY_MAUVILLE ==="{
                    return if settings.wally_keeps_starter{handle_wally(settings, all_stats, trainer.trainer_full_name, 1, wally)}else{trainers::get_random_trainer(trainer, settings, all_stats)}
                }
                else{
                    return if settings.wally_keeps_starter{handle_wally(settings, all_stats, trainer.trainer_full_name, 2, wally)}else{trainers::get_random_trainer(trainer, settings, all_stats)}
                }
            }
            _ => {
                //Handle Rivals and all their fights (I swear this is a better way to do this than manually inputting all like 32 fights)
                for i in ["May","Brendan"]{
                    for j in 0..3{
                        let strts = ["TREECKO","TORCHIC","MUDKIP"];
                        //starter_ids offset by 1 so that the rival gets the right pokemon
                        let starter_ids = [&starters.torchic.pokemon_id,&starters.mudkip.pokemon_id,&starters.treeko.pokemon_id];
                        for k in 0..5{
                            let random_starter = get_random_starter(settings, all_stats);
                            let random_team = randomize_rival_team(settings, all_stats);
                            let routes = ["ROUTE_103","RUSTBORO","ROUTE_110","ROUTE_119","LILYCOVE"];
                            if trainer.trainer_full_name == format!("=== TRAINER_{}_{}_{} ===",i,routes[k],strts[j]).to_uppercase(){
                                return handle_rival(settings, all_stats, trainer, k as i16+1, 
                                    if settings.rival_keeps_starter {starter_ids[j]} else{&random_starter},
                                    if settings.rival_consistent_team {rival} else{&random_team});
                            }
                        }
                    }
                }
                return trainers::get_random_trainer(trainer, settings, all_stats);
            }
        }
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
                    has_mega = true;
                    num_gimmick += 1;
                }
            }
            1 => {
                if !has_legend{
                    has_legend = true;
                    num_gimmick += 1;
                }
            }
            2 => {
                if !has_z_crystal{
                    has_z_crystal = true;
                    num_gimmick += 1;
                }
            }
            3 => {
                if !has_dynamax{
                    has_dynamax = true;
                    num_gimmick += 1;
                }
            }
            4 => {
                if !has_terra{
                    has_mega = true;
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
        let mut temp_mon = trainers::get_random_pokemon_for_trainer(trainer.clone().trainer_name, &trainer.pokemon[trainer.pokemon.len()-1],all_stats,settings,true);
        while (pokemon::get_pokemon_data(temp_mon.species,all_stats).type1 != pkmn_type && pokemon::get_pokemon_data(temp_mon.species,all_stats).type2 != pkmn_type){
            println!("Adding Loop");
            temp_mon = trainers::get_random_pokemon_for_trainer(trainer.clone().trainer_name, &trainer.pokemon[trainer.pokemon.len()-1],all_stats,settings,true);
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
    let mut cur_party_member = 0;
    //Combine it with the original party
    let mut new_party: VecDeque<TrainerPokemon> = VecDeque::new();
    for cur_mon in added_pokemon{
        new_party.push_front(cur_mon);
        cur_party_member += 1;
    }

    if has_dynamax{
        let mut temp_mon = trainers::get_random_pokemon_for_trainer(trainer.clone().trainer_name, &trainer.pokemon[cur_party_member],all_stats,settings,false);
        while (pokemon::get_pokemon_data(temp_mon.species,all_stats).type1 != pkmn_type && pokemon::get_pokemon_data(temp_mon.species,all_stats).type2 != pkmn_type){
            println!("Dynamax Loop");
            temp_mon = trainers::get_random_pokemon_for_trainer(trainer.clone().trainer_name, &trainer.pokemon[cur_party_member],all_stats,settings,false);
        }
        temp_mon.extra_scripts = format!("Dynamax Level: {}\nGigantamax: Yes",numGym - 2);
        new_party.push_front(temp_mon);
        cur_party_member += 1;
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
            temp_mon.extra_scripts = format!("Tera Type: {}",pokemon::type_to_string(pkmn_type));
        }
        else{
            temp_mon.extra_scripts = format!("Terra Type: {}",pokemon::type_to_string(get_random_type(settings)));
        }
        new_party.push_front(temp_mon);
        cur_party_member += 1;
    }
    if has_z_crystal{
        let mut temp_mon = trainers::get_random_pokemon_for_trainer(trainer.clone().trainer_name, &trainer.pokemon[cur_party_member],all_stats,settings,false);
        while (pokemon::get_pokemon_data(temp_mon.species,all_stats).type1 != pkmn_type && pokemon::get_pokemon_data(temp_mon.species,all_stats).type2 != pkmn_type){
            println!("Z crystal Loop");
            temp_mon = trainers::get_random_pokemon_for_trainer(trainer.clone().trainer_name, &trainer.pokemon[cur_party_member],all_stats,settings,false);
        }
        temp_mon.held_items = trainers::get_z_crystal(temp_mon.species,pkmn_type);
        new_party.push_front(temp_mon);
        cur_party_member += 1;
    }

    while new_party.len() < trainer.pokemon.len(){
        let mut temp_mon = trainers::get_random_pokemon_for_trainer(trainer.clone().trainer_name, &trainer.pokemon[cur_party_member],all_stats,settings,false);
        while (pokemon::get_pokemon_data(temp_mon.species,all_stats).type1 != pkmn_type && pokemon::get_pokemon_data(temp_mon.species,all_stats).type2 != pkmn_type){
            println!("Adding Loop");
            temp_mon = trainers::get_random_pokemon_for_trainer(trainer.clone().trainer_name, &trainer.pokemon[cur_party_member],all_stats,settings,false);
        }
        new_party.push_front(temp_mon);
        cur_party_member += 1;
    }
    let mut new_trainer = trainer.clone();
    new_trainer.pokemon = Vec::from(new_party);
    return new_trainer;
}

fn get_gym_trainer_pokemon(trainer: Trainer, settings: &mut settings::Settings,all_stats: &Vec<pokemon::PokemonStats>,pkmn_type: pokemon::Type,legend_rule: settings::AllowLegendaries) -> Trainer{
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

fn get_random_type(settings: &mut settings::Settings) -> pokemon::Type{
    let rand_val = settings::get_next_seed(0, 19 as i32, settings);
    return match rand_val{
        0 => pokemon::Type::Normal,
        1 => pokemon::Type::Fire,
        2 => pokemon::Type::Water,
        4 => pokemon::Type::Electric,
        5 => pokemon::Type::Grass,
        6 => pokemon::Type::Ice,
        7 => pokemon::Type::Fighting,
        8 => pokemon::Type::Poison,
        9 => pokemon::Type::Ground,
        10 => pokemon::Type::Flying,
        11 => pokemon::Type::Psychic,
        12 => pokemon::Type::Bug,
        13 => pokemon::Type::Rock,
        14 => pokemon::Type::Ghost,
        15 => pokemon::Type::Dragon,
        16 => pokemon::Type::Dark,
        17 => pokemon::Type::Steel,
        18 => pokemon::Type::Fairy,
        _ => pokemon::Type::Stellar
    };
}

pub fn randomize_gym_types(num_badges: i16,settings: &mut settings::Settings) -> Vec<pokemon::Type>{
    let mut all_badges : Vec<pokemon::Type> = Vec::new();
    for i in 0..num_badges{
        all_badges.push(get_random_type(settings));
        println!("Badge {} is # {}",i,all_badges[i as usize] as i32);
    }
    return all_badges;
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
fn randomize_rival_team(settings: &mut settings::Settings,all_stats: &Vec<pokemon::PokemonStats>) -> MayBrendanTeam{
    let mut fake_rival = Trainer{

        trainer_full_name: "TRAINER_RIVAL".to_string(),
        trainer_name: "rival".to_string(),
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
                iv: 15,
                level: 31,
                extra_scripts: "".to_string(),
                species: pokemon::Pokemon::Tropius,
                moves: Vec::new(),
                held_items: "".to_string()
            },
            TrainerPokemon{
                iv: 15,
                level: 32,
                extra_scripts: "".to_string(),
                species: pokemon::Pokemon::Ludicolo,
                moves: Vec::new(),
                held_items: "".to_string()
            },
            TrainerPokemon{
                iv: 15,
                level: 32,
                extra_scripts: "".to_string(),
                species: pokemon::Pokemon::Slugma,
                moves: Vec::new(),
                held_items: "".to_string()
            }
        ]
    };
    fake_rival = trainers::get_random_trainer(fake_rival,settings,all_stats);
    MayBrendanTeam{
        pokemon2: fake_rival.pokemon[0].species,
        pokemon3: fake_rival.pokemon[1].species,
        pokemon4: fake_rival.pokemon[2].species,
    }
}
//Treeko team ->
//starter: Combusken
//pokemon2: Ludicolo
//pokemon3: Pelipper
//pokemon4: Tropius
fn handle_rival(settings: &mut settings::Settings,all_stats: &Vec<pokemon::PokemonStats>,mut trainer: Trainer,num_battle: i16,starter: &pokemon::Pokemon,rival_team: &trainers::MayBrendanTeam) -> Trainer{
    trainer.pokemon = match num_battle{
        2 => vec![ //Rustboro
            TrainerPokemon{
                iv: 25,
                level: 13,
                extra_scripts: "".to_string(),
                species: scale_pokemon(rival_team.pokemon2.clone(), 13, all_stats, settings).pokemon_id,
                moves: trainers::create_moveset(settings, scale_pokemon(rival_team.pokemon2.clone(), 13, all_stats, &mut settings.clone()).pokemon_id, 13, Vec::new()),
                held_items: trainers::create_held_item(settings, scale_pokemon(rival_team.pokemon2.clone(), 13, all_stats, &mut settings.clone()).pokemon_id, 13, "".to_string())
            },
            TrainerPokemon{
                iv: 25,
                level: 15,
                extra_scripts: "".to_string(),
                species: scale_pokemon(starter.clone(), 15, all_stats, settings).pokemon_id,
                moves: trainers::create_moveset(settings, scale_pokemon(starter.clone(), 15, all_stats, &mut settings.clone()).pokemon_id, 15, Vec::new()),
                held_items: trainers::create_held_item(settings, scale_pokemon(starter.clone(), 15, all_stats, &mut settings.clone()).pokemon_id, 15, "".to_string())
            } 
        ],
        3 => vec![ //Route 110 (Hell)
            TrainerPokemon{
                iv: 50,
                level: 18,
                extra_scripts: "".to_string(),
                species: scale_pokemon(rival_team.pokemon3.clone(), 18, all_stats, settings).pokemon_id,
                moves: trainers::create_moveset(settings, scale_pokemon(rival_team.pokemon3.clone(), 18, all_stats, &mut settings.clone()).pokemon_id, 18, Vec::new()),
                held_items: trainers::create_held_item(settings, scale_pokemon(rival_team.pokemon3.clone(), 18, all_stats, &mut settings.clone()).pokemon_id, 18, "".to_string())
            },
            TrainerPokemon{
                iv: 50,
                level: 18,
                extra_scripts: "".to_string(),
                species: scale_pokemon(rival_team.pokemon2, 18, all_stats, settings).pokemon_id,
                moves: trainers::create_moveset(settings, scale_pokemon(rival_team.pokemon3.clone(), 18, all_stats, &mut settings.clone()).pokemon_id, 18, Vec::new()),
                held_items: trainers::create_held_item(settings, scale_pokemon(rival_team.pokemon3.clone(), 18, all_stats, &mut settings.clone()).pokemon_id, 18, "".to_string())
            },
            TrainerPokemon{
                iv: 100,
                level: 20,
                extra_scripts: "".to_string(),
                species: scale_pokemon(starter.clone(), 20, all_stats, settings).pokemon_id,
                moves: trainers::create_moveset(settings, scale_pokemon(starter.clone(), 20, all_stats, &mut settings.clone()).pokemon_id, 20, Vec::new()),
                held_items: trainers::create_held_item(settings, scale_pokemon(starter.clone(), 20, all_stats, &mut settings.clone()).pokemon_id, 20, "".to_string())
            }
        ],
        4 => vec![ //Route 119
            TrainerPokemon{
                iv: 100,
                level: 29,
                extra_scripts: "".to_string(),
                species: scale_pokemon(rival_team.pokemon3.clone(), 29, all_stats, settings).pokemon_id,
                moves: trainers::create_moveset(settings, scale_pokemon(rival_team.pokemon3.clone(), 29, all_stats, &mut settings.clone()).pokemon_id, 29, Vec::new()),
                held_items: trainers::create_held_item(settings, scale_pokemon(rival_team.pokemon3.clone(), 29, all_stats, &mut settings.clone()).pokemon_id, 29, "".to_string())
            },
            TrainerPokemon{
                iv: 100,
                level: 29,
                extra_scripts: "".to_string(),
                species: scale_pokemon(rival_team.pokemon2, 29, all_stats, settings).pokemon_id,
                moves: trainers::create_moveset(settings, scale_pokemon(rival_team.pokemon3.clone(), 29, all_stats, &mut settings.clone()).pokemon_id, 29, Vec::new()),
                held_items: trainers::create_held_item(settings, scale_pokemon(rival_team.pokemon3.clone(), 29, all_stats, &mut settings.clone()).pokemon_id, 29, "".to_string())
            },
            TrainerPokemon{
                iv: 100,
                level: 31,
                extra_scripts: "".to_string(),
                species: scale_pokemon(starter.clone(), 31, all_stats, settings).pokemon_id,
                moves: trainers::create_moveset(settings, scale_pokemon(starter.clone(), 31, all_stats, &mut settings.clone()).pokemon_id, 31, Vec::new()),
                held_items: trainers::create_held_item(settings, scale_pokemon(starter.clone(), 31, all_stats, &mut settings.clone()).pokemon_id, 31, "".to_string())
            }
        ],
        5 => vec![ //Lillycove
            //Dont have to scale pokemon here because by default they are scaled to this fight
            TrainerPokemon{
                iv: 15,
                level: 31,
                extra_scripts: "".to_string(),
                species: rival_team.pokemon4.clone(),
                moves: trainers::create_moveset(settings,rival_team.pokemon4.clone(),31,Vec::new()),
                held_items: trainers::create_held_item(settings, rival_team.pokemon4.clone(), 31, "".to_string())
            },
            TrainerPokemon{
                iv: 15,
                level: 32,
                extra_scripts: "".to_string(),
                species: rival_team.pokemon3.clone(),
                moves: trainers::create_moveset(settings, rival_team.pokemon3.clone(), 32, Vec::new()),
                held_items: trainers::create_held_item(settings, rival_team.pokemon3.clone(), 32, "".to_string())
            },
            TrainerPokemon{
                iv: 15,
                level: 32,
                extra_scripts: "".to_string(),
                species: rival_team.pokemon2.clone(),
                moves: trainers::create_moveset(settings, rival_team.pokemon3.clone(), 32, Vec::new()),
                held_items: trainers::create_held_item(settings, rival_team.pokemon3.clone(), 32, "".to_string())
            },
            TrainerPokemon{
                iv: 15,
                level: 34,
                extra_scripts: "".to_string(),
                species: scale_pokemon(starter.clone(), 34, all_stats, settings).pokemon_id,
                moves: trainers::create_moveset(settings, starter.clone(), 34, Vec::new()),
                held_items: trainers::create_held_item(settings, starter.clone(), 34, "".to_string())
            }
        ],
        _ => vec![TrainerPokemon{ //First Battle (Route 103)
            iv: 0,
            level: 5,
            extra_scripts: "".to_string(),
            species: scale_pokemon(starter.clone(), 5, all_stats, settings).pokemon_id,
            moves: Vec::new(),
            held_items: "".to_string()
        }]
    };
    return trainer;
}

fn handle_wally(settings: &mut settings::Settings,all_stats: &Vec<pokemon::PokemonStats>,rival_name: String,num_battle: i16,wally_team: &trainers::WallyTeam) -> Trainer{
    match num_battle{
        1 => Trainer{ //Mawville City Rival Fight
            trainer_full_name: rival_name,
            trainer_name: "WALLY".to_string(),
            class: "Rival".to_string(),
            pic: "Wally".to_string(),
            gender: "Male".to_string(),
            music: "Male".to_string(),
            items: "".to_string(),
            double_battle: "No".to_string(),
            ai: "Check Bad Move / Try To Faint / Check Viability".to_string(),
            portrait: "".to_string(),
            pokemon: vec![
                TrainerPokemon{
                    iv: 30,
                    level: 16,
                    extra_scripts: "".to_string(),
                    species: scale_pokemon(wally_team.ralt_substitute.clone(),16,all_stats,settings).pokemon_id,
                    moves: trainers::create_moveset(settings,scale_pokemon(wally_team.ralt_substitute,16,all_stats,&mut settings.clone()).pokemon_id,16,Vec::new()),
                    held_items: trainers::create_held_item(settings, scale_pokemon(wally_team.ralt_substitute,16,all_stats,&mut settings.clone()).pokemon_id, 16, "".to_string())
                }
            ]
        },
        2 => Trainer { //Victory Road
            trainer_full_name: rival_name,
            trainer_name: "WALLY".to_string(),
            class: "Rival".to_string(),
            pic: "Wally".to_string(),
            gender: "Male".to_string(),
            music: "Male".to_string(),
            items: "".to_string(),
            double_battle: "No".to_string(),
            ai: "Check Bad Move / Try To Faint / Check Viability".to_string(),
            portrait: "".to_string(),
            pokemon: vec![
                TrainerPokemon{
                    iv: 15,
                    level: 44,
                    extra_scripts: "".to_string(),
                    species: scale_pokemon(wally_team.pokemon2, 44, all_stats, settings).pokemon_id,
                    moves: trainers::create_moveset(settings,scale_pokemon(wally_team.pokemon2,44,all_stats,&mut settings.clone()).pokemon_id,44,Vec::new()),
                    held_items: trainers::create_held_item(settings, scale_pokemon(wally_team.pokemon2, 44, all_stats, &mut settings.clone()).pokemon_id, 44, "".to_string())
                },
                TrainerPokemon{
                    iv: 15,
                    level: 43,
                    extra_scripts: "".to_string(),
                    species: scale_pokemon(wally_team.pokemon3, 43, all_stats, settings).pokemon_id,
                    moves: trainers::create_moveset(settings,scale_pokemon(wally_team.pokemon3,43,all_stats,&mut settings.clone()).pokemon_id,43,Vec::new()),
                    held_items: trainers::create_held_item(settings, scale_pokemon(wally_team.pokemon3, 43, all_stats, &mut settings.clone()).pokemon_id, 43, "".to_string())
                },
                TrainerPokemon{
                    iv: 15,
                    level: 44,
                    extra_scripts: "".to_string(),
                    species: scale_pokemon(wally_team.pokemon4, 44, all_stats, settings).pokemon_id,
                    moves: trainers::create_moveset(settings,scale_pokemon(wally_team.pokemon4,44,all_stats,&mut settings.clone()).pokemon_id,44,Vec::new()),
                    held_items: trainers::create_held_item(settings, scale_pokemon(wally_team.pokemon4, 44, all_stats, &mut settings.clone()).pokemon_id, 44, "".to_string())
                },
                TrainerPokemon{
                    iv: 15,
                    level: 41,
                    extra_scripts: "".to_string(),
                    species: scale_pokemon(wally_team.pokemon5, 41, all_stats, settings).pokemon_id,
                    moves: trainers::create_moveset(settings,scale_pokemon(wally_team.pokemon5,41,all_stats,&mut settings.clone()).pokemon_id,41,Vec::new()),
                    held_items: trainers::create_held_item(settings, scale_pokemon(wally_team.pokemon4, 41, all_stats, &mut settings.clone()).pokemon_id, 41, "".to_string())
                },
                TrainerPokemon{
                    iv: 15,
                    level: 45,
                    extra_scripts: "".to_string(),
                    species: scale_pokemon(wally_team.ralt_substitute, 45, all_stats, settings).pokemon_id,
                    moves: trainers::create_moveset(settings,scale_pokemon(wally_team.ralt_substitute,45,all_stats,&mut settings.clone()).pokemon_id,45,Vec::new()),
                    held_items: trainers::create_held_item(settings, scale_pokemon(wally_team.pokemon4, 45, all_stats, &mut settings.clone()).pokemon_id, 45, "".to_string())
                }
            ]
        },
        _ => Trainer {trainer_full_name: "TRAINER_WALLY".to_string(), trainer_name: rival_name,
        class: "Rival".to_string(),
        pic: "Wally".to_string(),
        gender: "Male".to_string(),
        music: "Male".to_string(),
        items: "".to_string(),
        double_battle: "No".to_string(),
        ai: "Check Bad Move / Try To Faint / Check Viability".to_string(),
        portrait: "".to_string(),
        pokemon: vec![TrainerPokemon{ //Fake Battle (Route 103)
            iv: 0,
            level: 5,
            extra_scripts: "".to_string(),
            species: scale_pokemon(wally_team.ralt_substitute, 5, all_stats, settings).pokemon_id,
            moves: Vec::new(),
            held_items: "".to_string()
        }]},
    }
}