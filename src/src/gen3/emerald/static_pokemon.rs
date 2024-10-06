use crate::src::gen3::trainers::{scale_pokemon, WallyTeam, MayBrendanTeam};
use crate::src::pokemon::{pokemon_to_formatted_name, PokemonStats, Pokemon};
use crate::src::settings;
use crate::src::gen3::wild_pokemon::get_legendary_pokemon;
use std::fs;

const MUSIC: [&str;20] = ["MUS_VS_KYOGRE_GROUDON","MUS_VS_RAYQUAZA","MUS_RG_VS_DEOXYS","MUS_RG_VS_LEGEND","MUS_VS_MEW","MUS_VS_REGI"
,"MUS_ENCOUNTER_MALE","MUS_ENCOUNTER_FEMALE","MUS_ENCOUNTER_GIRL","MUS_ENCOUNTER_INTENSE","MUS_ENCOUNTER_COOL","MUS_ENCOUNTER_AQUA"
,"MUS_ENCOUNTER_MAGMA","MUS_ENCOUNTER_SWIMMER","MUS_ENCOUNTER_TWINS","MUS_ENCOUNTER_ELITE_FOUR","MUS_ENCOUNTER_HIKER",
"MUS_ENCOUNTER_INTERVIEWER","MUS_ENCOUNTER_RICH","MUS_ENCOUNTER_SUSPICIOUS"];
fn get_music(settings: &mut settings::Settings) -> String{
    MUSIC[settings::get_next_seed(0, MUSIC.len() as i32 -1, settings) as usize].to_string()
}
const TRANSITION: [&str;17] = ["B_TRANSITION_KYOGRE","B_TRANSITION_GROUDON","B_TRANSITION_RAYQUAZA","B_TRANSITION_BLUR",
"B_TRANSITION_ANGLED_WIPES","B_TRANSITION_RIPPLE","B_TRANSITION_GRID_SQUARES","B_TRANSITION_REGIROCK","B_TRANSITION_REGICE","B_TRANSITION_REGISTEEL"
,"B_TRANSITION_CHAMPION","B_TRANSITION_SIDNEY","B_TRANSITION_PHOEBE","B_TRANSITION_GLACIA","B_TRANSITION_DRAKE","B_TRANSITION_MAGMA","B_TRANSITION_AQUA"];
fn get_transition(settings: &mut settings::Settings) -> String{
    TRANSITION[settings::get_next_seed(0, TRANSITION.len() as i32 -1, settings) as usize].to_string()
}
pub fn randomize_static_pokemon(settings: &mut settings::Settings,pokemon_data: &Vec<PokemonStats>,rival: &MayBrendanTeam,wally: &WallyTeam){
    if settings.randomize_wild_pokemon == false{
        fs::write("decomp/pokeemerald-expansion/src/battle_setup.c",fs::read_to_string("data/emerald/battle_setup/original").expect("Could not read original battle_setup")).expect("Could not write to battle_setup.c");
        return;
    }
    let mut file_read : Vec<String> = Vec::new();
    for i in 1..6{
        println!("{}",i);
        file_read.push(
            fs::read_to_string(format!("data/emerald/battle_setup/pt{}",i)).expect("Could not Read")
        );
    }
    let mut all_legends :Vec<Pokemon> = Vec::new();//This is a dumb fuckin hack that I should have forseen but I dont want to rewrite all my code so here we are
                                                    //All this variable does is make sure that there are no duplicate legends because that causes problems with a switch statement later on
    //Legendary Pokemon
    let groudon = get_legendary_pokemon(settings, pokemon_data, 70,&mut all_legends);
    let kyogre = get_legendary_pokemon(settings, pokemon_data, 70,&mut all_legends);
    let regirock = get_legendary_pokemon(settings, pokemon_data, 40,&mut all_legends);
    let regice = get_legendary_pokemon(settings, pokemon_data, 40,&mut all_legends);
    let registeel = get_legendary_pokemon(settings, pokemon_data, 40,&mut all_legends);
    let rayquaza = get_legendary_pokemon(settings, pokemon_data, 70,&mut all_legends);
    let mew = get_legendary_pokemon(settings, pokemon_data, 30,&mut all_legends);
    let lugia = get_legendary_pokemon(settings, pokemon_data, 70,&mut all_legends);
    let hooh = get_legendary_pokemon(settings, pokemon_data, 70,&mut all_legends);
    let deoxys = get_legendary_pokemon(settings, pokemon_data, 30,&mut all_legends);

    //Setup file
    fs::write("decomp/pokeemerald-expansion/src/battle_setup.c", format!("{}{}{}",
    fs::read_to_string("data/emerald/battle_setup.c").expect("Could not Read Battle setup"),
    pokemon_to_formatted_name(scale_pokemon(wally.ralt_substitute,5,pokemon_data,settings).pokemon_id,pokemon_data),
    fs::read_to_string("data/emerald/battle_setup2.c").expect("Could not Read Battle setup 2")
    )).expect("Could not write to file battle_setup.c");
}