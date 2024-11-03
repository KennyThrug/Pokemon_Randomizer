use std::fs;
use crate::src::{settings, pokemon::{PokemonStats, get_pokemon_data}};
use json::{self, JsonValue::Null};

use crate::src::pokemon;
use crate::src::gen3::starter_randomization;

use super::emerald::{static_pokemon, special_trainers::{handle_special_trainer, self}};

#[derive(Clone)]
pub struct Trainer{
    pub trainer_full_name: String, //The Trainer name in code, such as TRAINER_GRUNT_SEAFLOOR_CAVERN_1
    pub trainer_name: String, //Trainer name in game, such as Grunt
    pub class: String,
    pub pic: String,
    pub gender: String,
    pub music: String,
    pub items: String,
    pub double_battle: String,
    pub ai: String,
    pub portrait: String,
    pub pokemon: Vec<TrainerPokemon>
}
#[derive(Clone)]
pub struct TrainerPokemon{
    pub iv: i32,
    pub species: pokemon::Pokemon,
    pub level: i32,
    pub extra_scripts: String,
    pub moves: Vec<String>,
    pub held_items: String,
}
pub fn write_trainers_to_file(filename:String,trainers: Vec<Trainer>,all_stats: &Vec<pokemon::PokemonStats>,settings: &mut settings::Settings){
    let mut file_text = "".to_string();
    for cur_trainer in trainers{
        //println!("{}",cur_trainer.trainer_name);
        // let has_held_item = cur_trainer.pokemon[0].held_items.len() != 0;
        // let has_custom_moves = cur_trainer.pokemon[0].moves.len() != 0;
        //println!("item: {} moves: {}",has_held_item,has_custom_moves);

        //TODO format this to be in the same format as src/data/trainers.party

        let mut all_pokemon : String = "".to_string();
        for cur_pokemon in cur_trainer.pokemon{
            all_pokemon.push_str(get_pokemon(cur_pokemon,all_stats,settings).as_str());
        }

        // println!("This: {}", all_pokemon);
        file_text += format!("{}",cur_trainer.trainer_full_name).as_str();
        file_text += format!("\nName: {}",cur_trainer.trainer_name).as_str();
        file_text += format!("\nClass: {}",cur_trainer.class).as_str();
        file_text += format!("\nPic: {}",cur_trainer.pic).as_str();
        file_text += format!("\nGender: {}",cur_trainer.gender).as_str();
        if cur_trainer.items != ""{
            file_text += format!("\nItems: {}",cur_trainer.items).as_str();
        }
        file_text += format!("\nMusic: {}",cur_trainer.music).as_str();
        file_text += format!("\nDouble Battle: {}",cur_trainer.double_battle).as_str();
        if cur_trainer.ai != ""{
            file_text += format!("\nAI: {}",cur_trainer.ai).as_str();
        }
        if cur_trainer.portrait != ""{
            file_text += format!("\nPortrait: {}",cur_trainer.portrait).as_str();
        }
        file_text += format!("\n\n{}",all_pokemon).as_str();
    }
    //println!("text: {}",file_text);
    fs::write(filename,file_text).expect("Could not write trainer data");
}

fn read_all_trainers(filename: String,all_stats: &Vec<pokemon::PokemonStats>) -> Vec<Trainer>{
    let data = fs::read_to_string(filename).expect("unable to read file");
    let mut all_trainers: Vec<Trainer> = Vec::new();

    let mut cur_trainer = Trainer{
        trainer_full_name: "".to_string(),
        trainer_name: "".to_string(),
        class: "".to_string(),
        pic: "".to_string(),
        gender: "".to_string(),
        music: "".to_string(),
        items: "".to_string(),
        double_battle: "".to_string(),
        ai: "".to_string(),
        portrait: "".to_string(),
        pokemon: Vec::new()
    };
    let mut cur_pokemon = TrainerPokemon{
        iv : 0,
        species : pokemon::Pokemon::None,
        level : 5,
        extra_scripts: "".to_string(),
        moves : Vec::new(),
        held_items: "".to_string()
    };
    for i in data.lines(){
        //Split against :
        let mut cat : Vec<&str> = i.split(':').collect();
        let mut second_half_temp = "".to_string();
        for i in 0..cat.len(){
            cat[i] = cat[i].trim();
        }
        if i != "" && i.to_string().chars().nth(0).unwrap() == '='{
            if cur_trainer.trainer_full_name != ""{
                all_trainers.push(cur_trainer);
            }
            cur_trainer = Trainer{
                trainer_full_name: "".to_string(),
                trainer_name: "".to_string(),
                class: "".to_string(),
                pic: "".to_string(),
                gender: "".to_string(),
                music: "".to_string(),
                items: "".to_string(),
                double_battle: "".to_string(),
                ai: "".to_string(),
                portrait: "".to_string(),
                pokemon: Vec::new()
            };
            cur_trainer.trainer_full_name = i.to_string()
        }
        else if cat[0] == "Name"{
            cur_trainer.trainer_name = cat[1].to_string();
        }
        else if cat[0] == "Class"{
            cur_trainer.class = cat[1].to_string();
        }
        else if cat[0] == "Pic"{
            cur_trainer.pic = cat[1].to_string();
        }
        else if cat[0] == "Gender"{
            cur_trainer.gender = cat[1].to_string();
        }
        else if cat[0] == "Music"{
            cur_trainer.music = cat[1].to_string();
        }
        else if cat[0] == "Items"{
            cur_trainer.items = cat[1].to_string();
        }
        else if cat[0] == "Double Battle"{
            cur_trainer.double_battle = cat[1].to_string();
        }
        else if cat[0] == "AI"{
            cur_trainer.ai = cat[1].to_string();
        }
        else if cat[0] == "Mugshot"{
            cur_trainer.ai.push_str(format!("\n{}",i).as_str());
        }
        else if i == ""{
            //Add pokemon to list of pokemon if needed
            if cur_pokemon.species != pokemon::Pokemon::None{
                cur_trainer.pokemon.push(cur_pokemon);
            }
            //Start new Pokemon
            cur_pokemon = TrainerPokemon{
                iv : 0,
                species : pokemon::Pokemon::None,
                level : 5,
                extra_scripts: "".to_string(),
                moves : Vec::new(),
                held_items: "".to_string()
            };
        }
        //Get Pokemon and stuff
        else if cur_pokemon.species == pokemon::Pokemon::None{
            let cur_pokemon_name : Vec<&str> = i.split('@').collect();
            if cur_pokemon_name.len() > 1{
                println!("Name: {} Other: {}",cur_pokemon_name[0],cur_pokemon_name[1]);
                cur_pokemon.held_items = cur_pokemon_name[1].to_string();
            }
            cur_pokemon.species = pokemon::get_pokemon_from_name(cur_pokemon_name[0].trim().to_string(),all_stats)
        }
        else if cat[0] == "Level"{
            cur_pokemon.level = i.to_string()[7..].parse::<i32>().unwrap();
        }
        else if cat[0] == "IVs"{
            cur_pokemon.iv = 0;
        }
        else if i != "" && i.to_string().chars().nth(0).unwrap() == '-'{
            cur_pokemon.moves.push(i.to_string()[2..].to_string());
        }
    }
    //Read JSON file and put data in data
    return all_trainers;
}

