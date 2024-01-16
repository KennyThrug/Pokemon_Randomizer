use std::fs;

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
        all_items.push(
            Item{
                item_name: cur_item[0].to_string(),
                item_script: cur_item[4].to_string(),
                location_type: parse_location_type(cur_item[3].to_string()),
                item_type: parse_item_type(cur_item[2].to_string()),
                item_hidden: false,
                prerequisites: Vec::new() //TODO fix the prereqs & item_hidden
            }
        );
    }
    return all_items;
}

pub fn randomize_items(){
    let mut all_items = get_all_items("data/emerald/item_locations.csv".to_string());
    randomize(&mut all_items);
    write_items_to_file("decomp/pokeemerald-expansion/data/scripts/item_ball_scripts.inc".to_string(),all_items);
}

pub fn randomize(all_items: &mut Vec<Item>){
    //TODO this
}

fn write_items_to_file(filename: String,items: Vec<Item>){
    let mut final_string: String = "".to_string();
    for cur_item in items{
        final_string.push_str(convert_item_to_function(cur_item).as_str());
    }
    println!("test1");
    fs::write(filename,final_string.to_string()).expect("couldn't write to file");
    println!("test2");
}

fn convert_item_to_function(cur_item: Item) -> String{
    println!("{} is of type: {}",&cur_item.item_script,convert_type(&cur_item.item_type));
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

fn convert_type(item: &Item_type) -> String{
    return match item{
        Item_type::NORMAL_ITEM => "Normal".to_string(),
        Item_type::TRAINER => "Trainer".to_string(),
        Item_type::EGG => "Egg".to_string(),
        Item_type::POKEMON => "Pokemon".to_string(),
        Item_type::TRAP => "Trap".to_string()
    };
}