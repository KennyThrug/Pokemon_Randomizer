use rand::{Rng, rngs::StdRng, SeedableRng};

mod gen3;
mod settings;
mod pokemon;
fn main() {
    println!("Hello, world!");
    gen3::emerald::startup::randomize_pokemon(&mut create_sample_settings());
}

fn create_sample_settings() -> settings::Settings{
    let seed_string = "Default".to_string();
    let bytes: [u8;32] = settings::convert_string_to_seed(seed_string); //HERE
    let mut ret : settings::Settings = settings::Settings{
        seed: "Default".to_string(),
        seed_val: StdRng::from_seed(bytes),
        allow_pokemon_past_generation: true,
        randomize_wild_pokemon: true,
        allow_legends_in_wild_pool: false,
        allow_megas_in_wild_pool: true,
        randomize_trainer_pokemon: true,
        allow_trainer_legendaries: settings::AllowLegendaries::NoLegends,
        allow_leader_legendaries: settings::AllowLegendaries::AceLegend,
        gym_type: settings::GymType::RandomType,
        recieve_pokemon_reward_gym: true,
        randomize_gym_locations: settings::GymLocationRandomization::RandomizeCompletely,
        add_rare_candy: 70,
        add_held_items_later_gens: true,
        items_from_trainers: true,
        important_items_only_from_trainers: true,
        add_master_balls: 10,
        allow_pokeballs: false,
        allow_healing_items: true,
        randomize_hidden_items: false,
        gym_leader_keys: true,
        allow_hm_use: true
    };
    return ret; 
}