use crate::src::gen3::item_randomization::{Item,Item_type,Location_type};
use crate::src::settings;
use crate::src::gen3::game_chooser;
use crate::src::pokemon;
use std::fs;
use glob::glob;

//Top level function to be called initially
pub fn create_rando_scripts(settings: &mut settings::Settings,mut all_items: Vec<Item>,pokemon_data: &Vec<pokemon::PokemonStats>){
    let mut final_string = game_chooser::startup_stuff(settings);
    let mut all_item_balls : Vec<Item> = Vec::new();
    let mut all_trainers : Vec<Item> = Vec::new();
    for cur_item in all_items{
        match cur_item.location_type{
            Location_type::NPC | Location_type::LEGENDARY_POKEMON | Location_type::GYM_LEADER => {
                final_string.push_str(convert_item_to_function(cur_item.clone(),settings,pokemon_data).as_str());
            }
            Location_type::ITEM_BALL | Location_type::HIDDEN_ITEM => {
                all_item_balls.push(cur_item.clone());
            }
            Location_type::TRAINER => {
                all_trainers.push(cur_item.clone());
            }
        }
    }
    create_map_jsons(settings,all_item_balls);
    fs::write("decomp/pokeemerald-expansion/data/scripts/randomizer_scripts.inc",final_string);
}

//Goes through every map.json and change stuff
fn create_map_jsons(settings: &mut settings::Settings, mut all_items: Vec<Item>) -> Vec<Item>{
    println!("Folders coming up");
    for folder in glob("decomp/pokeemerald-expansion/data/maps/**/*.json").expect("Failed to read glob pattern") {
        match folder {
            Ok(path) => {
                change_item_in_map_json(path.display().to_string(),&mut all_items);
            },
            Err(e) => println!("{:?}", e),
        }
    }
    return all_items;
}

fn extra_rival_stuff() -> String{
    return "case TRAINER_BRENDAN_ROUTE_103_MUDKIP, Route103_EventScript_Item_Rival
    case TRAINER_BRENDAN_ROUTE_110_MUDKIP, Route110_EventScript_GiveItemfinder_Item2
    case TRAINER_BRENDAN_ROUTE_119_MUDKIP, Route119_EventScript_RivalExitScottArrive_Item
    case TRAINER_BRENDAN_ROUTE_103_TREECKO, Route103_EventScript_Item_Rival
    case TRAINER_BRENDAN_ROUTE_110_TREECKO, Route110_EventScript_GiveItemfinder_Item2
    case TRAINER_BRENDAN_ROUTE_119_TREECKO, Route119_EventScript_RivalExitScottArrive_Item
    case TRAINER_BRENDAN_ROUTE_103_TORCHIC, Route103_EventScript_Item_Rival
    case TRAINER_BRENDAN_ROUTE_110_TORCHIC, Route110_EventScript_GiveItemfinder_Item2
    case TRAINER_BRENDAN_ROUTE_119_TORCHIC, Route119_EventScript_RivalExitScottArrive_Item
    case TRAINER_BRENDAN_ROUTE_103_MUDKIP, Route103_EventScript_Item_Rival
    case TRAINER_MAY_ROUTE_103_TREECKO, Route103_EventScript_Item_Rival
    case TRAINER_MAY_ROUTE_110_TREECKO, Route110_EventScript_GiveItemfinder_Item2
    case TRAINER_MAY_ROUTE_119_TREECKO, Route119_EventScript_RivalExitScottArrive_Item
    case TRAINER_MAY_ROUTE_103_TORCHIC, Route103_EventScript_Item_Rival
    case TRAINER_MAY_ROUTE_110_TORCHIC, Route110_EventScript_GiveItemfinder_Item2
    case TRAINER_MAY_ROUTE_119_TORCHIC, Route119_EventScript_RivalExitScottArrive_Item
    case TRAINER_BRENDAN_LILYCOVE_MUDKIP, LilycoveCity_EventScript_Rival_Item
    case TRAINER_BRENDAN_LILYCOVE_TREECKO, LilycoveCity_EventScript_Rival_Item
    case TRAINER_BRENDAN_LILYCOVE_TORCHIC, LilycoveCity_EventScript_Rival_Item
    case TRAINER_MAY_LILYCOVE_TREECKO, LilycoveCity_EventScript_Rival_Item
    case TRAINER_MAY_LILYCOVE_TORCHIC, LilycoveCity_EventScript_Rival_Item
    return".to_string()
}