pub fn get_pokemon(pokemon: TrainerPokemon,all_stats: &Vec<pokemon::PokemonStats>,settings: &mut settings::Settings) -> String{
    let mut formatted_moves : String = "".to_string();

    //Final Randomization for sub species
    let mut pre_formatted_name = last_minute_pokemon_name_changes(pokemon.species,all_stats,settings);
    if pre_formatted_name == ""{
        println!("Test");
        pre_formatted_name = pokemon::pokemon_to_formatted_name(pokemon.species,all_stats);
    }
    else{
        println!("Test2");
        pre_formatted_name = pokemon::format_pokemon_name(pre_formatted_name.clone());
    }
    let mut formatted_name : String = if pokemon.held_items == ""{
        pre_formatted_name
    }
    else{
        format!("{} @ {}",pre_formatted_name,pokemon.held_items)
    };
    for i in pokemon.moves{
        formatted_moves.push_str(format!("- {}\n",i).as_str());
    }
    return format!("{pkmn_name}
Level: {level}{extra_script}
IVs: {iv} HP / {iv} Atk / {iv} Def / {iv} SpA / {iv} SpD / {iv} Spe
{moves}
",pkmn_name=formatted_name,level=pokemon.level,
iv=pokemon.iv,moves=formatted_moves,extra_script=pokemon.extra_scripts);
}

pub fn shuffle_trainers(settings: &mut settings::Settings,all_stats: &Vec<pokemon::PokemonStats>,trainer_parties_read_filename: String,trainer_parties_write_filename: String,starters: starter_randomization::Starter){
    let mut trainer_data = read_all_trainers(trainer_parties_read_filename,all_stats);
    let (rival_team,wally_team) = create_rival_teams(settings, all_stats);
    static_pokemon::randomize_static_pokemon(settings, all_stats, &rival_team, &wally_team);
    let gym_types = special_trainers::randomize_gym_types(13,settings);
    for i in 0..trainer_data.len(){
        if special_trainers::check_if_special_trainer(trainer_data[i].clone()){
            trainer_data[i] = handle_special_trainer(trainer_data[i].clone(), settings, all_stats,&starters,&rival_team,&wally_team,gym_types.clone());
        }
        else{//Regular Trainers
            trainer_data[i] = get_random_trainer(trainer_data[i].clone(), settings, all_stats)
        }
    }
    //println!("len: {}",trainer_data.len());
    write_trainers_to_file(trainer_parties_write_filename, trainer_data,all_stats,settings);
}

pub fn get_random_trainer(trainer: Trainer, settings: &mut settings::Settings,all_stats: &Vec<pokemon::PokemonStats>) -> Trainer{
    let mut trainer_pkmn: Vec<TrainerPokemon> = Vec::new();
    let mut has_legend = false;
    for cur_pkmn in trainer.pokemon{
        let next_pkmn = get_random_pokemon_for_trainer(trainer.trainer_name.clone(), &cur_pkmn,all_stats,settings,
                if settings.allow_trainer_legendaries == settings::AllowLegendaries::NoLegends ||
                     (settings.allow_trainer_legendaries == settings::AllowLegendaries::OneLegend && has_legend ||
                    settings.allow_trainer_legendaries == settings::AllowLegendaries::AceLegend && has_legend)
                        {false}else{true});
        if get_pokemon_data(next_pkmn.species, all_stats).status == pokemon::LegendStatus::Legendary || get_pokemon_data(next_pkmn.species, all_stats).status == pokemon::LegendStatus::LegendMega{
            has_legend = true;
        }
        trainer_pkmn.push(next_pkmn)
    }
    if settings.allow_trainer_legendaries == settings::AllowLegendaries::AceLegend && has_legend{
        for i in 0..trainer_pkmn.len(){
            println!("LL: {}",trainer_pkmn[i].species as i32);
            if get_pokemon_data(trainer_pkmn[i].species,all_stats).status == pokemon::LegendStatus::Legendary || 
            get_pokemon_data(trainer_pkmn[i].species,all_stats).status == pokemon::LegendStatus::Legendary{
                //Switch Legend with Ace Pokemon
                let temp = trainer_pkmn[i].species;
                let last_pos = trainer_pkmn.len()-1;
                trainer_pkmn[i].species = trainer_pkmn[last_pos].species;
                trainer_pkmn[last_pos].species = temp;
            }
        }
    }
    return Trainer{
        trainer_full_name: trainer.trainer_full_name,
        trainer_name: trainer.trainer_name,
        class: trainer.class,
        pic: trainer.pic,
        gender: trainer.gender,
        music: trainer.music,
        items: "".to_string(),
        double_battle: trainer.double_battle,
        ai: trainer.ai,
        portrait: "".to_string(),
        pokemon: trainer_pkmn
    };
}

