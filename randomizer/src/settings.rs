//Contains the settings for all the games, big ass struct probably
//Not all settings are implemented, see README for more information
pub struct Settings{
    //Wild Pokemon Randomization
    allow_pokemon_past_generation: bool,
    randomize_wild_pokemon: bool,
    allow_legends_in_wild_pool: bool,
    //Trainer Randomization
    randomize_trainer_pokemon: bool,
    allow_trainer_legendaries: AllowLegendaries,
    //Gym Leader Randomization
    allow_leader_legendaries: AllowLegendaries,
    gym_type: GymType,
    recieve_pokemon_reward_gym: bool,
    randomize_gym_locations: GymLocationRandomization,
    //Item Randomization
    add_rare_candy: i32, //Number of Rare candies to be added (have default)
    add_held_items_later_gens: bool,
    items_from_trainers: bool,
    important_items_only_from_trainers: bool,
    add_master_balls: i32,
    allow_pokeballs: bool,
    allow_healing_items: bool,
    randomize_hidden_items: bool,
    gym_leader_keys: bool,
    //Other Settings
    allow_hm_use: bool,
}

pub enum AllowLegendaries{
    NoLegends,
    OneLegend,
    AceLegend
}
pub enum GymType{
    CompletelyRandom,
    KeepType,
    RandomType
}
pub enum GymLocationRandomization{
    NoRandomization,
    RandomizeWithinGame,
    RandomizeWithinGeneration,
    RandomizeCompletely,
}

pub fn read_json_for_settings(){

}