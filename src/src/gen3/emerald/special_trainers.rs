use crate::src::{gen3::trainers::{Trainer, self, TrainerPokemon, scale_pokemon}, settings, pokemon};



pub fn check_if_special_trainer(trainer: Trainer) -> bool{
    for i in ["d","s"]{
        if trainer.trainer_name == i.to_string(){
            return true;
        }
    }
    return false;
}
pub fn handle_special_trainer(trainer: Trainer, settings: &mut settings::Settings,all_stats: &Vec<pokemon::PokemonStats>,
    starters: &trainers::Starter,rival: &trainers::MayBrendanTeam,wally: &trainers::WallyTeam) -> Trainer{
    return trainer;
}

fn handle_rival(settings: &mut settings::Settings,all_stats: &Vec<pokemon::PokemonStats>,rival_name: String,num_battle: i16,starter: &pokemon::Pokemon,rival_team: &trainers::MayBrendanTeam) -> Trainer{
    match num_battle{
        _ => Trainer { trainer_name: rival_name, pokemon: vec![TrainerPokemon{ //First Battle (Route 103)
            iv: 0,
            level: 5,
            species: scale_pokemon(starter.clone(), 5, all_stats, settings).pokemon_id,
            moves: Vec::new(),
            held_items: "".to_string()
        }]},
    }
}