pub fn get_random_pokemon_for_trainer(trainer_name: String, pokemon: &TrainerPokemon,pokemon_data: &Vec<pokemon::PokemonStats>,settings: &mut settings::Settings,can_be_legend: bool) -> TrainerPokemon{
    if !settings.randomize_trainer_pokemon{
        return pokemon.clone();
    }
    let rand_val = settings::get_next_seed(0, (pokemon_data.len() as i32), settings);
    let new_pokemon = scale_pokemon(pokemon_data[rand_val as usize].clone().pokemon_id,pokemon.level,pokemon_data,settings);
    if !can_be_legend && (new_pokemon.status == pokemon::LegendStatus::Legendary || new_pokemon.status == pokemon::LegendStatus::LegendMega){
        return get_random_pokemon_for_trainer(trainer_name, pokemon, pokemon_data, settings,can_be_legend);
    }
    if settings.trainer_legendaries_rare && (new_pokemon.status == pokemon::LegendStatus::Legendary || new_pokemon.status == pokemon::LegendStatus::LegendMega){
        if settings::get_next_seed(0, 20, settings) != 0{
            return get_random_pokemon_for_trainer(trainer_name, pokemon, pokemon_data, settings, can_be_legend)
        }
    }

    TrainerPokemon {
        iv: pokemon.iv,
        species: new_pokemon.pokemon_id.clone(),
        level: pokemon.level,
        extra_scripts: "".to_string(),
        moves: create_moveset(settings,new_pokemon.pokemon_id,pokemon.level,pokemon.moves.clone()),
        held_items: create_held_item(settings,new_pokemon.pokemon_id,pokemon.level,pokemon.held_items.clone(),false,pokemon_data),
        // gimmick: None,
        // gimmick_string: "".to_string()
    }
}

pub fn scale_pokemon(pokemon: pokemon::Pokemon,level: i32,all_stats: &Vec<pokemon::PokemonStats>,settings: &mut settings::Settings) -> PokemonStats{
    let stats = get_pokemon_data(pokemon, all_stats).clone();
    if !settings.trainers_scale{
        return stats;
    }
    if get_pokemon_data(pokemon, all_stats).min_level > level as i16{
        if get_pokemon_data(pokemon,all_stats).evolve_from == pokemon::Pokemon::None{
            println!("Failsafe triggered 1, pokemon= {}, level = {}",pokemon as i32,level);
            return get_pokemon_data(pokemon,all_stats);
        }
        return scale_pokemon(get_pokemon_data(pokemon,all_stats).evolve_from, level, all_stats, settings);
    }
    for cur_evolution in randomize_next_evolutions(get_pokemon_data(pokemon, all_stats).evolve_into.clone(),settings){
        if cur_evolution == pokemon::Pokemon::None{
            println!("Failsafe triggered 2, pokemon= {}, level = {}",pokemon as i32,level);
            return get_pokemon_data(pokemon,all_stats);
        }
        if get_pokemon_data(cur_evolution, all_stats).min_level <= level as i16{
            return scale_pokemon(cur_evolution, level, all_stats, settings);
        }
    }
    return stats;
}
//Warning: Do not use this on a non-copied Vector
fn randomize_next_evolutions(mut next_evolutions: Vec<pokemon::Pokemon>,settings: &mut settings::Settings) -> Vec<pokemon::Pokemon>{
    let mut return_values: Vec<pokemon::Pokemon> = Vec::new();
    while next_evolutions.len() > 0 {
        return_values.push(next_evolutions.remove(settings::get_next_seed(0, next_evolutions.len() as i32, settings) as usize));
    }
    return return_values;
}

