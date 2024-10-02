use crate::src::settings;

pub fn get_startup_stuff(settings: &mut settings::Settings) -> String{
    let mut final_string : String = "Startup_Stuff::\n".to_string();
    final_string.push_str(get_ball_code(settings).as_str());
    final_string.push_str(get_hm_code(settings).as_str());
    final_string.push_str(get_rare_candy_code(settings).as_str());
    final_string.push_str("return\n");
    return final_string
}


fn get_ball_code(settings: &mut settings::Settings) -> String{
    //TODO this
    return if settings.make_pokeballs_masterballs{
        "setflag FLAG_ALL_BALLS_MASTERBALL\n".to_string()
    }
    else{
        "clearflag FLAG_ALL_BALLS_MASTERBALL\n".to_string()
    }
}

fn get_hm_code(settings: &mut settings::Settings) -> String{
    return if settings.allow_hm_use{
        "clearflag FLAG_HMS_NEED_BADGES\n".to_string()
    }
    else{
        "setflag FLAG_HMS_NEED_BADGES\n".to_string()
    }
}

fn get_rare_candy_code(settings: &mut settings::Settings) -> String{
    return if settings.rare_candy_modification{
        "setflag FLAG_RARE_CANDY_MOD\n".to_string()
    }
    else{
        "clearflag FLAG_RARE_CANDY_MOD\n".to_string()
    }
}