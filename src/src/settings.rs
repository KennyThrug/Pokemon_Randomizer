use csv::Error;
use rand::{Rng, rngs::StdRng, SeedableRng};

//Contains the settings for all the games, big ass struct probably
//Not all settings are implemented, see README for more information
#[derive(Clone)]
pub struct Settings{
    //Seed
    pub seed: String,
    pub seed_val: StdRng,
    //Wild Pokemon Randomization
    pub randomize_wild_pokemon: bool,
    pub randomize_starter_pokemon: bool,
    pub allow_starter_legendary: WildLegends,
    pub scale_starter: bool,
    pub allow_pokemon_future_generation: bool,
    pub scale_wild_pokemon: bool,
    pub allow_legends_in_wild_pool: WildLegends,
    pub allow_megas_in_wild_pool: WildLegends,
    pub force_legendaries_to_legendaries: LegendRarity,
    //Trainer Randomization
    pub randomize_trainer_pokemon: bool,
    pub trainers_scale: bool,
    pub allow_trainer_legendaries: AllowLegendaries,
    pub trainer_legendaries_rare: bool,
    //Rival Randomization
    pub rival_keeps_starter: bool,
    pub rival_consistent_team: bool,
    pub wally_keeps_starter: bool,
    //Gym Leader Randomization
    pub allow_leader_legendaries: AllowLegendaries,
    pub gym_type: GymType,
    pub recieve_pokemon_reward_gym: bool,
    pub randomize_gym_locations: GymLocationRandomization,
    //Item Randomization
    pub randomize_items: bool,
    pub add_rare_candy: i32, //Number of Rare candies to be added (have default)
    pub add_held_items: bool,
    pub add_held_items_later_gens: bool,
    pub items_from_trainers: bool,
    pub add_pokeballs: i32,
    pub allow_pokeballs_from_store: bool,
    pub make_pokeballs_masterballs: bool,
    pub randomize_stores: bool,
    pub randomize_hms: bool,
    pub number_hms: i32,
    pub randomize_key_items: bool,
    
    pub allow_healing_items: bool,
    pub randomize_hidden_items: bool,
    pub gym_leader_keys: bool,
    //Evolution Settings
    
    //Other Settings
    pub allow_hm_use: bool,
}

#[derive(PartialEq,Clone)]
pub enum LegendRarity{
    AlwaysLegendary,
    SometimesLegendary,//Will allow it to be legendary, but wont force it
    LikelyLegendary,//Will ignore 1 in 100 non-legendaries, and will try to force a pokemon to be legendary, but can fail
    NotLegendary
}