pub fn create_moveset(settings: &mut settings::Settings,pokemon: pokemon::Pokemon,level: i32,old_moveset: Vec<String>) -> Vec<String>{
    //Placeholder for now, functionality will be added later
    return Vec::new();
}
pub fn create_held_item(settings: &mut settings::Settings,pokemon: pokemon::Pokemon,level: i32,old_item: String,gym_leader: bool,pokemon_data: &Vec<pokemon::PokemonStats>) -> String{
    //Placeholder for now, functionality will be added later
    let mut item = get_mega_stone(pokemon);
    if item != "" {return item;}

    let mut rand_val = settings::get_next_seed(0, 100, settings);
    //Have a higher chance for gym leaders (and rivals) to have items
    if rand_val >= level * if gym_leader{2}else{1}{
        return "".to_string();
    }
    if old_item != "" && rand_val <= level / 2{
        return old_item;
    }
    rand_val = settings::get_next_seed(0, 3, settings);
    item = get_mon_specific_item(pokemon,settings);
    if item != "" && rand_val != 0{return item;} // 1/3 chance to not have a mon specific item, to spice things up
    rand_val = settings::get_next_seed(0, 131, settings);
    return match rand_val{
        0 => get_gem(pokemon,pokemon_data,settings),
        1 => get_type_boosting_item(pokemon,pokemon_data,settings),
        2 => "ITEM_CHOICE_BAND".to_string(),
        3 => "ITEM_CHOICE_SPECS".to_string(),
        4 => "ITEM_CHOICE_SCARF".to_string(),
        5 => "ITEM_FLAME_ORB".to_string(),
        6 => "ITEM_TOXIC_ORB".to_string(),
        7 => "ITEM_DAMP_ROCK".to_string(),
        8 => "ITEM_HEAT_ROCK".to_string(),
        9 => "ITEM_SMOOTH_ROCK".to_string(),
        10 => "ITEM_ICY_ROCK".to_string(),
        11 => "ITEM_ELECTRIC_SEED".to_string(),
        12 => "ITEM_PSYCHIC_SEED".to_string(),
        13 => "ITEM_MISTY_SEED".to_string(),
        14 => "ITEM_GRASSY_SEED".to_string(),
        15 => "ITEM_ABSORB_BULB".to_string(),
        16 => "ITEM_CELL_BATTERY".to_string(),
        17 => "ITEM_LUMINOUS_MOSS".to_string(),
        18 => "ITEM_SNOWBALL".to_string(),
        19 => "ITEM_BRIGHT_POWDER".to_string(),
        20 => "ITEM_WHITE_HERB".to_string(),
        21 => "ITEM_EXP_SHARE".to_string(),
        22 => "ITEM_QUICK_CLAW".to_string(),
        23 => "ITEM_SOOTHE_BELL".to_string(),
        24 => "ITEM_MENTAL_HERB".to_string(),
        25 => "ITEM_KINGS_ROCK".to_string(),
        26 => "ITEM_AMULET_COIN".to_string(),
        27 => "ITEM_CLEANSE_TAG".to_string(),
        28 => "ITEM_SMOKE_BALL".to_string(),
        29 => "ITEM_FOCUS_BAND".to_string(),
        30 => "ITEM_LUCKY_EGG".to_string(),
        31 => "ITEM_SCOPE_LENS".to_string(),
        32 => "ITEM_LEFTOVERS".to_string(),
        33 => "ITEM_SHELL_BELL".to_string(),
        34 => "ITEM_WIDE_LENS".to_string(),
        35 => "ITEM_MUSCLE_BAND".to_string(),
        36 => "ITEM_WISE_GLASSES".to_string(),
        37 => "ITEM_EXPERT_BELT".to_string(),
        38 => "ITEM_LIGHT_CLAY".to_string(),
        39 => "ITEM_LIFE_ORB".to_string(),
        40 => "ITEM_POWER_HERB".to_string(),
        41 => "ITEM_FOCUS_SASH".to_string(),
        42 => "ITEM_ZOOM_LENS".to_string(),
        43 => "ITEM_METRONOME".to_string(),
        44 => "ITEM_IRON_BALL".to_string(),
        45 => "ITEM_LAGGING_TAIL".to_string(),
        46 => "ITEM_DESTINY_KNOT".to_string(),
        47 => "ITEM_BLACK_SLUDGE".to_string(),
        48 => "ITEM_GRIP_CLAW".to_string(),
        49 => "ITEM_STICKY_BARB".to_string(),
        50 => "ITEM_SHED_SHELL".to_string(),
        51 => "ITEM_BIG_ROOT".to_string(),
        52 => "ITEM_RAZOR_CLAW".to_string(),
        53 => "ITEM_RAZOR_FANG".to_string(),
        54 => "ITEM_EVIOLITE".to_string(),
        55 => "ITEM_FLOAT_STONE".to_string(),
        56 => "ITEM_ROCKY_HELMET".to_string(),
        57 => "ITEM_AIR_BALLOON".to_string(),
        58 => "ITEM_RED_CARD".to_string(),
        59 => "ITEM_RING_TARGET".to_string(),
        60 => "ITEM_BINDING_BAND".to_string(),
        61 => "ITEM_EJECT_BUTTON".to_string(),
        62 => "ITEM_WEAKNESS_POLICY".to_string(),
        63 => "ITEM_ASSAULT_VEST".to_string(),
        64 => "ITEM_SAFETY_GOGGLES".to_string(),
        65 => "ITEM_ADRENALINE_ORB".to_string(),
        66 => "ITEM_TERRAIN_EXTENDER".to_string(),
        67 => "ITEM_PROTECTIVE_PADS".to_string(),
        68 => "ITEM_THROAT_SPRAY".to_string(),
        69 => "ITEM_EJECT_PACK".to_string(),
        70 => "ITEM_HEAVY_DUTY_BOOTS".to_string(),
        71 => "ITEM_BLUNDER_POLICY".to_string(),
        72 => "ITEM_ROOM_SERVICE".to_string(),
        73 => "ITEM_UTILITY_UMBRELLA".to_string(),
        74 => "ITEM_CHERI_BERRY".to_string(),
        75 => "ITEM_CHESTO_BERRY".to_string(),
        76 => "ITEM_PECHA_BERRY".to_string(),
        77 => "ITEM_RAWST_BERRY".to_string(),
        78 => "ITEM_ASPEAR_BERRY".to_string(),
        79 => "ITEM_LEPPA_BERRY".to_string(),
        80 => "ITEM_ORAN_BERRY".to_string(),
        81 => "ITEM_PERSIM_BERRY".to_string(),
        82 => "ITEM_LUM_BERRY".to_string(),
        83 => "ITEM_SITRUS_BERRY".to_string(),
        84 => "ITEM_FIGY_BERRY".to_string(),
        85 => "ITEM_WIKI_BERRY".to_string(),
        86 => "ITEM_MAGO_BERRY".to_string(),
        87 => "ITEM_AGUAV_BERRY".to_string(),
        88 => "ITEM_IAPAPA_BERRY".to_string(),
        89 => "ITEM_RAZZ_BERRY".to_string(),
        90 => "ITEM_BLUK_BERRY".to_string(),
        91 => "ITEM_NANAB_BERRY".to_string(),
        92 => "ITEM_WEPEAR_BERRY".to_string(),
        93 => "ITEM_PINAP_BERRY".to_string(),
        94 => "ITEM_POMEG_BERRY".to_string(),
        95 => "ITEM_KELPSY_BERRY".to_string(),
        96 => "ITEM_QUALOT_BERRY".to_string(),
        97 => "ITEM_HONDEW_BERRY".to_string(),
        98 => "ITEM_GREPA_BERRY".to_string(),
        99 => "ITEM_TAMATO_BERRY".to_string(),
        100 => "ITEM_OCCA_BERRY".to_string(),
        101 => "ITEM_PASSHO_BERRY".to_string(),
        102 => "ITEM_WACAN_BERRY".to_string(),
        103 => "ITEM_RINDO_BERRY".to_string(),
        104 => "ITEM_YACHE_BERRY".to_string(),
        105 => "ITEM_CHOPLE_BERRY".to_string(),
        106 => "ITEM_KEBIA_BERRY".to_string(),
        107 => "ITEM_SHUCA_BERRY".to_string(),
        108 => "ITEM_COBA_BERRY".to_string(),
        109 => "ITEM_PAYAPA_BERRY".to_string(),
        110 => "ITEM_TANGA_BERRY".to_string(),
        111 => "ITEM_CHARTI_BERRY".to_string(),
        112 => "ITEM_KASIB_BERRY".to_string(),
        113 => "ITEM_HABAN_BERRY".to_string(),
        114 => "ITEM_COLBUR_BERRY".to_string(),
        115 => "ITEM_BABIRI_BERRY".to_string(),
        116 => "ITEM_ROSELI_BERRY".to_string(),
        117 => "ITEM_LIECHI_BERRY".to_string(),
        118 => "ITEM_GANLON_BERRY".to_string(),
        119 => "ITEM_SALAC_BERRY".to_string(),
        120 => "ITEM_PETAYA_BERRY".to_string(),
        121 => "ITEM_APICOT_BERRY".to_string(),
        122 => "ITEM_LANSAT_BERRY".to_string(),
        123 => "ITEM_STARF_BERRY".to_string(),
        124 => "ITEM_ENIGMA_BERRY".to_string(),
        125 => "ITEM_MICLE_BERRY".to_string(),
        126 => "ITEM_CUSTAP_BERRY".to_string(),
        127 => "ITEM_JABOCA_BERRY".to_string(),
        128 => "ITEM_ROWAP_BERRY".to_string(),
        129 => "ITEM_KEE_BERRY".to_string(),
        130 => "ITEM_MARANGA_BERRY".to_string(),
        _ => "".to_string()
    };
    old_item
}

