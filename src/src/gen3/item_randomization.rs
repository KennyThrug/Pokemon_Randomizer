use std::fs;
use crate::src::settings;
use crate::src::gen3::wild_pokemon;
use crate::src::pokemon;
use crate::src::gen3::game_chooser;
use crate::src::gen3::logic;
use crate::src::hint_system;

#[derive(Clone)]
pub struct Item{
    pub item_name: String,
    pub trainer_name: String,
    pub item_script: String,
    pub post_item_script: String,
    pub location_type: Location_type,
    pub location_area: String,
    pub item_type: Item_type,
    pub item_hidden: bool,
    pub prerequisites: Vec<String>
}
#[derive(PartialEq,Clone)]
pub enum Location_type{
    ITEM_BALL,
    HIDDEN_ITEM,
    TRAINER,
    NPC,
    LEGENDARY_POKEMON,
    GYM_LEADER,
}
#[derive(PartialEq,Clone)]
pub enum Item_type{
    NORMAL_ITEM,
    TRAINER,
    EGG,
    POKEMON,
    TRAP,
    BADGE
}
fn parse_location_type(raw_string: String) -> Location_type{
    match raw_string.as_str(){
        "Item Ball" => Location_type::ITEM_BALL,
        "Hidden Item" => Location_type::HIDDEN_ITEM,
        "Trainer" => Location_type::TRAINER,
        "NPC" => Location_type::NPC,
        "Legendary Pokemon" => Location_type::LEGENDARY_POKEMON,
        "Gym Leader" => Location_type::GYM_LEADER,
        &_ => Location_type::TRAINER
    }
}
fn parse_item_type(raw_string: String) -> Item_type{
    match raw_string.as_str(){
        "Normal Item" => Item_type::NORMAL_ITEM,
        "Trainer" => Item_type::TRAINER,
        "Egg" => Item_type::EGG,
        "Pokemon" => Item_type::POKEMON,
        "Trap" => Item_type::TRAP,
        "Badge" => Item_type::BADGE,
        &_ => Item_type::NORMAL_ITEM
    }
}

fn get_all_items(settings: &mut settings::Settings) -> Vec<Item>{
    let csv: String = fs::read_to_string(game_chooser::get_item_locations(settings)).unwrap();
    let mut reader = csv::Reader::from_reader(csv.as_bytes());
    let mut all_items : Vec<Item> = Vec::new();
    for cur_item in reader.records(){
        let cur_item = cur_item.unwrap();

        let mut prereqs: Vec<String> = Vec::new();
        for i in 7..14{
            if(cur_item[i].to_string() == ""){
                break;
            }
            prereqs.push(cur_item[i].to_string());
        }
        all_items.push(
            Item{
                item_name: cur_item[0].to_string(),
                item_script: cur_item[4].to_string(),
                post_item_script: cur_item[5].to_string(),
                trainer_name: cur_item[0].to_uppercase().to_string(),
                location_type: parse_location_type(cur_item[3].to_string()),
                location_area: cur_item[1].to_string(),
                item_type: parse_item_type(cur_item[2].to_string()),
                item_hidden: if cur_item[6].to_string() == "TRUE" {true} else {false},
                prerequisites: prereqs
            }
        );
    }
    return all_items;
}

//Entry point, called by an outside file (i.e. emerald/startup.rs)
//Doesn't actually do the randomization of the files, but calls functions that do (randomize)
pub fn randomize_items(settings: &mut settings::Settings,pokemon_data: &Vec<pokemon::PokemonStats>) -> Vec<Item>{
    let mut all_items = get_all_items(settings);
    //Note: this "randomize" function passes ownership of all_items
    all_items = randomize(all_items,settings,pokemon_data);
    return all_items;
}

