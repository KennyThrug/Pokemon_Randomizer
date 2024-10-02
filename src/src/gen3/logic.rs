use crate::src::settings;

pub struct Connections{
    current_location: String,
    from_location: String,
    req_items: Vec<String>
}

//Checks if the item can go in this slot, returns false if it cannot, true if it can
pub fn check_logic(settings: &mut settings::Settings,item_name: String, location_name: String, prereqs: Vec<String>) -> bool{
    for cur_item in prereqs.iter(){
        if *cur_item == item_name{
            println!("Prereqs");
            return false;
        }
    }
    return check_area_for_item(Vec::new(),item_name,location_name,Vec::new());
}

fn check_area_for_item(previous_areas: Vec<String>,item_name : String,location_name: String,requirements: Vec<String>) -> bool{
    // println!("{} , {}",item_name,location_name.to_string());
    if item_name == "ITEM_HM02" && location_name != "Abandoned Ship"{
        // println!("Route 114 : {}, {}",(Locations::AbandonedShip as u8).to_string(),location_name.to_string());
        // println!("Mastah");
        return false;
    }
    return true;
}

fn add_connection(all_connections : &mut Vec<Connections>, current_location: String, previous_location: String, requirements: Vec<String>){

}

fn setup_all_connections() -> Vec<Connections>{
    let mut all_connections : Vec<Connections> = Vec::new();
    
    // add_connection(all_connections,current_location)

    return all_connections;
}