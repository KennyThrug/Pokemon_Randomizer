use std::fs;
use crate::src::settings;

//Contains all the locations an item could be
struct Item{
    item_name: String,
    item_script: String,
    location_type: Location_type,
    item_type: Item_type,
    item_hidden: bool,
    prerequisites: Vec<String>
}

#[derive(PartialEq)]
enum Location_type{
    ITEM_BALL,
    HIDDEN_ITEM,
    TRAINER,
    NPC,
    LEGENDARY_POKEMON,
    GYM_LEADER
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
#[derive(PartialEq)]
enum Item_type{
    NORMAL_ITEM,
    TRAINER,
    EGG,
    POKEMON,
    TRAP
}
fn parse_item_type(raw_string: String) -> Item_type{
    match raw_string.as_str(){
        "Normal Item" => Item_type::NORMAL_ITEM,
        "Trainer" => Item_type::TRAINER,
        "Egg" => Item_type::EGG,
        "Pokemon" => Item_type::POKEMON,
        "Trap" => Item_type::TRAP,
        &_ => Item_type::NORMAL_ITEM
    }
}

fn get_all_items(filename: String) -> Vec<Item>{
    let csv: String = fs::read_to_string(filename).unwrap();
    let mut reader = csv::Reader::from_reader(csv.as_bytes());
    let mut all_items : Vec<Item> = Vec::new();
    for cur_item in reader.records(){
        let cur_item = cur_item.unwrap();

        let mut prereqs: Vec<String> = Vec::new();
        for i in 6..12{
            if(cur_item[i].to_string() == ""){
                break;
            }
            prereqs.push(cur_item[i].to_string());
        }
        all_items.push(
            Item{
                item_name: cur_item[0].to_string(),
                item_script: cur_item[4].to_string(),
                location_type: parse_location_type(cur_item[3].to_string()),
                item_type: parse_item_type(cur_item[2].to_string()),
                item_hidden: if cur_item[5].to_string() == "true" {true} else {false},
                prerequisites: prereqs
            }
        );
    }
    return all_items;
}

//Entry point, called by an outside file (i.e. emerald/startup.rs)
//Doesn't actually do the randomization of the files, but calls functions that do (randomize)
pub fn randomize_items(settings: &mut settings::Settings){
    let mut all_items = get_all_items("data/emerald/item_locations.csv".to_string());
    //Note: this "randomize" function passes ownership of all_items
    all_items = randomize(all_items,settings);
    write_items_to_file("decomp/pokeemerald-expansion/data/scripts/item_ball_scripts.inc".to_string(),all_items);
}

//Function that handles the actual randomization
fn randomize(mut all_items: Vec<Item>,settings: &mut settings::Settings) -> Vec<Item>{
    if(settings.randomize_items == false){return all_items;}//No point in this function if randomization is off
    //Step one, get all the items we need to add to the pool
    let mut all_items_to_add = add_items_to_pool(settings);
    //Step two, randomize the items
    let all_items = randomize_vector_item(settings,all_items);
    //Step three, make a banned list of items
    //For example, if HM's aren't enabled, it will add the HM's
    let banned_list: Vec<String> = Vec::new();
    if settings.randomize_hms == false {
        banned_list.push_all(["ITEM_HM01","ITEM_HM02","ITEM_HM03","ITEM_HM04","ITEM_HM05","ITEM_HM06","ITEM_HM07","ITEM_HM08"]);
    }
    if settings.randomize_key_items == false{
        banned_list.push_all([
            "ITEM_BICYCLE",
            "ITEM_MACH_BIKE",
            "ITEM_ACRO_BIKE",
            "ITEM_OLD_ROD",
            "ITEM_GOOD_ROD",
            "ITEM_SUPER_ROD",
            "ITEM_DOWSING_MACHINE",
            "ITEM_TOWN_MAP",
            "ITEM_VS_SEEKER",
            "ITEM_TM_CASE",
            "ITEM_BERRY_POUCH",
            "ITEM_POKEMON_BOX_LINK",
            "ITEM_COIN_CASE",
            "ITEM_POWDER_JAR",
            "ITEM_WAILMER_PAIL",
            "ITEM_POKE_RADAR",
            "ITEM_POKEBLOCK_CASE",
            "ITEM_SOOT_SACK",
            "ITEM_POKE_FLUTE",
            "ITEM_FAME_CHECKER",
            "ITEM_TEACHY_TV",
            "ITEM_SS_TICKET",
            "ITEM_EON_TICKET",
            "ITEM_MYSTIC_TICKET",
            "ITEM_AURORA_TICKET",
            "ITEM_OLD_SEA_MAP",
            "ITEM_LETTER",
            "ITEM_DEVON_PARTS",
            "ITEM_GO_GOGGLES",
            "ITEM_DEVON_SCOPE",
            "ITEM_BASEMENT_KEY",
            "ITEM_SCANNER",
            "ITEM_STORAGE_KEY",
            "ITEM_KEY_TO_ROOM_1",
            "ITEM_KEY_TO_ROOM_2",
            "ITEM_KEY_TO_ROOM_4",
            "ITEM_KEY_TO_ROOM_6",
            "ITEM_METEORITE",
            "ITEM_MAGMA_EMBLEM",
            "ITEM_CONTEST_PASS",
            "ITEM_OAKS_PARCEL",
            "ITEM_SECRET_KEY",
            "ITEM_BIKE_VOUCHER",
            "ITEM_GOLD_TEETH",
            "ITEM_CARD_KEY",
            "ITEM_LIFT_KEY",
            "ITEM_SILPH_SCOPE",
            "ITEM_TRI_PASS",
            "ITEM_RAINBOW_PASS",
            "ITEM_TEA",
            "ITEM_RUBY",
            "ITEM_SAPPHIRE"
        ])
    }
    //Finally, combine the items
    let final_items: Vec<Item> = Vec::new();
    while(all_items.len() > 0){
        let cur_item = all_items.pop();
        //Check if the item is a trainer (if setting off)
        if settings.items_from_trainers == false && cur_item.location_type == Location_type::TRAINER{
            final_items.push(cur_item);
            continue;
        }
        //Check if the item location is in the banned list
        for banned in banned_list.iter(){
            if cur_item.item_name == banned{
                final_items.push(cur_item);
                continue;
            }
        }
        //Item to be added
        let cur_item_to_add = all_items_to_add.pop();
        //Check to make sure that the item is not a prereq of itself
        //TODO

    }
    return final_items;
}

//Order of item type in order of importance
/**
 * HM's
 * Story Key Items
 * Misc. Key Items
 * Battle Mechanic Key Items
 * Form-changing Key Items
 * Colored Orbs

 * ----------- All future are in one pool--------------------
 * Pokeballs
 * Medicine
 * Regional Specialties
 * Vitamins
 * EV Feathers
 * Ability Modifiers
 * Mints
 * Candy
 * Medicinal Flutes
 * Encounter-modifying Flutes
 * Encounter Modifiers
 * X Items
 * Escape Items
 * Treasures
 * Fossils
 * Mulch
 * Apricorns
 * Misc
 * Mail
 * Evolution Items
 * Nectars
 * Plates
 * Drives
 * Memories
 * Mega Stones
 * Gems
 * Z-Crystals
 * Species-specific Held Items
 * Incenses
 * Contest Scarves
 * EV Gain Modifiers
 * Type-boosting Held Items
 * Choice Items
 * Status Orbs
 * Weather Rocks
 * Terrain Seeds
 * Type Activated Stat Modifiers
 * Misc. Held Items
 * Berries
 * TM's
 * Charms
 * Gen IX Items
 */

fn add_items_to_pool(settings: &mut settings::Settings) -> Vec<String>{
    let data = fs::read_to_string("data/emerald/items.json").expect("unable to read file");
    let mut parsed_data = json::parse(&data).unwrap();
    let mut total_items: Vec<String> = Vec::new();
    //Key Items
    if(settings.randomize_hms){
        add_items_of_type(&mut parsed_data["HMs"],&mut total_items);
    }
    if(settings.randomize_key_items){
        add_items_of_type(&mut parsed_data["Story Key Items"],&mut total_items);
        add_items_of_type(&mut parsed_data["Misc. Key Items"],&mut total_items);
        add_items_of_type(&mut parsed_data["Battle Mechanic Key Items"],&mut total_items);
        add_items_of_type(&mut parsed_data["Form-changing Key Items"],&mut total_items);
        add_items_of_type(&mut parsed_data["Colored Orbs"],&mut total_items);
    }
    for i in 0..settings.add_rare_candy{
        total_items.push("ITEM_RARE_CANDY".to_string());
    }
    for i in 0..settings.add_pokeballs{
        total_items.push(parsed_data["PokeBalls"][settings::get_next_seed(0,20,settings) as usize].to_string());
    }
    let mut shuffled_items: Vec<String> = Vec::new();
    let item_types_to_add = vec!["Medicine","Potions","Potions","Potions","Potions","Potions",
    "Regional Specialties","Vitamins","EV Feathers","Ability Modifiers","Mints",
    "Candy","Medicinal Flutes","Encounter-modifying Flutes","Encounter Modifiers",
    "X Items","X Items","Escape Items","Escape Items","Treasures","Fossils",
    "Mulch","Misc","Apricorns","Mail","Evolution Items","Nectars","Plates",
    "Memories","Mega Stones","Gems","Z-Crystals","Species-specific Held Items",
    "Incenses","Contest Scarves","EV Gain Modifiers","Type-boosting Held Items",
    "Choice Items","Status Orbs","Weather Rocks","Terrain Seeds","Type Activated Stat Modifiers",
    "Misc. Held Items","Berries","TM's","Charms","Gen IX Items","HM's","HM's"];
    //This has many potions so that there are plenty of potions (probably the most important item to have multis of)
    for i in item_types_to_add.iter(){
        add_items_of_type(&mut parsed_data[*i],&mut shuffled_items);
    }
    shuffled_items = randomize_vector(settings,&mut shuffled_items)
    shuffled_items.append(final_items);
    return shuffled_items;
}

//This function takes a vector full of strings, and randomizes the order
fn randomize_vector(settings: &mut settings::Settings,items: &mut Vec<String>) -> Vec<String>{
    let mut randomized_items: Vec<String> = Vec::new();
    while(items.len() != 0){
        randomized_items.push(items.swap_remove(settings::get_next_seed(0,items.len() as i32,settings) as usize));
    }
    return randomized_items;
}
//Same as previous function, just with Items instead of strings
fn randomize_vector_item(settings: &mut settings::Settings,items: &mut Vec<Item>) -> Vec<Item>{
    let mut randomized_items: Vec<Item> = Vec::new();
    while(items.len() != 0){
        randomized_items.push(items.swap_remove(settings::get_next_seed(0,items.len() as i32,settings) as usize));
    }
    return randomized_items;
}

//Helper function for add_items_to_pool, adds items to the array so I don't have a ton of duplicate code
fn add_items_of_type(data: &mut json::JsonValue,total_items: &mut Vec<String>){
    let mut data_temp = data.clone();
    for i in 0..data_temp.len(){
        total_items.push(data_temp.pop().to_string());
        println!("{}:{}",total_items.len(),total_items[total_items.len()-1]);
    }
}

fn write_items_to_file(filename: String,items: Vec<Item>){
    let mut final_string: String = "".to_string();
    for cur_item in items{
        final_string.push_str(convert_item_to_function(cur_item).as_str());
    }
    fs::write(filename,final_string.to_string()).expect("couldn't write to file");
}

//Formats the item to a correctly functioning file to then be compiled
fn convert_item_to_function(cur_item: Item) -> String{
    if cur_item.item_script == "map.json"{
        //TODO Handle this
        return "".to_string();
    }
    let mut final_string = format!("{}::\n",cur_item.item_script);
    if cur_item.item_type == Item_type::POKEMON {
        final_string.push_str(format!("givemon {}, 25, ITEM_NONE\n",cur_item.item_name).as_str());
    }
    else if cur_item.item_type == Item_type::EGG {
        final_string.push_str(format!("giveegg {}\n",cur_item.item_name).as_str());
    }
    else if cur_item.item_type == Item_type::TRAP {
        //TODO traps
    }
    else if(cur_item.item_type == Item_type::NORMAL_ITEM){
        if(cur_item.location_type == Location_type::NPC || cur_item.location_type == Location_type::TRAINER){
            final_string.push_str(format!("giveitem {}\n",cur_item.item_name).as_str());
        }
        else{
            final_string.push_str(format!("finditem {}\n",cur_item.item_name).as_str());
        }
    }
    final_string.push_str("\nreturn\n\n");
    final_string
}

//Only used for testing purposes
fn convert_type(item: &Item_type) -> String{
    return match item{
        Item_type::NORMAL_ITEM => "Normal".to_string(),
        Item_type::TRAINER => "Trainer".to_string(),
        Item_type::EGG => "Egg".to_string(),
        Item_type::POKEMON => "Pokemon".to_string(),
        Item_type::TRAP => "Trap".to_string()
    };
}