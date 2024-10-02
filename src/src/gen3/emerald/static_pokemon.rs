use crate::src::gen3::trainers::{scale_pokemon, WallyTeam, MayBrendanTeam};
use crate::src::pokemon::{format_pokemon_name, PokemonStats, Pokemon};
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
        // fs::write("decomp/pokeemerald-expansion/src/battle_setup.c",fs::read_to_string("data/emerald/battle_setup/original").expect("Could not read original battle_setup")).expect("Could not write to battle_setup.c");
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

//     //Setup file
//     fs::write("decomp/pokeemerald-expansion/src/battle_setup.c", format!("{}{}{}{}:
//     gBattleTypeFlags |= BATTLE_TYPE_GROUDON;
//     CreateBattleStartTask({}, {});
//     break;
// case {}:
// gBattleTypeFlags |= BATTLE_TYPE_KYOGRE;
// CreateBattleStartTask({}, {});
// break;
// case {}:
// gBattleTypeFlags |= BATTLE_TYPE_RAYQUAZA;
// CreateBattleStartTask({}, {});
// break;
// case {}:
// CreateBattleStartTask({}, {});
// break;
// case {}:
// case {}:
// CreateBattleStartTask({}, {});
// break;
// case {}:
// CreateBattleStartTask({}, {});
// break;
// }}

// IncrementGameStat(GAME_STAT_TOTAL_BATTLES);
// IncrementGameStat(GAME_STAT_WILD_BATTLES);
// IncrementDailyWildBattles();
// TryUpdateGymLeaderRematchFromWild();
// }}

// void StartGroudonKyogreBattle(void)
// {{
// LockPlayerFieldControls();
// gMain.savedCallback = CB2_EndScriptedWildBattle;
// gBattleTypeFlags = BATTLE_TYPE_LEGENDARY | BATTLE_TYPE_KYOGRE_GROUDON;

// if (gGameVersion == VERSION_RUBY)
// CreateBattleStartTask(B_TRANSITION_ANGLED_WIPES, MUS_VS_KYOGRE_GROUDON); // GROUDON
// else
// CreateBattleStartTask(B_TRANSITION_RIPPLE, MUS_VS_KYOGRE_GROUDON); // KYOGRE

// IncrementGameStat(GAME_STAT_TOTAL_BATTLES);
// IncrementGameStat(GAME_STAT_WILD_BATTLES);
// IncrementDailyWildBattles();
// TryUpdateGymLeaderRematchFromWild();
// }}

// void StartRegiBattle(void)
// {{
// u8 transitionId;
// u16 species;

// LockPlayerFieldControls();
// gMain.savedCallback = CB2_EndScriptedWildBattle;
// gBattleTypeFlags = BATTLE_TYPE_LEGENDARY | BATTLE_TYPE_REGI;

// species = GetMonData(&gEnemyParty[0], MON_DATA_SPECIES);
// switch (species)
// {{
// case {}:
// transitionId = {};
// break;
// case {}:
// transitionId = {};
// break;
// case {}:
// transitionId = {}{}{};

// if (gTrainers[gTrainerBattleOpponent_A].trainerClass == TRAINER_CLASS_ELITE_FOUR)
// {{
//     if (gTrainerBattleOpponent_A == TRAINER_SIDNEY)
//     return {};
//     if (gTrainerBattleOpponent_A == TRAINER_PHOEBE)
//         return {};
//         if (gTrainerBattleOpponent_A == TRAINER_GLACIA)
//             return {};
//             if (gTrainerBattleOpponent_A == TRAINER_DRAKE)
//                 return {};
//                 return {};
//             }}
        
//             if (gTrainers[gTrainerBattleOpponent_A].trainerClass == TRAINER_CLASS_CHAMPION)
//                 return {};

//                 if (gTrainers[gTrainerBattleOpponent_A].trainerClass == TRAINER_CLASS_TEAM_MAGMA
//                     || gTrainers[gTrainerBattleOpponent_A].trainerClass == TRAINER_CLASS_MAGMA_LEADER
//                     || gTrainers[gTrainerBattleOpponent_A].trainerClass == TRAINER_CLASS_MAGMA_ADMIN)
//                     return {};

