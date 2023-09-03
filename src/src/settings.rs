use csv::Error;
use rand::{Rng, rngs::StdRng, SeedableRng};

//Contains the settings for all the games, big ass struct probably
//Not all settings are implemented, see README for more information
pub struct Settings{
    //Seed
    pub seed: String,
    pub seed_val: StdRng,
    //Wild Pokemon Randomization
    pub randomize_wild_pokemon: bool,
    pub allow_pokemon_past_generation: bool,
    pub allow_legends_in_wild_pool: bool,
    pub allow_megas_in_wild_pool: bool,
    //Trainer Randomization
    pub randomize_trainer_pokemon: bool,
    pub trainers_scale: bool,
    pub allow_trainer_legendaries: AllowLegendaries,
    //Gym Leader Randomization
    pub allow_leader_legendaries: AllowLegendaries,
    pub gym_type: GymType,
    pub recieve_pokemon_reward_gym: bool,
    pub randomize_gym_locations: GymLocationRandomization,
    //Item Randomization
    pub add_rare_candy: i32, //Number of Rare candies to be added (have default)
    pub add_held_items: bool,
    pub add_held_items_later_gens: bool,
    pub items_from_trainers: bool,
    pub important_items_only_from_trainers: bool,
    pub add_pokeballs: i32,
    pub allow_pokeballs_from_store: bool,
    pub make_balls_reusable: bool,
    
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
    AceLegend,
    Allow
}
pub enum WildLegends{
    NoLegends, //Or Always Legends
    SometimesLegends, //Tries not to have legends, but sometimes fails
    AllowLegends //Allows Legends in same pool as regular pokemon
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

pub fn read_json_for_settings(json_string: String) -> Result<Settings,Error>{
    let parsed_json = json::parse(&json_string).unwrap();
    println!("{}",parsed_json["seed"]);
    let bytes: [u8;32] = convert_string_to_seed(parsed_json["seed"].to_string()); //HERE
    Ok(Settings{
        seed: parsed_json["seed"].to_string(),
        seed_val: StdRng::from_seed(bytes),
        //Wild Pokemon
        randomize_wild_pokemon: parsed_json["randomize_wild_pokemon"].as_bool().unwrap(),
        allow_pokemon_past_generation: parsed_json["allow_pokemon_past_generation"].as_bool().unwrap(),
        allow_legends_in_wild_pool: parsed_json["allow_legends_in_wild_pool"].as_bool().unwrap(),
        allow_megas_in_wild_pool: parsed_json["allow_megas_in_wild_pool"].as_bool().unwrap(),
        //Trainer Randomization
        randomize_trainer_pokemon: parsed_json["randomize_trainer_pokemon"].as_bool().unwrap(),
        trainers_scale: parsed_json["trainers_scale"].as_bool().unwrap(),
        allow_trainer_legendaries: convert_string_to_allow_legendaries(parsed_json["allow_trainer_legendaries"].to_string()),
        //Gym Leader Randomization
        allow_leader_legendaries: convert_string_to_allow_legendaries(parsed_json["allow_leader_legendaries"].to_string()),
        gym_type: convert_string_to_gym_type(parsed_json["gym_type"].to_string()),
        recieve_pokemon_reward_gym: parsed_json["recieve_pokemon_reward_gym"].as_bool().unwrap(),
        randomize_gym_locations: convert_string_to_gym_location(parsed_json["randomize_gym_locations"].to_string()),
        //Item Randomization
        add_rare_candy: parsed_json["add_rare_candy"].as_i32().unwrap(),
        add_held_items: parsed_json["add_held_items"].as_bool().unwrap(),
        add_held_items_later_gens: parsed_json["add_held_items_later_gens"].as_bool().unwrap(),
        items_from_trainers: parsed_json["items_from_trainers"].as_bool().unwrap(),
        important_items_only_from_trainers: parsed_json["important_items_only_from_trainers"].as_bool().unwrap(),
        add_pokeballs: parsed_json["add_pokeballs"].as_i32().unwrap(),
        allow_pokeballs_from_store: parsed_json["allow_pokeballs_from_store"].as_bool().unwrap(),
        make_balls_reusable: parsed_json["make_balls_reusable"].as_bool().unwrap(),
        allow_healing_items: parsed_json["allow_healing_items"].as_bool().unwrap(),
        randomize_hidden_items: parsed_json["randomize_hidden_items"].as_bool().unwrap(),
        gym_leader_keys: parsed_json["gym_leader_keys"].as_bool().unwrap(),
        //Evolution Settings
        //Other Settings
        allow_hm_use: parsed_json["allow_hm_use"].as_bool().unwrap()
    })
}
fn convert_string_to_allow_legendaries(string: String) -> AllowLegendaries{
    match string.as_str(){
        "NoLegends" => AllowLegendaries::NoLegends,
        "OneLegend" => AllowLegendaries::OneLegend,
        "AceLegend" => AllowLegendaries::AceLegend,
        "Allow" => AllowLegendaries::Allow,
        _ => AllowLegendaries::NoLegends
    }
}
fn convert_string_to_gym_location(string: String) -> GymLocationRandomization{
    match string.as_str(){
        "NoRanomization" => GymLocationRandomization::NoRandomization,
        "RandomizeWithinGame" => GymLocationRandomization::RandomizeWithinGame,
        "RandomizeWithinGeneration" => GymLocationRandomization::RandomizeWithinGeneration,
        "RandomizeCompletely" => GymLocationRandomization::RandomizeCompletely,
        _ => GymLocationRandomization::NoRandomization
    }
}
fn convert_string_to_gym_type(string: String) -> GymType{
    match string.as_str(){
        "CompletelyRandom" => GymType::CompletelyRandom,
        "KeepType" => GymType::KeepType,
        "Random_Type" => GymType::RandomType,
        _ => GymType::CompletelyRandom
    }
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