fn create_rival_teams(settings: &mut settings::Settings,pokemon_data: &Vec<pokemon::PokemonStats>) -> (MayBrendanTeam,WallyTeam){
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
                level: 31,
                extra_scripts: "".to_string(),
                species: pokemon::Pokemon::Tropius,
                moves: Vec::new(),
                held_items: "".to_string()
            },
            TrainerPokemon{
                iv: 150,
                level: 32,
                extra_scripts: "".to_string(),
                species: pokemon::Pokemon::Ludicolo,
                moves: Vec::new(),
                held_items: "".to_string()
            },
            TrainerPokemon{
                iv: 150,
                level: 32,
                extra_scripts: "".to_string(),
                species: pokemon::Pokemon::Slugma,
                moves: Vec::new(),
                held_items: "".to_string()
            }
        ]
    };
    fake_rival = get_random_trainer(fake_rival,settings,pokemon_data);
    let may_team = MayBrendanTeam{
        pokemon2: fake_rival.pokemon[0].species,
        pokemon3: fake_rival.pokemon[1].species,
        pokemon4: fake_rival.pokemon[2].species,
    };
    let mut fake_wally = Trainer{
        trainer_full_name: "TRAINER_WALLY".to_string(),
        trainer_name: "wally".to_string(),
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
                level: 44,
                extra_scripts: "".to_string(),
                species: pokemon::Pokemon::Altaria,
                moves: Vec::new(),
                held_items: "".to_string()
            },
            TrainerPokemon{
                    iv: 150,
                    level: 43,
                    extra_scripts: "".to_string(),
                    species: pokemon::Pokemon::Delcatty,
                    moves: Vec::new(),
                    held_items: "".to_string()
                },
            TrainerPokemon{
                iv: 150,
                level: 44,
                extra_scripts: "".to_string(),
                species: pokemon::Pokemon::Roselia,
                moves: Vec::new(),
                held_items: "".to_string()
            },
            TrainerPokemon{
                iv: 150,
                level: 41,
                extra_scripts: "".to_string(),
                species: pokemon::Pokemon::Magneton,
                moves: Vec::new(),
                held_items: "".to_string()
            },
            TrainerPokemon{
                iv: 150,
                level: 45,
                extra_scripts: "".to_string(),
                species: pokemon::Pokemon::Gardevoir,
                moves: Vec::new(),
                held_items: "".to_string()
            }
        ]
    };
    fake_wally = get_random_trainer(fake_wally, settings, pokemon_data);
    let wally_team = WallyTeam{
        ralt_substitute: fake_wally.pokemon[4].species,
        pokemon2: fake_wally.pokemon[0].species,
        pokemon3: fake_wally.pokemon[1].species,
        pokemon4: fake_wally.pokemon[2].species,
        pokemon5: fake_wally.pokemon[3].species
    };
    (may_team,wally_team)
}

