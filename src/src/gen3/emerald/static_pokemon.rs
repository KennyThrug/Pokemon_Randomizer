use crate::src::gen3::trainers::{scale_pokemon, WallyTeam, MayBrendanTeam};
use crate::src::pokemon;
use crate::src::settings;
use crate::src::gen3::wild_pokemon;
use std::fs;

//These are leftover from a previous version that randomized music and transitions. Will keep to possibly be used later, but currently goes unused
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
pub fn randomize_static_pokemon(settings: &mut settings::Settings,pokemon_data: &Vec<pokemon::PokemonStats>,rival: &MayBrendanTeam,wally: &WallyTeam){
    fs::write("decomp/pokeemerald-expansion/src/battle_setup.c", format!("{}{}{}",
        fs::read_to_string("data/gen3/emerald/battle_setup.c").expect("Could not Read Battle setup"),
        pokemon::pokemon_to_formatted_name(scale_pokemon(wally.ralt_substitute,5,pokemon_data,settings).pokemon_id,pokemon_data),
        fs::read_to_string("data/gen3/emerald/battle_setup2.c").expect("Could not Read Battle setup 2")
        )).expect("Could not write to file battle_setup.c");

    legendary_pokemon(settings,pokemon_data)
}

fn get_legends(settings: &mut settings::Settings,pokemon_data: &Vec<pokemon::PokemonStats>) -> Vec<pokemon::Pokemon>{
    if !settings.randomize_wild_pokemon{
        return vec![
            pokemon::Pokemon::Registeel,
            pokemon::Pokemon::Deoxys,
            pokemon::Pokemon::Regirock,
            pokemon::Pokemon::Mew,
            pokemon::Pokemon::Regice,
            pokemon::Pokemon::Kyogre,
            pokemon::Pokemon::Hooh,
            pokemon::Pokemon::Rayquaza,
            pokemon::Pokemon::Groudon
            ];
    }
    let levels = vec![40,30,40,30,40,70,70,70,70,70];
    let mut legends: Vec<pokemon::Pokemon> = Vec::new();
    for i in 0..9{
        legends.push(wild_pokemon::get_legendary_pokemon(settings,pokemon_data,levels[i],&legends).pokemon_id);
    }
    return legends;
}

