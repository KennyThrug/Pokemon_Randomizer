use std::fs;
use crate::src::settings;

pub fn add_line_to_spoiler(line: String, settings: &mut settings::Settings){
    if !settings.show_spoiler{return;}
    settings.spoiler.push_str(format!("\n{}",line.as_str()).as_str());
}

pub fn create_spoiler_log(settings: &mut settings::Settings){
    if !settings.show_spoiler{return;}
    fs::write(format!("out/{}/spoiler.txt",settings.seed),&settings.spoiler);
}