fn change_item_in_map_json(filename: String,all_items: &mut Vec<Item>){
    let data = fs::read_to_string(filename.clone()).expect("unable to read file");
    let mut parsed_data = json::parse(&data).unwrap();
    let mut end_data = json::JsonValue::new_array();
    while !parsed_data["object_events"].is_empty(){
        let mut cur_obj = parsed_data["object_events"].pop();
        println!("Objects: {}",cur_obj["script"].to_string());

        //Get Randomized items and set them into their spots
        for cur_item in &mut all_items.iter(){
            if cur_obj["flag"].to_string() == cur_item.item_script{
                cur_obj["script"] = json::JsonValue::String(get_item_script(cur_item.item_type.clone()));
                cur_obj["trainer_sight_or_berry_tree_id"] = json::JsonValue::String(cur_item.item_name.clone());
                break;
            }
        }

        //Add back into array
        end_data.push(cur_obj);
    }
    parsed_data["object_events"] = end_data;
    fs::write(filename.clone(),parsed_data.to_string());
}

fn get_item_script(item_type :Item_type) -> String{
    return match item_type{
        Item_type::NORMAL_ITEM => "Common_EventScript_FindItem".to_string(),
        Item_type::TRAINER => "common_rando_".to_string(),
        Item_type::EGG => "common_rando_".to_string(),
        Item_type::POKEMON => "Randomizer_Eventscript_Give_Pokemon".to_string(),
        Item_type::TRAP => "common_rando_".to_string(),
        Item_type::BADGE => "common_rando_".to_string()
    };
}

//Formats the item to a correctly functioning file to then be compiled, adds trainer functions to the end to be added later
fn convert_item_to_function(cur_item: Item,settings: &mut settings::Settings,pokemon_data: &Vec<pokemon::PokemonStats>) -> String{
    let mut final_string = format!("{}::\n",cur_item.item_script);

    if cur_item.item_type == Item_type::NORMAL_ITEM {
        final_string.push_str(format!("giveitem {}",cur_item.clone().item_name).as_str());
    }
    else if cur_item.item_type == Item_type::EGG {
        final_string.push_str(format!("setvar VAR_TEMP_TRANSFERRED_SPECIES, {}\ncall Randomizer_Eventscript_Give_Egg",
        cur_item.item_name).as_str());
    }
    else if cur_item.item_type == Item_type::POKEMON {
        final_string.push_str(format!("setvar VAR_TEMP_TRANSFERRED_SPECIES, {}\ncall Randomizer_Eventscript_Give_Pokemon",
        cur_item.item_name).as_str());
    }
    else if cur_item.item_type == Item_type::TRAP{
        final_string.push_str(format!("setwildbattle {}, 40\ndowildbattle",
        cur_item.item_name).as_str());
    }
    else if cur_item.item_type == Item_type::BADGE {
        final_string.push_str(format!(
            "message {}
	waitmessage
	call Common_EventScript_PlayGymBadgeFanfare
	setflag {}
	setvar VAR_RESULT, TRUE",convert_badge_to_message(cur_item.item_name.clone()),cur_item.item_name).as_str());
    }
    //Add extra stuff that actually does the items

    final_string.push_str("\nreturn\n\n");
    final_string
}

fn convert_badge_to_message(badge_name: String) -> String{
    return match badge_name.as_str(){
        "FLAG_BADGE01_GET" => "RustboroCity_Gym_Text_ReceivedStoneBadge",
        "FLAG_BADGE02_GET" => "DewfordTown_Gym_Text_ReceivedKnuckleBadge",
        "FLAG_BADGE03_GET" => "MauvilleCity_Gym_Text_ReceivedDynamoBadge",
        "FLAG_BADGE04_GET" => "LavaridgeTown_Gym_1F_Text_ReceivedHeatBadge",
        "FLAG_BADGE05_GET" => "PetalburgCity_Gym_Text_ReceivedBalanceBadge",
        "FLAG_BADGE06_GET" => "FortreeCity_Gym_Text_ReceivedFeatherBadge",
        "FLAG_BADGE07_GET" => "MossdeepCity_Gym_Text_ReceivedMindBadge",
        "FLAG_BADGE08_GET" => "SootopolisCity_Gym_1F_Text_ReceivedRainBadge",
        &_ => ""
    }.to_string();
}