#[derive(PartialEq,Clone)]
pub enum AllowLegendaries{
    NoLegends,
    OneLegend,
    AceLegend,
    Allow,
}
#[derive(PartialEq)]
#[derive(Clone, Copy)]
pub enum WildLegends{
    NoLegends, //Or Always Legends
    SometimesLegends, //Tries not to have legends, but sometimes fails
    AllowLegends //Allows Legends in same pool as regular pokemon
}
#[derive(Clone)]
pub enum GymType{
    CompletelyRandom,
    KeepType,
    RandomType
}
#[derive(Clone)]
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
        randomize_starter_pokemon: parsed_json["randomize_starter_pokemon"].as_bool().unwrap(),
        allow_starter_legendary: convert_string_to_wild_legends(parsed_json["allow_starter_legendary"].to_string()),
        scale_starter: parsed_json["scale_starter"].as_bool().unwrap(),
        allow_pokemon_future_generation: parsed_json["allow_pokemon_future_generation"].as_bool().unwrap(),
        scale_wild_pokemon: parsed_json["scale_wild_pokemon"].as_bool().unwrap(),
        allow_legends_in_wild_pool: convert_string_to_wild_legends(parsed_json["allow_legends_in_wild_pool"].to_string()),
        allow_megas_in_wild_pool: convert_string_to_wild_legends(parsed_json["allow_megas_in_wild_pool"].to_string()),
        force_legendaries_to_legendaries: convert_string_to_legend_rarity(parsed_json["force_legendaries_to_legendaries"].to_string()),
        //Trainer Randomization
        randomize_trainer_pokemon: parsed_json["randomize_trainer_pokemon"].as_bool().unwrap(),
        trainers_scale: parsed_json["trainers_scale"].as_bool().unwrap(),
        allow_trainer_legendaries: convert_string_to_allow_legendaries(parsed_json["allow_trainer_legendaries"].to_string()),
        trainer_legendaries_rare: parsed_json["trainer_legendaries_rare"].as_bool().unwrap(),
        //Rival Randomization
        rival_keeps_starter: parsed_json["rival_keeps_starter"].as_bool().unwrap(),
        rival_consistent_team: parsed_json["rival_consistent_team"].as_bool().unwrap(),
        wally_keeps_starter: parsed_json["wally_keeps_starter"].as_bool().unwrap(),
        //Gym Leader Randomization
        allow_leader_legendaries: convert_string_to_allow_legendaries(parsed_json["allow_leader_legendaries"].to_string()),
        gym_type: convert_string_to_gym_type(parsed_json["gym_type"].to_string()),
        recieve_pokemon_reward_gym: parsed_json["recieve_pokemon_reward_gym"].as_bool().unwrap(),
        randomize_gym_locations: convert_string_to_gym_location(parsed_json["randomize_gym_locations"].to_string()),
        //Item Randomization
        randomize_items: parsed_json["randomize_items"].as_bool().unwrap(),
        add_rare_candy: parsed_json["add_rare_candy"].to_string().parse().unwrap(),
        add_held_items: parsed_json["add_held_items"].as_bool().unwrap(),
        add_held_items_later_gens: parsed_json["add_held_items_later_gens"].as_bool().unwrap(),
        items_from_trainers: parsed_json["items_from_trainers"].as_bool().unwrap(),
        add_pokeballs: parsed_json["add_pokeballs"].to_string().parse().unwrap(),
        allow_pokeballs_from_store: parsed_json["allow_pokeballs_from_store"].as_bool().unwrap(),
        make_pokeballs_masterballs: parsed_json["make_pokeballs_masterballs"].as_bool().unwrap(),
        randomize_stores: parsed_json["randomize_stores"].as_bool().unwrap(),
        allow_healing_items: parsed_json["allow_healing_items"].as_bool().unwrap(),
        randomize_hidden_items: parsed_json["randomize_hidden_items"].as_bool().unwrap(),
        gym_leader_keys: parsed_json["gym_leader_keys"].as_bool().unwrap(),
        randomize_hms: parsed_json["randomize_hms"].as_bool().unwrap(),
        number_hms: parsed_json["numberhms"].to_string().parse().unwrap(),
        randomize_key_items: parsed_json["randomize_key_items"].as_bool().unwrap(),
        //Evolution Settings
        //Other Settings
        allow_hm_use: parsed_json["allow_hm_use"].as_bool().unwrap()
    })
}
fn convert_string_to_wild_legends(string: String) -> WildLegends{
    match string.as_str(){
        "NoLegends" => WildLegends::NoLegends,
        "SometimesLegends" => WildLegends::SometimesLegends,
        "AllowLegends" => WildLegends::AllowLegends,
        _ => WildLegends::AllowLegends
    }
}
fn convert_string_to_allow_legendaries(string: String) -> AllowLegendaries{
    match string.as_str(){
        "NoLegends" => AllowLegendaries::NoLegends,
        "OneLegend" => AllowLegendaries::OneLegend,
        "AceLegend" => AllowLegendaries::AceLegend,
        "AllowLegends" => AllowLegendaries::Allow,
        _ => AllowLegendaries::NoLegends
    }
}
fn convert_string_to_legend_rarity(string: String) -> LegendRarity{
    match string.as_str(){
        "AlwaysLegendary" => LegendRarity::AlwaysLegendary,
        "SometimesLegendary" => LegendRarity::SometimesLegendary,
        "LikelyLegendary" => LegendRarity::LikelyLegendary,
        _ => LegendRarity::NotLegendary
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