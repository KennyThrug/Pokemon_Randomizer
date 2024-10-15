use crate::src::gen3::emerald;
use crate::src::settings;

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

pub fn randomizer_scripts(settings: &mut settings::Settings) -> String{
    if settings.game == settings::Game::Emerald{
        return "decomp/pokeemerald-expansion/data/scripts/randomizer_scripts.inc".to_string();
    }
    return "".to_string();
}

pub fn startup_stuff(settings: &mut settings::Settings) -> String{
    if settings.game == settings::Game::Emerald{
        return emerald::startup_stuff::get_startup_stuff(settings);
    }
    return "\nreturn".to_string();
}