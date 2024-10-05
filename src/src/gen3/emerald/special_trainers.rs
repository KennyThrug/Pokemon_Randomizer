use crate::src::{gen3::{trainers::{Trainer, self, TrainerPokemon, scale_pokemon, MayBrendanTeam}, starter_randomization}, settings, pokemon};



pub fn check_if_special_trainer(trainer: Trainer) -> bool{
    for i in [
        "WALLY","BRENDAN","MAY"
        ]{
        if trainer.trainer_name == i.to_string(){
            return true;
        }
    }
    return false;
}
//Pick Torchic -> Rival gets Mudkip
//Pick Treeko -> Rival gets Torchic
//Pick Mudkip -> Rival gets Treeko
pub fn handle_special_trainer(trainer: Trainer, settings: &mut settings::Settings,all_stats: &Vec<pokemon::PokemonStats>,
    starters: &starter_randomization::Starter,rival: &trainers::MayBrendanTeam,wally: &trainers::WallyTeam) -> Trainer{
        match trainer.trainer_name.as_str(){
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
                                println!("Is rival");
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
fn get_random_starter(settings: &mut settings::Settings,all_stats: &Vec<pokemon::PokemonStats>) -> pokemon::Pokemon{
    let mut fake_starter_trainer = Trainer{
        trainer_full_name: "r".to_string(),
        trainer_name: "r".to_string(),
        class: "".to_string(),
        pic: "".to_string(),
        gender: "".to_string(),
        music: "".to_string(),
        double_battle: "".to_string(),
        ai: "".to_string(),
        portrait: "".to_string(),
        pokemon: vec![
            TrainerPokemon{
                iv: 200,
                level: 34,
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
        ai: "".to_string(),
        portrait: "".to_string(),
        pokemon: vec![
            TrainerPokemon{
                iv: 15,
                level: 31,
                species: pokemon::Pokemon::Tropius,
                moves: Vec::new(),
                held_items: "".to_string()
            },
            TrainerPokemon{
                iv: 15,
                level: 32,
                species: pokemon::Pokemon::Ludicolo,
                moves: Vec::new(),
                held_items: "".to_string()
            },
            TrainerPokemon{
                iv: 15,
                level: 32,
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
    println!("Trainer Name: {}", trainer.trainer_full_name);
    trainer.pokemon = match num_battle{
        2 => vec![ //Rustboro
            TrainerPokemon{
                iv: 25,
                level: 13,
                species: scale_pokemon(rival_team.pokemon2.clone(), 13, all_stats, settings).pokemon_id,
                moves: trainers::create_moveset(settings, scale_pokemon(rival_team.pokemon2.clone(), 13, all_stats, &mut settings.clone()).pokemon_id, 13, Vec::new()),
                held_items: trainers::create_held_item(settings, scale_pokemon(rival_team.pokemon2.clone(), 13, all_stats, &mut settings.clone()).pokemon_id, 13, "".to_string())
            },
            TrainerPokemon{
                iv: 25,
                level: 15,
                species: scale_pokemon(starter.clone(), 15, all_stats, settings).pokemon_id,
                moves: trainers::create_moveset(settings, scale_pokemon(starter.clone(), 15, all_stats, &mut settings.clone()).pokemon_id, 15, Vec::new()),
                held_items: trainers::create_held_item(settings, scale_pokemon(starter.clone(), 15, all_stats, &mut settings.clone()).pokemon_id, 15, "".to_string())
            } 
        ],
        3 => vec![ //Route 110 (Hell)
            TrainerPokemon{
                iv: 50,
                level: 18,
                species: scale_pokemon(rival_team.pokemon3.clone(), 18, all_stats, settings).pokemon_id,
                moves: trainers::create_moveset(settings, scale_pokemon(rival_team.pokemon3.clone(), 18, all_stats, &mut settings.clone()).pokemon_id, 18, Vec::new()),
                held_items: trainers::create_held_item(settings, scale_pokemon(rival_team.pokemon3.clone(), 18, all_stats, &mut settings.clone()).pokemon_id, 18, "".to_string())
            },
            TrainerPokemon{
                iv: 50,
                level: 18,
                species: scale_pokemon(rival_team.pokemon2, 18, all_stats, settings).pokemon_id,
                moves: trainers::create_moveset(settings, scale_pokemon(rival_team.pokemon3.clone(), 18, all_stats, &mut settings.clone()).pokemon_id, 18, Vec::new()),
                held_items: trainers::create_held_item(settings, scale_pokemon(rival_team.pokemon3.clone(), 18, all_stats, &mut settings.clone()).pokemon_id, 18, "".to_string())
            },
            TrainerPokemon{
                iv: 100,
                level: 20,
                species: scale_pokemon(starter.clone(), 20, all_stats, settings).pokemon_id,
                moves: trainers::create_moveset(settings, scale_pokemon(starter.clone(), 20, all_stats, &mut settings.clone()).pokemon_id, 20, Vec::new()),
                held_items: trainers::create_held_item(settings, scale_pokemon(starter.clone(), 20, all_stats, &mut settings.clone()).pokemon_id, 20, "".to_string())
            }
        ],
        4 => vec![ //Route 119
            TrainerPokemon{
                iv: 100,
                level: 29,
                species: scale_pokemon(rival_team.pokemon3.clone(), 29, all_stats, settings).pokemon_id,
                moves: trainers::create_moveset(settings, scale_pokemon(rival_team.pokemon3.clone(), 29, all_stats, &mut settings.clone()).pokemon_id, 29, Vec::new()),
                held_items: trainers::create_held_item(settings, scale_pokemon(rival_team.pokemon3.clone(), 29, all_stats, &mut settings.clone()).pokemon_id, 29, "".to_string())
            },
            TrainerPokemon{
                iv: 100,
                level: 29,
                species: scale_pokemon(rival_team.pokemon2, 29, all_stats, settings).pokemon_id,
                moves: trainers::create_moveset(settings, scale_pokemon(rival_team.pokemon3.clone(), 29, all_stats, &mut settings.clone()).pokemon_id, 29, Vec::new()),
                held_items: trainers::create_held_item(settings, scale_pokemon(rival_team.pokemon3.clone(), 29, all_stats, &mut settings.clone()).pokemon_id, 29, "".to_string())
            },
            TrainerPokemon{
                iv: 100,
                level: 31,
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
                species: rival_team.pokemon4.clone(),
                moves: trainers::create_moveset(settings,rival_team.pokemon4.clone(),31,Vec::new()),
                held_items: trainers::create_held_item(settings, rival_team.pokemon4.clone(), 31, "".to_string())
            },
            TrainerPokemon{
                iv: 15,
                level: 32,
                species: rival_team.pokemon3.clone(),
                moves: trainers::create_moveset(settings, rival_team.pokemon3.clone(), 32, Vec::new()),
                held_items: trainers::create_held_item(settings, rival_team.pokemon3.clone(), 32, "".to_string())
            },
            TrainerPokemon{
                iv: 15,
                level: 32,
                species: rival_team.pokemon2.clone(),
                moves: trainers::create_moveset(settings, rival_team.pokemon3.clone(), 32, Vec::new()),
                held_items: trainers::create_held_item(settings, rival_team.pokemon3.clone(), 32, "".to_string())
            },
            TrainerPokemon{
                iv: 15,
                level: 34,
                species: scale_pokemon(starter.clone(), 34, all_stats, settings).pokemon_id,
                moves: trainers::create_moveset(settings, starter.clone(), 34, Vec::new()),
                held_items: trainers::create_held_item(settings, starter.clone(), 34, "".to_string())
            }
        ],
        _ => vec![TrainerPokemon{ //First Battle (Route 103)
            iv: 0,
            level: 5,
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
            class: "Class: Rival".to_string(),
            pic: "Pic: Wally".to_string(),
            gender: "Gender: Male".to_string(),
            music: "Music: Male".to_string(),
            double_battle: "Double Battle: No".to_string(),
            ai: "AI: Check Bad Move / Try To Faint / Check Viability".to_string(),
            portrait: "".to_string(),
            pokemon: vec![
                TrainerPokemon{
                    iv: 30,
                    level: 16,
                    species: scale_pokemon(wally_team.ralt_substitute.clone(),16,all_stats,settings).pokemon_id,
                    moves: trainers::create_moveset(settings,scale_pokemon(wally_team.ralt_substitute,16,all_stats,&mut settings.clone()).pokemon_id,16,Vec::new()),
                    held_items: trainers::create_held_item(settings, scale_pokemon(wally_team.ralt_substitute,16,all_stats,&mut settings.clone()).pokemon_id, 16, "".to_string())
                }
            ]
        },
        2 => Trainer { //Victory Road
            trainer_full_name: rival_name,
            trainer_name: "WALLY".to_string(),
            class: "Class: Rival".to_string(),
            pic: "Pic: Wally".to_string(),
            gender: "Gender: Male".to_string(),
            music: "Music: Male".to_string(),
            double_battle: "Double Battle: No".to_string(),
            ai: "AI: Check Bad Move / Try To Faint / Check Viability".to_string(),
            portrait: "".to_string(),
            pokemon: vec![
                TrainerPokemon{
                    iv: 15,
                    level: 44,
                    species: scale_pokemon(wally_team.pokemon2, 44, all_stats, settings).pokemon_id,
                    moves: trainers::create_moveset(settings,scale_pokemon(wally_team.pokemon2,44,all_stats,&mut settings.clone()).pokemon_id,44,Vec::new()),
                    held_items: trainers::create_held_item(settings, scale_pokemon(wally_team.pokemon2, 44, all_stats, &mut settings.clone()).pokemon_id, 44, "".to_string())
                },
                TrainerPokemon{
                    iv: 15,
                    level: 43,
                    species: scale_pokemon(wally_team.pokemon3, 43, all_stats, settings).pokemon_id,
                    moves: trainers::create_moveset(settings,scale_pokemon(wally_team.pokemon3,43,all_stats,&mut settings.clone()).pokemon_id,43,Vec::new()),
                    held_items: trainers::create_held_item(settings, scale_pokemon(wally_team.pokemon3, 43, all_stats, &mut settings.clone()).pokemon_id, 43, "".to_string())
                },
                TrainerPokemon{
                    iv: 15,
                    level: 44,
                    species: scale_pokemon(wally_team.pokemon4, 44, all_stats, settings).pokemon_id,
                    moves: trainers::create_moveset(settings,scale_pokemon(wally_team.pokemon4,44,all_stats,&mut settings.clone()).pokemon_id,44,Vec::new()),
                    held_items: trainers::create_held_item(settings, scale_pokemon(wally_team.pokemon4, 44, all_stats, &mut settings.clone()).pokemon_id, 44, "".to_string())
                },
                TrainerPokemon{
                    iv: 15,
                    level: 41,
                    species: scale_pokemon(wally_team.pokemon5, 41, all_stats, settings).pokemon_id,
                    moves: trainers::create_moveset(settings,scale_pokemon(wally_team.pokemon5,41,all_stats,&mut settings.clone()).pokemon_id,41,Vec::new()),
                    held_items: trainers::create_held_item(settings, scale_pokemon(wally_team.pokemon4, 41, all_stats, &mut settings.clone()).pokemon_id, 41, "".to_string())
                },
                TrainerPokemon{
                    iv: 15,
                    level: 45,
                    species: scale_pokemon(wally_team.ralt_substitute, 45, all_stats, settings).pokemon_id,
                    moves: trainers::create_moveset(settings,scale_pokemon(wally_team.ralt_substitute,45,all_stats,&mut settings.clone()).pokemon_id,45,Vec::new()),
                    held_items: trainers::create_held_item(settings, scale_pokemon(wally_team.pokemon4, 45, all_stats, &mut settings.clone()).pokemon_id, 45, "".to_string())
                }
            ]
        },
        _ => Trainer {trainer_full_name: "TRAINER_WALLY".to_string(), trainer_name: rival_name,
        class: "Class: Rival".to_string(),
        pic: "Pic: Wally".to_string(),
        gender: "Gender: Male".to_string(),
        music: "Music: Male".to_string(),
        double_battle: "Double Battle: No".to_string(),
        ai: "AI: Check Bad Move / Try To Faint / Check Viability".to_string(),
        portrait: "".to_string(),
        pokemon: vec![TrainerPokemon{ //Fake Battle (Route 103)
            iv: 0,
            level: 5,
            species: scale_pokemon(wally_team.ralt_substitute, 5, all_stats, settings).pokemon_id,
            moves: Vec::new(),
            held_items: "".to_string()
        }]},
    }
}