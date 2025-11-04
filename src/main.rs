use std::fs;
use src::settings::read_json_for_settings;
mod src;
use crate::src::game_chooser;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    println!("hello world");
    // let mut file = fs::File::open("settings.json")?;
    // println!("heo");
    let stringy = fs::read_to_string("./settings.json")?;
    println!("{}",stringy);
    let mut settings = read_json_for_settings(stringy).unwrap();
    game_chooser::main_randomizer_script(&mut settings);
    println!("finally");
    Ok(())
}