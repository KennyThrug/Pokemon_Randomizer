use crate::src::gen3::emerald;
use crate::src::settings;

pub fn convert_to_location(location: String,settings: &mut settings::Settings) -> u8{
    if settings.game == settings::Game::Emerald{
        return emerald::logic::convert_to_location(location) as u8;
    }
    return 0
}

pub fn get_item_locations(settings: &mut settings::Settings) -> String{
    if settings.game == settings::Game::Emerald{
        return "data/emerald/item_locations.csv".to_string();
    }
    return "".to_string();
}

pub fn get_items(settings: &mut settings::Settings) -> String{
    if settings.game == settings::Game::Emerald{
        return "data/emerald/items.json".to_string();
    }
    return "".to_string();
}

//Returns true if item can go in slot, false if it cannot
pub fn get_logic(settings: &mut settings::Settings,item_name: String, location_name: u8, prereqs: Vec<String>) -> bool{
    if settings.game == settings::Game::Emerald{
        return emerald::logic::check_logic(settings,item_name,location_name,prereqs)
    }
    return true
}

pub fn startup_stuff(settings: &mut settings::Settings) -> String{
    if settings.game == settings::Game::Emerald{
        return emerald::startup_stuff::get_startup_stuff(settings);
    }
    return "\nreturn".to_string();
}