pub fn get_mon_specific_item(pokemon_species: pokemon::Pokemon,settings: &mut settings::Settings) -> String{
    if pokemon_species == pokemon::Pokemon::Silvally{
        return match settings::get_next_seed(0,17,settings){
            0 => "ITEM_FIRE_MEMORY",
            1 => "ITEM_WATER_MEMORY",
            2 => "ITEM_ELECTRIC_MEMORY",
            3 => "ITEM_GRASS_MEMORY",
            4 => "ITEM_ICE_MEMORY",
            5 => "ITEM_FIGHTING_MEMORY",
            6 => "ITEM_POISON_MEMORY",
            7 => "ITEM_GROUND_MEMORY",
            8 => "ITEM_FLYING_MEMORY",
            9 => "ITEM_PSYCHIC_MEMORY",
            10 => "ITEM_BUG_MEMORY",
            11 => "ITEM_ROCK_MEMORY",
            12 => "ITEM_GHOST_MEMORY",
            13 => "ITEM_DRAGON_MEMORY",
            14 => "ITEM_DARK_MEMORY",
            15 => "ITEM_STEEL_MEMORY",
            16 => "ITEM_FAIRY_MEMORY",
            _ => ""
        }.to_string();
    }
    if pokemon_species == pokemon::Pokemon::Arceus{
        if settings::get_next_seed(0,6,settings) == 0{//1 out of 6 get a z crystal instead of a plate
            return get_z_crystal(pokemon_species,special_trainers::get_random_type(settings));
        }
        return match settings::get_next_seed(0,17,settings){
            0 => "ITEM_FLAME_PLATE",
            1 => "ITEM_SPLASH_PLATE",
            2 => "ITEM_ZAP_PLATE",
            3 => "ITEM_MEADOW_PLATE",
            4 => "ITEM_ICICLE_PLATE",
            5 => "ITEM_FIST_PLATE",
            6 => "ITEM_TOXIC_PLATE",
            7 => "ITEM_EARTH_PLATE",
            8 => "ITEM_SKY_PLATE",
            9 => "ITEM_MIND_PLATE",
            10 => "ITEM_INSECT_PLATE",
            11 => "ITEM_STONE_PLATE",
            12 => "ITEM_SPOOKY_PLATE",
            13 => "ITEM_DRACO_PLATE",
            14 => "ITEM_DREAD_PLATE",
            15 => "ITEM_IRON_PLATE",
            16 => "ITEM_PIXIE_PLATE",
            _ => ""
        }.to_string();
    }
    if pokemon_species == pokemon::Pokemon::Genesect{
        return match settings::get_next_seed(0,4,settings){
            0 => "ITEM_DOUSE_DRIVE",
            1 => "ITEM_SHOCK_DRIVE",
            2 => "ITEM_BURN_DRIVE",
            3 => "ITEM_CHILL_DRIVE",
            _ => ""
        }.to_string();
    }
    if pokemon_species == pokemon::Pokemon::Clamperl{
        return match settings::get_next_seed(0,1,settings){
            0 => "ITEM_DEEP_SEA_SCALE",
            1 => "ITEM_DEEP_SEA_TOOTH",
            _ => ""
        }.to_string();
    }
    if pokemon_species == pokemon::Pokemon::Pikachu{
        return "ITEM_LIGHT_BALL".to_string();
    }
    if pokemon_species == pokemon::Pokemon::Chansey{
        return "ITEM_LUCKY_PUNCH".to_string();
    }
    if pokemon_species == pokemon::Pokemon::Ditto{
        return match settings::get_next_seed(0,1,settings){
            0 => "ITEM_METAL_POWDER",
            1 => "ITEM_QUICK_POWDER",
            _ => ""
        }.to_string();
    }
    if pokemon_species == pokemon::Pokemon::Latias || pokemon_species == pokemon::Pokemon::Latios{
        return "ITEM_SOUL_DEW".to_string();
    }
    if pokemon_species == pokemon::Pokemon::Farfetchd || pokemon_species == pokemon::Pokemon::FarfetchdGalarian || pokemon_species == pokemon::Pokemon::Sirfetchd{
        return "ITEM_LEEK".to_string();
    }
    if pokemon_species == pokemon::Pokemon::Cubone || pokemon_species == pokemon::Pokemon::Marowak || pokemon_species == pokemon::Pokemon::MarowakAlolan{
        return "ITEM_THICK_CLUB".to_string();
    }
    if pokemon_species == pokemon::Pokemon::Spritzee{
        return "ITEM_SACHET".to_string();
    }
    if pokemon_species == pokemon::Pokemon::Swirlix{
        return "ITEM_WHIPPED_DREAM".to_string();
    }
    if pokemon_species == pokemon::Pokemon::Happiny{
        return "ITEM_OVAL_STONE".to_string();
    }
    if pokemon_species == pokemon::Pokemon::Kyurem{
        return "ITEM_DNA_SPLICERS".to_string();
    }
    if pokemon_species == pokemon::Pokemon::ShayminSky{
        return "ITEM_GRACIDEA".to_string();
    }
    if pokemon_species == pokemon::Pokemon::Palkia{
        return "ITEM_LUSTROUS_ORB".to_string();
    }
    if pokemon_species == pokemon::Pokemon::Electabuzz{
        return "ITEM_GRISEOUS_ORB".to_string();
    }
    return "".to_string();
}

