use rand::{Rng, rngs::StdRng, SeedableRng};

//Contains the settings for all the games, big ass struct probably
//Not all settings are implemented, see README for more information
pub struct Settings{
    //Seed
    pub seed: String,
    pub seed_val: StdRng,
    //Wild Pokemon Randomization
    pub allow_pokemon_past_generation: bool,
    pub randomize_wild_pokemon: bool,
    pub allow_legends_in_wild_pool: bool,
    pub allow_megas_in_wild_pool: bool,
    //Trainer Randomization
    pub randomize_trainer_pokemon: bool,
    pub allow_trainer_legendaries: AllowLegendaries,
    //Gym Leader Randomization
    pub allow_leader_legendaries: AllowLegendaries,
    pub gym_type: GymType,
    pub recieve_pokemon_reward_gym: bool,
    pub randomize_gym_locations: GymLocationRandomization,
    //Item Randomization
    pub add_rare_candy: i32, //Number of Rare candies to be added (have default)
    pub add_held_items_later_gens: bool,
    pub items_from_trainers: bool,
    pub important_items_only_from_trainers: bool,
    pub add_master_balls: i32,
    pub allow_pokeballs: bool,
    pub allow_healing_items: bool,
    pub randomize_hidden_items: bool,
    pub gym_leader_keys: bool,
    //Evolution Settings
    
    //Other Settings
    pub allow_hm_use: bool,
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

pub fn get_next_seed(lower_bounds: i32, upper_bounds: i32, settings: &mut Settings) -> i32{
    settings.seed_val.gen_range(lower_bounds..upper_bounds)
}
pub fn convert_string_to_seed(string_seed: String) -> [u8;32]{
    let bytes =  string_seed.as_bytes();
    let mut result: [u8;32] = [0;32];
    let mut j : usize = 0; //Index of bytes
    for i in 0..32{
        if bytes.len() == j{
            j = 0;
        }
        result[i] = bytes[j];
        j += 1;
    }
    return result;
}