fn legendary_pokemon(settings: &mut settings::Settings,pokemon_data: &Vec<pokemon::PokemonStats>){
    let all_legendary_pokemon = get_legends(settings,pokemon_data);
    let legendary_pokemon_str = format!("AncientTomb_EventScript_Registeel::
        lock
        faceplayer
        waitse
        playmoncry {Registeel}, CRY_MODE_ENCOUNTER
        delay 40
        waitmoncry
        setwildbattle {Registeel}, 40
        setflag FLAG_SYS_CTRL_OBJ_DELETE
        special StartRegiBattle
        waitstate
        clearflag FLAG_SYS_CTRL_OBJ_DELETE
        specialvar VAR_RESULT, GetBattleOutcome
        goto_if_eq VAR_RESULT, B_OUTCOME_WON, AncientTomb_EventScript_DefeatedRegisteel
        goto_if_eq VAR_RESULT, B_OUTCOME_RAN, AncientTomb_EventScript_RanFromRegisteel
        goto_if_eq VAR_RESULT, B_OUTCOME_PLAYER_TELEPORTED, AncientTomb_EventScript_RanFromRegisteel
        setflag FLAG_DEFEATED_REGISTEEL
        release
        end
    
    BirthIsland_Exterior_EventScript_Deoxys::
        waitse
        setfieldeffectargument 0, LOCALID_BIRTH_ISLAND_EXTERIOR_ROCK
        setfieldeffectargument 1, MAP_NUM(BIRTH_ISLAND_EXTERIOR)
        setfieldeffectargument 2, MAP_GROUP(BIRTH_ISLAND_EXTERIOR)
        dofieldeffect FLDEFF_DESTROY_DEOXYS_ROCK
        playbgm MUS_RG_ENCOUNTER_DEOXYS, FALSE
        waitfieldeffect FLDEFF_DESTROY_DEOXYS_ROCK
        addobject LOCALID_DEOXYS
        applymovement LOCALID_DEOXYS, BirthIsland_Exterior_Movement_DeoxysApproach
        waitmovement 0
        waitse
        playmoncry {Deoxys}, CRY_MODE_ENCOUNTER
        delay 40
        waitmoncry
        setvar VAR_LAST_TALKED, LOCALID_DEOXYS
        seteventmon {Deoxys}, 30
        setflag FLAG_SYS_CTRL_OBJ_DELETE
        special BattleSetup_StartLegendaryBattle
        waitstate
        clearflag FLAG_SYS_CTRL_OBJ_DELETE
        specialvar VAR_RESULT, GetBattleOutcome
        goto_if_eq VAR_RESULT, B_OUTCOME_WON, BirthIsland_Exterior_EventScript_DefeatedDeoxys
        goto_if_eq VAR_RESULT, B_OUTCOME_RAN, BirthIsland_Exterior_EventScript_RanFromDeoxys
        goto_if_eq VAR_RESULT, B_OUTCOME_PLAYER_TELEPORTED, BirthIsland_Exterior_EventScript_RanFromDeoxys
        setflag FLAG_BATTLED_DEOXYS
        release
        end
    
    DesertRuins_EventScript_Regirock::
        lock
        faceplayer
        waitse
        playmoncry {Regirock}, CRY_MODE_ENCOUNTER
        delay 40
        waitmoncry
        setwildbattle {Regirock}, 40
        setflag FLAG_SYS_CTRL_OBJ_DELETE
        special StartRegiBattle
        waitstate
        clearflag FLAG_SYS_CTRL_OBJ_DELETE
        specialvar VAR_RESULT, GetBattleOutcome
        goto_if_eq VAR_RESULT, B_OUTCOME_WON, DesertRuins_EventScript_DefeatedRegirock
        goto_if_eq VAR_RESULT, B_OUTCOME_RAN, DesertRuins_EventScript_RanFromRegirock
        goto_if_eq VAR_RESULT, B_OUTCOME_PLAYER_TELEPORTED, DesertRuins_EventScript_RanFromRegirock
        setflag FLAG_DEFEATED_REGIROCK
        release
        end
    
    FarawayIsland_Interior_EventScript_Mew::
        lock
        faceplayer
        applymovement LOCALID_FARAWAY_ISLAND_MEW, FarawayIsland_Interior_Movement_MewAppear
        waitmovement 0
        setvar VAR_0x8004, 0
        special SetMewAboveGrass
        message FarawayIsland_Interior_Text_Mew
        waitse
        playmoncry {Mew}, CRY_MODE_ENCOUNTER
        call_if_eq VAR_FACING, DIR_NORTH, FarawayIsland_Interior_EventScript_FoundMewNorth
        call_if_eq VAR_FACING, DIR_SOUTH, FarawayIsland_Interior_EventScript_FoundMewSouth
        call_if_eq VAR_FACING, DIR_WEST, FarawayIsland_Interior_EventScript_FoundMewWest
        call_if_eq VAR_FACING, DIR_EAST, FarawayIsland_Interior_EventScript_FoundMewEast
        special DestroyMewEmergingGrassSprite
        delay 40
        waitmoncry
        seteventmon {Mew}, 30
        setflag FLAG_SYS_CTRL_OBJ_DELETE
        special BattleSetup_StartLegendaryBattle
        waitstate
        clearflag FLAG_SYS_CTRL_OBJ_DELETE
        specialvar VAR_RESULT, GetBattleOutcome
        goto_if_eq VAR_RESULT, B_OUTCOME_WON, FarawayIsland_Interior_EventScript_MewDefeated
        goto_if_eq VAR_RESULT, B_OUTCOME_RAN, FarawayIsland_Interior_EventScript_PlayerOrMewRan
        goto_if_eq VAR_RESULT, B_OUTCOME_PLAYER_TELEPORTED, FarawayIsland_Interior_EventScript_PlayerOrMewRan
        goto_if_eq VAR_RESULT, B_OUTCOME_MON_TELEPORTED, FarawayIsland_Interior_EventScript_PlayerOrMewRan
        setflag FLAG_CAUGHT_MEW
        release
        end
    
    IslandCave_EventScript_Regice::
        lock
        faceplayer
        waitse
        playmoncry {Regice}, CRY_MODE_ENCOUNTER
        delay 40
        waitmoncry
        setwildbattle {Regice}, 40
        setflag FLAG_SYS_CTRL_OBJ_DELETE
        special StartRegiBattle
        waitstate
        clearflag FLAG_SYS_CTRL_OBJ_DELETE
        specialvar VAR_RESULT, GetBattleOutcome
        goto_if_eq VAR_RESULT, B_OUTCOME_WON, IslandCave_EventScript_DefeatedRegice
        goto_if_eq VAR_RESULT, B_OUTCOME_RAN, IslandCave_EventScript_RanFromRegice
        goto_if_eq VAR_RESULT, B_OUTCOME_PLAYER_TELEPORTED, IslandCave_EventScript_RanFromRegice
        setflag FLAG_DEFEATED_REGICE
        release
        end
    
    MarineCave_End_EventScript_Kyogre::
        lockall
        applymovement OBJ_EVENT_ID_PLAYER, Common_Movement_FaceUp
        waitmovement 0
        applymovement LOCALID_KYOGRE, MarineCave_End_Movement_KyogreApproach
        waitmovement 0
        waitse
        playmoncry {Kyogre}, CRY_MODE_ENCOUNTER
        delay 40
        waitmoncry
        setvar VAR_LAST_TALKED, LOCALID_KYOGRE
        setwildbattle {Kyogre}, 70
        setflag FLAG_SYS_CTRL_OBJ_DELETE
        special BattleSetup_StartLegendaryBattle
        waitstate
        clearflag FLAG_SYS_CTRL_OBJ_DELETE
        setvar VAR_TEMP_1, 0
        specialvar VAR_RESULT, GetBattleOutcome
        goto_if_eq VAR_RESULT, B_OUTCOME_WON, MarineCave_End_EventScript_DefeatedKyogre
        goto_if_eq VAR_RESULT, B_OUTCOME_RAN, MarineCave_End_EventScript_RanFromKyogre
        goto_if_eq VAR_RESULT, B_OUTCOME_PLAYER_TELEPORTED, MarineCave_End_EventScript_RanFromKyogre
        setvar VAR_SHOULD_END_ABNORMAL_WEATHER, 1
        setflag FLAG_DEFEATED_KYOGRE
        releaseall
        end
    
    NavelRock_Top_EventScript_HoOh::
        lockall
        setvar VAR_TEMP_1, 1
        special SpawnCameraObject
        setvar VAR_0x8004, 3  @ num loops
        setvar VAR_0x8005, 35 @ delay
        special LoopWingFlapSE
        applymovement LOCALID_HO_OH, NavelRock_Top_Movement_HoOhAppear
        applymovement OBJ_EVENT_ID_CAMERA, NavelRock_Top_Movement_CameraPanUp
        waitmovement 0
        delay 50
        setweather WEATHER_NONE
        doweather
        waitse
        playmoncry {Hooh}, CRY_MODE_ENCOUNTER
        delay 30
        waitmoncry
        delay 60
        setvar VAR_0x8004, 3  @ num loops
        setvar VAR_0x8005, 35 @ delay
        special LoopWingFlapSE
        applymovement OBJ_EVENT_ID_CAMERA, NavelRock_Top_Movement_CameraPanDown
        applymovement LOCALID_HO_OH, NavelRock_Top_Movement_HoOhApproach
        waitmovement 0
        special RemoveCameraObject
        seteventmon {Hooh}, 70
        setflag FLAG_SYS_CTRL_OBJ_DELETE
        special BattleSetup_StartLegendaryBattle
        waitstate
        clearflag FLAG_SYS_CTRL_OBJ_DELETE
        setvar VAR_LAST_TALKED, LOCALID_HO_OH
        specialvar VAR_RESULT, GetBattleOutcome
        goto_if_eq VAR_RESULT, B_OUTCOME_WON, NavelRock_Top_EventScript_DefeatedHoOh
        goto_if_eq VAR_RESULT, B_OUTCOME_RAN, NavelRock_Top_EventScript_RanFromHoOh
        goto_if_eq VAR_RESULT, B_OUTCOME_PLAYER_TELEPORTED, NavelRock_Top_EventScript_RanFromHoOh
        setflag FLAG_CAUGHT_HO_OH
        releaseall
        end
        
SkyPillar_Top_EventScript_Rayquaza::
	lockall
	waitse
	playmoncry {Rayquaza}, CRY_MODE_ENCOUNTER
	delay 40
	waitmoncry
	setwildbattle {Rayquaza}, 70
	setflag FLAG_SYS_CTRL_OBJ_DELETE
	special BattleSetup_StartLegendaryBattle
	waitstate
	clearflag FLAG_SYS_CTRL_OBJ_DELETE
	specialvar VAR_RESULT, GetBattleOutcome
	goto_if_eq VAR_RESULT, B_OUTCOME_WON, SkyPillar_Top_EventScript_DefeatedRayquaza
	goto_if_eq VAR_RESULT, B_OUTCOME_RAN, SkyPillar_Top_EventScript_RanFromRayquaza
	goto_if_eq VAR_RESULT, B_OUTCOME_PLAYER_TELEPORTED, SkyPillar_Top_EventScript_RanFromRayquaza
	setflag FLAG_DEFEATED_RAYQUAZA
	releaseall
	end

TerraCave_End_EventScript_Groudon::
	lockall
	applymovement OBJ_EVENT_ID_PLAYER, Common_Movement_FaceUp
	waitmovement 0
	applymovement LOCALID_GROUDON, TerraCave_End_Movement_GroudonApproach
	waitmovement 0
	waitse
	playmoncry {Groudon}, CRY_MODE_ENCOUNTER
	delay 40
	waitmoncry
	setvar VAR_LAST_TALKED, LOCALID_GROUDON
	setwildbattle {Groudon}, 70
	setflag FLAG_SYS_CTRL_OBJ_DELETE
	special BattleSetup_StartLegendaryBattle
	waitstate
	clearflag FLAG_SYS_CTRL_OBJ_DELETE
	setvar VAR_TEMP_1, 0
	specialvar VAR_RESULT, GetBattleOutcome
	goto_if_eq VAR_RESULT, B_OUTCOME_WON, TerraCave_End_EventScript_DefeatedGroudon
	goto_if_eq VAR_RESULT, B_OUTCOME_RAN, TerraCave_End_EventScript_RanFromGroudon
	goto_if_eq VAR_RESULT, B_OUTCOME_PLAYER_TELEPORTED, TerraCave_End_EventScript_RanFromGroudon
	setvar VAR_SHOULD_END_ABNORMAL_WEATHER, 1
	setflag FLAG_DEFEATED_GROUDON
	releaseall
	end",
    Registeel=pokemon::pokemon_to_formatted_name(all_legendary_pokemon[0],pokemon_data),
    Deoxys=pokemon::pokemon_to_formatted_name(all_legendary_pokemon[1],pokemon_data),
    Regirock=pokemon::pokemon_to_formatted_name(all_legendary_pokemon[2],pokemon_data),
    Mew=pokemon::pokemon_to_formatted_name(all_legendary_pokemon[3],pokemon_data),
    Regice=pokemon::pokemon_to_formatted_name(all_legendary_pokemon[4],pokemon_data),
    Kyogre=pokemon::pokemon_to_formatted_name(all_legendary_pokemon[5],pokemon_data),
    Hooh=pokemon::pokemon_to_formatted_name(all_legendary_pokemon[6],pokemon_data),
    Rayquaza=pokemon::pokemon_to_formatted_name(all_legendary_pokemon[7],pokemon_data),
    Groudon=pokemon::pokemon_to_formatted_name(all_legendary_pokemon[8],pokemon_data));
    fs::write("decomp/pokeemerald-expansion/data/scripts/randomizer_legendary.inc",legendary_pokemon_str).expect("Could not write to file randomizer_legendary.inc");
}