//                     if (gTrainers[gTrainerBattleOpponent_A].trainerClass == TRAINER_CLASS_TEAM_AQUA
//                         || gTrainers[gTrainerBattleOpponent_A].trainerClass == TRAINER_CLASS_AQUA_LEADER
//                         || gTrainers[gTrainerBattleOpponent_A].trainerClass == TRAINER_CLASS_AQUA_ADMIN)
//                         return {};
// {}{};
// break;
// case TRAINER_ENCOUNTER_MUSIC_FEMALE:
// music = {};
// break;
// case TRAINER_ENCOUNTER_MUSIC_GIRL:
// music = {};
// break;
// case TRAINER_ENCOUNTER_MUSIC_INTENSE:
// music = {};
// break;
// case TRAINER_ENCOUNTER_MUSIC_COOL:
// music = {};
// break;
// case TRAINER_ENCOUNTER_MUSIC_AQUA:
// music = {};
// break;
// case TRAINER_ENCOUNTER_MUSIC_MAGMA:
// music = {};
// break;
// case TRAINER_ENCOUNTER_MUSIC_SWIMMER:
// music = {};
// break;
// case TRAINER_ENCOUNTER_MUSIC_TWINS:
// music = {};
// break;
// case TRAINER_ENCOUNTER_MUSIC_ELITE_FOUR:
// music = {};
// break;
// case TRAINER_ENCOUNTER_MUSIC_HIKER:
// music = {};
// break;
// case TRAINER_ENCOUNTER_MUSIC_INTERVIEWER:
// music = {};
// break;
// case TRAINER_ENCOUNTER_MUSIC_RICH:
// music = {};
// break;
// default:
// music = {}{}"
//     ,file_read[0]
//     ,format_pokemon_name(scale_pokemon(wally.ralt_substitute,5,pokemon_data,settings).pokemon_name)//Wallys Pokemon
//     ,file_read[1]
//     ,format_pokemon_name(groudon.pokemon_name)
//     ,get_transition(settings)
//     ,get_music(settings)
//     ,format_pokemon_name(kyogre.pokemon_name)
//     ,get_transition(settings)
//     ,get_music(settings)
//     ,format_pokemon_name(rayquaza.pokemon_name)
//     ,get_transition(settings)
//     ,get_music(settings)
//     ,format_pokemon_name(deoxys.pokemon_name)
//     ,get_transition(settings)
//     ,get_music(settings)
//     ,format_pokemon_name(lugia.pokemon_name)
//     ,format_pokemon_name(hooh.pokemon_name)
//     ,get_transition(settings)
//     ,get_music(settings)
//     ,format_pokemon_name(mew.pokemon_name)
//     ,get_transition(settings)
//     ,get_music(settings)
//     ,format_pokemon_name(regirock.pokemon_name)
//     ,get_transition(settings)
//     ,format_pokemon_name(regice.pokemon_name)
//     ,get_transition(settings)
//     ,format_pokemon_name(registeel.pokemon_name)
//     ,get_transition(settings)
//     ,file_read[2]
//     ,get_transition(settings)
//     ,get_transition(settings)
//     ,get_transition(settings)
//     ,get_transition(settings)
//     ,get_transition(settings)
//     ,get_transition(settings)
//     ,get_transition(settings)
//     ,get_transition(settings)
//     ,get_transition(settings)
//     ,file_read[3]
//     ,get_music(settings)
//     ,get_music(settings)
//     ,get_music(settings)
//     ,get_music(settings)
//     ,get_music(settings)
//     ,get_music(settings)
//     ,get_music(settings)
//     ,get_music(settings)
//     ,get_music(settings)
//     ,get_music(settings)
//     ,get_music(settings)
//     ,get_music(settings)
//     ,get_music(settings)
//     ,get_music(settings)
//     ,file_read[4]
//     )).expect("Could not write to file battle_setup.c");
}