//Primary function used to Randomize items. If you are looking to change randomization, probably look at this or look at logic
fn randomize(mut all_item_locations: Vec<Item>,settings: &mut settings::Settings,pokemon_data: &Vec<pokemon::PokemonStats>) -> Vec<Item>{
    if(settings.randomize_items == false){return all_item_locations;}//No point in this function if randomization is off
    //Step one, get all the items we need to add to the pool
    let mut all_items_to_add = add_items_to_pool(settings);
    //Step two, randomize the items
    let mut all_items = randomize_vector_item(settings,&mut all_item_locations.clone());
    let banned_items = get_banned_items(settings);
    let mut final_items: Vec<Item> = Vec::new();
    let mut added = false;
    println!("before : {}",all_item_locations.len());
    for mut cur_loc in all_item_locations{
        println!("Pre Item name: {} Item Script: {}",cur_loc.item_name,cur_loc.item_script);
        if cur_loc.item_script == "Oldale_Eventscript_Item".to_string(){
            println!("Right here!~!!!");
            println!("Oldale Item name: {} Item Script: {}",cur_loc.item_name,cur_loc.item_script);
        }
        added = false;
        if is_banned(cur_loc.clone().item_name,banned_items.clone()) 
        || (!settings.items_from_trainers && cur_loc.location_type == Location_type::TRAINER)
        || (!settings.randomize_hidden_items && cur_loc.item_hidden == true){
            final_items.push(cur_loc.clone());
            added = true;
        }
        let mut item_failed: Vec<String> = Vec::new();
        while !added{
            let item_add = all_items_to_add.pop().expect("Failed to get next item");
            if logic::check_logic(settings,item_add.clone(),cur_loc.clone().location_area,cur_loc.clone().prerequisites){
                hint_system::add_line_to_spoiler(format!("{} in {} randomized into {}",cur_loc.item_name,cur_loc.location_area,item_add),settings);
                cur_loc.item_type = get_item_type(item_add.clone());
                cur_loc.item_name = get_item_final_changes(cur_loc.clone(),item_add,settings,pokemon_data);
                final_items.push(cur_loc.clone());
                added = true;
            }
            else{
                println!("Failure");
                item_failed.push(item_add.clone());
            }
        }
        println!("Post Item name: {} Item Script: {}",cur_loc.item_name,cur_loc.item_script);
        all_items_to_add.append(&mut item_failed);
    }
    
    return final_items;
}

fn get_item_final_changes(item: Item,new_item: String, settings: &mut settings::Settings, pokemon_data: &Vec<pokemon::PokemonStats>) -> String{
    if item.item_type == Item_type::EGG{
        return wild_pokemon::get_random_wild_pokemon(settings,pokemon_data,5);//Some pokemon
    }
    if item.item_type == Item_type::POKEMON{
        //Pick a random level from 5 to 40 (maybe change this later to an option)
        let level_of_pokemon = settings::get_next_seed(5,40,settings) as i16;
        return format!("{}",
            wild_pokemon::get_random_wild_pokemon(settings,pokemon_data,level_of_pokemon));//Some pokemon
    }
    return new_item;
}

fn get_item_type(item: String) -> Item_type{
    //Check to see what type of item this is
    if item == "EGG"{
        return Item_type::EGG;
    }
    else if item == "POKEMON"{
        return Item_type::POKEMON;
    }
    //check if it is a Badge (Has format FLAG_UNUSED_0x8E5-C)
    else if item.len() > 11 && item[0..10].to_string() == "FLAG_BADGE".to_string(){
        return Item_type::BADGE;
    }
    else if item == "TRAP"{
        return Item_type::TRAP;
    }
    else{
        return Item_type::NORMAL_ITEM;
    }
}

fn is_banned(item: String,banned_item : Vec<String>) -> bool{
    for i in banned_item{
        if i == item{
            return true;
        }
    }
    return false;
}