pub fn get_mega_stone(pokemon_species: pokemon::Pokemon) -> String{
    match pokemon_species{
        pokemon::Pokemon::VenusaurMega => "ITEM_VENUSAURITE",
        pokemon::Pokemon::CharizardMegaX => "ITEM_CHARIZARDITE_X",
        pokemon::Pokemon::CharizardMegaY => "ITEM_CHARIZARDITE_Y",
        pokemon::Pokemon::BlastoiseMega => "ITEM_BLASTOISINITE",
        pokemon::Pokemon::BeedrillMega => "ITEM_BEEDRILLITE",
        pokemon::Pokemon::PidgeotMega => "ITEM_PIDGEOTITE",
        pokemon::Pokemon::AlakazamMega => "ITEM_ALAKAZITE",
        pokemon::Pokemon::SlowbroMega => "ITEM_SLOWBRONITE",
        pokemon::Pokemon::GengarMega => "ITEM_GENGARITE",
        pokemon::Pokemon::KangaskhanMega => "ITEM_KANGASKHANITE",
        pokemon::Pokemon::PinsirMega => "ITEM_PINSIRITE",
        pokemon::Pokemon::GyaradosMega => "ITEM_GYARADOSITE",
        pokemon::Pokemon::AerodactylMega => "ITEM_AERODACTYLITE",
        pokemon::Pokemon::MewTwoMegaX => "ITEM_MEWTWONITE_X",
        pokemon::Pokemon::MewTwoMegaY => "ITEM_MEWTWONITE_Y",
        pokemon::Pokemon::AmpharosMega => "ITEM_AMPHAROSITE",
        pokemon::Pokemon::SteelixMega => "ITEM_STEELIXITE",
        pokemon::Pokemon::ScizorMega => "ITEM_SCIZORITE",
        pokemon::Pokemon::HeracrossMega => "ITEM_HERACRONITE",
        pokemon::Pokemon::HoundoomMega => "ITEM_HOUNDOOMINITE",
        pokemon::Pokemon::TyranitarMega => "ITEM_TYRANITARITE",
        pokemon::Pokemon::SceptileMega => "ITEM_SCEPTILITE",
        pokemon::Pokemon::BlazikenMega => "ITEM_BLAZIKENITE",
        pokemon::Pokemon::SwampertMega => "ITEM_SWAMPERTITE",
        pokemon::Pokemon::GardevoirMega => "ITEM_GARDEVOIRITE",
        pokemon::Pokemon::SableyeMega => "ITEM_SABLENITE",
        pokemon::Pokemon::MawileMega => "ITEM_MAWILITE",
        pokemon::Pokemon::AggronMega => "ITEM_AGGRONITE",
        pokemon::Pokemon::MedichamMega => "ITEM_MEDICHAMITE",
        pokemon::Pokemon::ManectricMega => "ITEM_MANECTITE",
        pokemon::Pokemon::SharpedoMega => "ITEM_SHARPEDONITE",
        pokemon::Pokemon::CameruptMega => "ITEM_CAMERUPTITE",
        pokemon::Pokemon::AltariaMega => "ITEM_ALTARIANITE",
        pokemon::Pokemon::BanetteMega => "ITEM_BANETTITE",
        pokemon::Pokemon::AbsolMega => "ITEM_ABSOLITE",
        pokemon::Pokemon::GlalieMega => "ITEM_GLALITITE",
        pokemon::Pokemon::SalamenceMega => "ITEM_SALAMENCITE",
        pokemon::Pokemon::MetagrossMega => "ITEM_METAGROSSITE",
        pokemon::Pokemon::LatiasMega => "ITEM_LATIASITE",
        pokemon::Pokemon::LatiosMega => "ITEM_LATIOSITE",
        pokemon::Pokemon::LopunnyMega => "ITEM_LOPUNNITE",
        pokemon::Pokemon::GarchompMega => "ITEM_GARCHOMPITE",
        pokemon::Pokemon::LucarioMega => "ITEM_LUCARIONITE",
        pokemon::Pokemon::AbomasnowMega => "ITEM_ABOMASITE",
        pokemon::Pokemon::GalladeMega => "ITEM_GALLADITE",
        pokemon::Pokemon::AudinoMega => "ITEM_AUDINITE",
        pokemon::Pokemon::DiancieMega => "ITEM_DIANCITE",
        pokemon::Pokemon::GroudonPrimal => "ITEM_RED_ORB",
        pokemon::Pokemon::KyogrePrimal => "ITEM_BLUE_ORB",
        pokemon::Pokemon::ZamazentaCrownedShield => "ITEM_RUSTED_SHIELD",
        pokemon::Pokemon::ZacianCrownedSword => "ITEM_RUSTED_SWORD",
        _ => ""
    }.to_string()
}

pub fn get_z_crystal(pokemon_species: pokemon::Pokemon, prefered_type: pokemon::Type) -> String{
    let special_crystal = match pokemon_species{
        pokemon::Pokemon::RaichuAlolan => "ITEM_ALORAICHIUM_Z",
        pokemon::Pokemon::Decidueye => "ITEM_DECIDIUM_Z",
        pokemon::Pokemon::Eevee => "ITEM_EEVIUM_Z",
        pokemon::Pokemon::Incineroar => "ITEM_INCINIUM_Z",
        pokemon::Pokemon::Kommoo => "ITEM_KOMMONIUM_Z",
        pokemon::Pokemon::Lunala | pokemon::Pokemon::NecrozmaDuskMane => "ITEM_LUNALIUM_Z",
        pokemon::Pokemon::Lycanroc => "ITEM_LYCANIUM_Z",
        pokemon::Pokemon::Marshadow => "ITEM_MARSHADIUM_Z",
        pokemon::Pokemon::Mew => "ITEM_MEWNIUM_Z",
        pokemon::Pokemon::Mimikyu => "ITEM_MIMIKIUM_Z",
        pokemon::Pokemon::Pikachu => "ITEM_PIKANIUM_Z",
        pokemon::Pokemon::Primarina => "ITEM_PRIMARIUM_Z",
        pokemon::Pokemon::Snorlax => "ITEM_SNORLIUM_Z",
        pokemon::Pokemon::Solgaleo | pokemon::Pokemon::NecrozmaDawnWings => "ITEM_SOLGANIUM_Z",
        pokemon::Pokemon::TapuBulu | pokemon::Pokemon::TapuFini | pokemon::Pokemon::TapuKoko | pokemon::Pokemon::TapuLele => "ITEM_TAPUNIUM_Z",
        pokemon::Pokemon::NecrozmaUltra => "ITEM_ULTRANECROZIUM_Z",
        _ => ""
    }.to_string();
    if special_crystal != "" {return special_crystal;}
    return match prefered_type{
        pokemon::Type::Normal => "ITEM_NORMALIUM_Z".to_string(),
        pokemon::Type::Fire => "ITEM_FIRIUM_Z".to_string(),
        pokemon::Type::Water => "ITEM_WATERIUM_Z".to_string(),
        pokemon::Type::Electric => "ITEM_ELECTRIUM_Z".to_string(),
        pokemon::Type::Grass => "ITEM_GRASSIUM_Z".to_string(),
        pokemon::Type::Ice => "ITEM_ICIUM_Z".to_string(),
        pokemon::Type::Fighting => "ITEM_FIGHTINIUM_Z".to_string(),
        pokemon::Type::Poison => "ITEM_POISONIUM_Z".to_string(),
        pokemon::Type::Ground => "ITEM_GROUNDIUM_Z".to_string(),
        pokemon::Type::Flying => "ITEM_FLYINIUM_Z".to_string(),
        pokemon::Type::Psychic => "ITEM_PSYCHIUM_Z".to_string(),
        pokemon::Type::Bug => "ITEM_BUGINIUM_Z".to_string(),
        pokemon::Type::Rock => "ITEM_ROCKIUM_Z".to_string(),
        pokemon::Type::Ghost => "ITEM_GHOSTIUM_Z".to_string(),
        pokemon::Type::Dragon => "ITEM_DRAGONIUM_Z".to_string(),
        pokemon::Type::Dark => "ITEM_DARKINIUM_Z".to_string(),
        pokemon::Type::Steel => "ITEM_STEELIUM_Z".to_string(),
        pokemon::Type::Fairy => "ITEM_FAIRIUM_Z".to_string(),
        _ => "ITEM_NORMALIUM_Z".to_string(),
    }
}