fn get_banned_items(settings: &mut settings::Settings) -> Vec<String>{
    let mut banned_list: Vec<String> = Vec::new();
    if settings.randomize_hms == false {
        banned_list.append(&mut vec![
            "ITEM_HM01".to_string(),
            "ITEM_HM02".to_string(),
            "ITEM_HM03".to_string(),
            "ITEM_HM04".to_string(),
            "ITEM_HM05".to_string(),
            "ITEM_HM06".to_string(),
            "ITEM_HM07".to_string(),
            "ITEM_HM08".to_string()
            ]);
    }
    if settings.randomize_gym_badges == false{
        banned_list.append(&mut vec![
            "FLAG_BADGE01_GET".to_string(),
            "FLAG_BADGE02_GET".to_string(),
            "FLAG_BADGE03_GET".to_string(),
            "FLAG_BADGE04_GET".to_string(),
            "FLAG_BADGE05_GET".to_string(),
            "FLAG_BADGE06_GET".to_string(),
            "FLAG_BADGE07_GET".to_string(),
            "FLAG_BADGE08_GET".to_string()
        ]);
    }
    if settings.randomize_key_items == false{
        banned_list.append(&mut vec![
            "ITEM_BICYCLE".to_string(),
            "ITEM_MACH_BIKE".to_string(),
            "ITEM_ACRO_BIKE".to_string(),
            "ITEM_OLD_ROD".to_string(),
            "ITEM_GOOD_ROD".to_string(),
            "ITEM_SUPER_ROD".to_string(),
            "ITEM_DOWSING_MACHINE".to_string(),
            "ITEM_TOWN_MAP".to_string(),
            "ITEM_VS_SEEKER".to_string(),
            "ITEM_TM_CASE".to_string(),
            "ITEM_BERRY_POUCH".to_string(),
            "ITEM_POKEMON_BOX_LINK".to_string(),
            "ITEM_COIN_CASE".to_string(),
            "ITEM_POWDER_JAR".to_string(),
            "ITEM_WAILMER_PAIL".to_string(),
            "ITEM_POKE_RADAR".to_string(),
            "ITEM_POKEBLOCK_CASE".to_string(),
            "ITEM_SOOT_SACK".to_string(),
            "ITEM_POKE_FLUTE".to_string(),
            "ITEM_FAME_CHECKER".to_string(),
            "ITEM_TEACHY_TV".to_string(),
            "ITEM_SS_TICKET".to_string(),
            "ITEM_EON_TICKET".to_string(),
            "ITEM_MYSTIC_TICKET".to_string(),
            "ITEM_AURORA_TICKET".to_string(),
            "ITEM_OLD_SEA_MAP".to_string(),
            "ITEM_LETTER".to_string(),
            "ITEM_DEVON_PARTS".to_string(),
            "ITEM_GO_GOGGLES".to_string(),
            "ITEM_DEVON_SCOPE".to_string(),
            "ITEM_BASEMENT_KEY".to_string(),
            "ITEM_SCANNER".to_string(),
            "ITEM_STORAGE_KEY".to_string(),
            "ITEM_KEY_TO_ROOM_1".to_string(),
            "ITEM_KEY_TO_ROOM_2".to_string(),
            "ITEM_KEY_TO_ROOM_4".to_string(),
            "ITEM_KEY_TO_ROOM_6".to_string(),
            "ITEM_METEORITE".to_string(),
            "ITEM_MAGMA_EMBLEM".to_string(),
            "ITEM_CONTEST_PASS".to_string(),
            "ITEM_OAKS_PARCEL".to_string(),
            "ITEM_SECRET_KEY".to_string(),
            "ITEM_BIKE_VOUCHER".to_string(),
            "ITEM_GOLD_TEETH".to_string(),
            "ITEM_CARD_KEY".to_string(),
            "ITEM_LIFT_KEY".to_string(),
            "ITEM_SILPH_SCOPE".to_string(),
            "ITEM_TRI_PASS".to_string(),
            "ITEM_RAINBOW_PASS".to_string(),
            "ITEM_TEA".to_string(),
            "ITEM_RUBY".to_string(),
            "ITEM_SAPPHIRE".to_string(),
            "ITEM_RED_ORB".to_string(),
            "ITEM_BLUE_ORB".to_string()
        ]);
    }
    return banned_list;
}