pub fn get_type_boosting_item(pokemon_species : pokemon::Pokemon, all_stats: &Vec<PokemonStats>,settings: &mut settings::Settings) -> String{
    let mut pkmn_type = pokemon::get_pokemon_data(pokemon_species,all_stats).type1;
    if pokemon::get_pokemon_data(pokemon_species,all_stats).type2 != pokemon::Type::None && settings::get_next_seed(0,1,settings) == 1{
        println!("Inclusive");
        pkmn_type = pokemon::get_pokemon_data(pokemon_species,all_stats).type2;
    }
    return match pkmn_type{
        pokemon::Type::Normal => "ITEM_SILK_SCARF".to_string(),
        pokemon::Type::Fire => "ITEM_CHARCOAL".to_string(),
        pokemon::Type::Water => "ITEM_MYSTIC_WATER".to_string(),
        pokemon::Type::Electric => "ITEM_MAGNET".to_string(),
        pokemon::Type::Grass => "ITEM_MIRACLE_SEED".to_string(),
        pokemon::Type::Ice => "ITEM_NEVER_MELT_ICE".to_string(),
        pokemon::Type::Fighting => "ITEM_BLACK_BELT".to_string(),
        pokemon::Type::Poison => "ITEM_POISON_BARB".to_string(),
        pokemon::Type::Ground => "ITEM_SOFT_SAND".to_string(),
        pokemon::Type::Flying => "ITEM_SHARP_BEAK".to_string(),
        pokemon::Type::Psychic => "ITEM_TWISTED_SPOON".to_string(),
        pokemon::Type::Bug => "ITEM_SILVER_POWDER".to_string(),
        pokemon::Type::Rock => "ITEM_HARD_STONE".to_string(),
        pokemon::Type::Ghost => "ITEM_SPELL_TAG".to_string(),
        pokemon::Type::Dragon => "ITEM_DRAGON_FANG".to_string(),
        pokemon::Type::Dark => "ITEM_BLACK_GLASSES".to_string(),
        pokemon::Type::Steel => "ITEM_METAL_COAT".to_string(),
        _ => "".to_string(),
    }
}

pub fn get_gem(pokemon_species : pokemon::Pokemon, all_stats: &Vec<PokemonStats>,settings: &mut settings::Settings) -> String{
    let mut pkmn_type = pokemon::get_pokemon_data(pokemon_species,all_stats).type1;
    if pokemon::get_pokemon_data(pokemon_species,all_stats).type2 != pokemon::Type::None && settings::get_next_seed(0,1,settings) == 1{
        println!("Inclusive");
        pkmn_type = pokemon::get_pokemon_data(pokemon_species,all_stats).type2;
    }
    return match pkmn_type{
        pokemon::Type::Normal => "ITEM_NORMAL_GEM".to_string(),
        pokemon::Type::Fire => "ITEM_FIRE_GEM".to_string(),
        pokemon::Type::Water => "ITEM_WATER_GEM".to_string(),
        pokemon::Type::Electric => "ITEM_ELECTRIC_GEM".to_string(),
        pokemon::Type::Grass => "ITEM_GRASS_GEM".to_string(),
        pokemon::Type::Ice => "ITEM_ICE_GEM".to_string(),
        pokemon::Type::Fighting => "ITEM_FIGHTING_GEM".to_string(),
        pokemon::Type::Poison => "ITEM_POISON_GEM".to_string(),
        pokemon::Type::Ground => "ITEM_GROUND_GEM".to_string(),
        pokemon::Type::Flying => "ITEM_FLYING_GEM".to_string(),
        pokemon::Type::Psychic => "ITEM_PSYCHIC_GEM".to_string(),
        pokemon::Type::Bug => "ITEM_BUG_GEM".to_string(),
        pokemon::Type::Rock => "ITEM_ROCK_GEM".to_string(),
        pokemon::Type::Ghost => "ITEM_GHOST_GEM".to_string(),
        pokemon::Type::Dragon => "ITEM_DRAGON_GEM".to_string(),
        pokemon::Type::Dark => "ITEM_DARK_GEM".to_string(),
        pokemon::Type::Steel => "ITEM_STEEL_GEM".to_string(),
        pokemon::Type::Fairy => "ITEM_FAIRY_GEM".to_string(),
        _ => "".to_string(),
    }
}
//Last minute pokemon changes, that can only be done right before outputting to file
//For example, pokemon like minior who have multiple forms, or mega evolutions that need to be turned into regular pokemon
fn last_minute_pokemon_name_changes(species: pokemon::Pokemon,all_stats: &Vec<pokemon::PokemonStats>,settings: &mut settings::Settings) -> String{
    if pokemon::get_pokemon_data(species,all_stats).status == pokemon::LegendStatus::Mega || pokemon::get_pokemon_data(species,all_stats).status == pokemon::LegendStatus::LegendMega {
        return pokemon::get_pokemon_data(pokemon::get_pokemon_data(species,all_stats).evolve_from,all_stats).pokemon_name;
    }
    return pokemon::pokemon_alternate_forms(species,all_stats,settings);
}

pub struct MayBrendanTeam{
    pub pokemon2: pokemon::Pokemon,
    pub pokemon3: pokemon::Pokemon,
    pub pokemon4: pokemon::Pokemon,
}
pub struct WallyTeam{
    pub ralt_substitute: pokemon::Pokemon,
    pub pokemon2: pokemon::Pokemon,
    pub pokemon3: pokemon::Pokemon,
    pub pokemon4: pokemon::Pokemon,
    pub pokemon5: pokemon::Pokemon,
}