//This function takes a vector full of strings, and randomizes the order
fn randomize_vector(settings: &mut settings::Settings,items: &mut Vec<String>) -> Vec<String>{
    let mut randomized_items: Vec<String> = Vec::new();
    while(items.len() != 0){
        randomized_items.push(items.swap_remove(settings::get_next_seed(0,items.len() as i32,settings) as usize));
        println!("{}",randomized_items[randomized_items.len()-1]);
    }
    return randomized_items;
}
//Same as previous function, just with Items instead of strings
fn randomize_vector_item(settings: &mut settings::Settings,items: &mut Vec<Item>) -> Vec<Item>{
    let mut randomized_items: Vec<Item> = Vec::new();
    while(items.len() != 0){
        println!("Item in first slot: {}",items[0].item_name);
        randomized_items.push(items.swap_remove(settings::get_next_seed(0,items.len() as i32,settings) as usize));
        println!("Item just added: {}",randomized_items[randomized_items.len()-1].item_name);
    }
    return randomized_items;
}



fn add_items_to_pool(settings: &mut settings::Settings) -> Vec<String>{
    let data = fs::read_to_string(game_chooser::get_items(settings).as_str()).expect("unable to read file");
    let mut parsed_data = json::parse(&data).unwrap();
    let mut total_items: Vec<String> = Vec::new();
    //Key Items
    for i in 0..settings.add_rare_candy{
        total_items.push("ITEM_RARE_CANDY".to_string());
    }
    for i in 0..settings.add_pokeballs{
        total_items.push(parsed_data["PokeBalls"][settings::get_next_seed(0,20,settings) as usize].to_string());
    }
    if(settings.randomize_key_items){
        add_items_of_type(&mut parsed_data["Story Key Items"],&mut total_items);
        add_items_of_type(&mut parsed_data["Misc. Key Items"],&mut total_items);
        add_items_of_type(&mut parsed_data["Battle Mechanic Key Items"],&mut total_items);
        add_items_of_type(&mut parsed_data["Form-changing Key Items"],&mut total_items);
        add_items_of_type(&mut parsed_data["Colored Orbs"],&mut total_items);
    }
    if(settings.randomize_hms){
        //because there always has to be at least one HM
        for i in 0..(settings.number_hms+1){
            add_items_of_type(&mut parsed_data["HMs"],&mut total_items);
        }
    }
    if(settings.randomize_gym_badges){
        add_items_of_type(&mut parsed_data["Badges"],&mut total_items);
    }
    //Add pokemon manually cuz why not
    total_items.push("POKEMON".to_string());//Castform
    total_items.push("POKEMON".to_string());//Beldum
    total_items.push("EGG".to_string());//Wynaut
    
    let mut shuffled_items: Vec<String> = Vec::new();
    let mut item_types_to_add = vec![
    "Regional Specialties","Vitamins","EV Feathers","Ability Modifiers","Mints",
    "Candy","Medicinal Flutes","Encounter-modifying Flutes","Encounter Modifiers",
    "X Items","X Items","Escape Items","Escape Items","Treasures","Fossils",
    "Mulch","Misc","Apricorns","Mail","Evolution Items","Nectars","Plates",
    "Memories","Mega Stones","Gems","Z-Crystals","Species-specific Held Items",
    "Incenses","Contest Scarves","EV Gain Modifiers","Type-boosting Held Items",
    "Choice Items","Status Orbs","Weather Rocks","Terrain Seeds","Type Activated Stat Modifiers",
    "Misc. Held Items","Berries","TMs","Charms","Gen IX Items"];
    if(settings.add_revives){
        for i in 0..6{
            item_types_to_add.push("Revives");
        }
    }
    if(settings.allow_healing_items){
        for i in 0..6{
            item_types_to_add.push("Medicine");
            item_types_to_add.push("Potions");
        }
    }
    //This has many potions so that there are plenty of potions (probably the most important item to have multis of)
    for i in item_types_to_add.iter(){
        add_items_of_type(&mut parsed_data[*i],&mut shuffled_items);
    }
    let mut new_suffled = randomize_vector(settings,&mut shuffled_items);

    //Make sure that the key items are at the END and not the begining
    new_suffled.append(&mut total_items);
    return new_suffled;
}

//Helper function for add_items_to_pool, adds items to the array so I don't have a ton of duplicate code
fn add_items_of_type(data: &mut json::JsonValue,total_items: &mut Vec<String>){
    let mut data_temp = data.clone();
    for i in 0..data_temp.len(){
        total_items.push(data_temp.pop().to_string());
    }
}