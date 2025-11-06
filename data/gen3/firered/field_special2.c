};

static u16 GetStarterSpeciesById(u16 idx)
{
    if (idx >= NELEMS(sStarterSpecies))
        idx = 0;
    return sStarterSpecies[idx];
}

u16 GetStarterSpecies(void)
{
    return GetStarterSpeciesById(VarGet(VAR_STARTER_MON));
}

void SetSeenMon(void)
{
    GetSetPokedexFlag(SpeciesToNationalDexNum(gSpecialVar_0x8004), 2);
}

void ResetContextNpcTextColor(void)
{
    gSelectedObjectEvent = 0;
    gSpecialVar_TextColor = NPC_TEXT_COLOR_DEFAULT;
}

static u8 GetFollowerGender(void)
{
    struct Pokemon *firstLiveMon = GetFirstLiveMon();
    if (firstLiveMon == NULL)
        return MON_GENDERLESS;
    return GetMonGender(firstLiveMon);
}

static u8 GetFollowerTextColor(void)
{
    switch (GetFollowerGender())
    {
    case MON_FEMALE:
        return NPC_TEXT_COLOR_FEMALE;
    case MON_MALE:
        return NPC_TEXT_COLOR_MALE;
    case MON_GENDERLESS:
    default:
        return NPC_TEXT_COLOR_NEUTRAL;
    }
}

u8 ContextNpcGetTextColor(void)
{
    u16 gfxId;
    if (gSpecialVar_TextColor != NPC_TEXT_COLOR_DEFAULT)
    {
        // A text color has been specified, use that
        return gSpecialVar_TextColor;
    }
    else if (gSelectedObjectEvent == 0)
    {
        // No text color specified and no object selected, use neutral
        return NPC_TEXT_COLOR_NEUTRAL;
    }
    else
    {
        // An object is selected and no color has been specified.
        // Use the text color normally associated with this object's sprite.
        struct MapPosition position;
        u8 objEventId;
        GetInFrontOfPlayerPosition(&position);
        objEventId = GetObjectEventIdByPosition(position.x, position.y, position.elevation);
        if (objEventId < OBJECT_EVENTS_COUNT)
            gfxId = gObjectEvents[objEventId].graphicsId;
        else
            gfxId = gObjectEvents[gSelectedObjectEvent].graphicsId;
        
        if (gfxId >= OBJ_EVENT_GFX_MON_BASE)
            return GetFollowerTextColor();

        if (gfxId >= OBJ_EVENT_GFX_VAR_0)
            gfxId = VarGetObjectEventGraphicsId(gfxId - OBJ_EVENT_GFX_VAR_0);
        return GetColorFromTextColorTable(gfxId);
    }
}

static bool8 HasMonBeenRenamed(u8 idx)
{
    struct Pokemon * pokemon = &gPlayerParty[idx];
    u8 language;
    GetMonData(pokemon, MON_DATA_NICKNAME, gStringVar1);
    language = GetMonData(pokemon, MON_DATA_LANGUAGE, &language);
    if (language != LANGUAGE_ENGLISH)
        return TRUE;
    else if (StringCompare(gSpeciesInfo[GetMonData(pokemon, MON_DATA_SPECIES, NULL)].speciesName, gStringVar1) != 0)
        return TRUE;
    else
        return FALSE;
}

bool8 HasLeadMonBeenRenamed(void)
{
    return HasMonBeenRenamed(GetLeadMonIndex());
}

void TV_PrintIntToStringVar(u8 varidx, s32 number)
{
    s32 n = CountDigits(number);
    ConvertIntToDecimalStringN(sStringVarPtrs[varidx], number, STR_CONV_MODE_LEFT_ALIGN, n);
}

s32 CountDigits(s32 number)
{
    if (number / 10 == 0)
        return 1;
    else if (number / 100 == 0)
        return 2;
    else if (number / 1000 == 0)
        return 3;
    else if (number / 10000 == 0)
        return 4;
    else if (number / 100000 == 0)
        return 5;
    else if (number / 1000000 == 0)
        return 6;
    else if (number / 10000000 == 0)
        return 7;
    else if (number / 100000000 == 0)
        return 8;
    else
        return 1;
}

bool8 NameRaterWasNicknameChanged(void)
{
    struct Pokemon * pokemon = &gPlayerParty[gSpecialVar_0x8004];
    GetMonData(pokemon, MON_DATA_NICKNAME, gStringVar1);
    if (StringCompare(gStringVar3, gStringVar1) == 0)
        return FALSE;
    else
        return TRUE;
}

void ChangeBoxPokemonNickname(void)
{
    struct BoxPokemon * pokemon = GetBoxedMonPtr(gSpecialVar_MonBoxId, gSpecialVar_MonBoxPos);
    u16 species;
    u8 gender;
    u32 personality;


    GetBoxMonData(pokemon, MON_DATA_NICKNAME, gStringVar3);
    GetBoxMonData(pokemon, MON_DATA_NICKNAME, gStringVar2);
    species = GetBoxMonData(pokemon, MON_DATA_SPECIES, NULL);
    gender = GetBoxMonGender(pokemon);
    personality = GetBoxMonData(pokemon, MON_DATA_PERSONALITY, NULL);
    DoNamingScreen(NAMING_SCREEN_NICKNAME, gStringVar2, species, gender, personality, ChangeBoxPokemonNickname_CB);
}

static void ChangeBoxPokemonNickname_CB(void)
{
    SetBoxMonNickAt(gSpecialVar_MonBoxId, gSpecialVar_MonBoxPos, gStringVar2);
    CB2_ReturnToFieldContinueScriptPlayMapMusic();
}

void ChangePokemonNickname(void)
{
    u16 species;
    u8 gender;
    u32 personality;

    GetMonData(&gPlayerParty[gSpecialVar_0x8004], MON_DATA_NICKNAME, gStringVar3);
    GetMonData(&gPlayerParty[gSpecialVar_0x8004], MON_DATA_NICKNAME, gStringVar2);
    species = GetMonData(&gPlayerParty[gSpecialVar_0x8004], MON_DATA_SPECIES, NULL);
    gender = GetMonGender(&gPlayerParty[gSpecialVar_0x8004]);
    personality = GetMonData(&gPlayerParty[gSpecialVar_0x8004], MON_DATA_PERSONALITY, NULL);
    DoNamingScreen(NAMING_SCREEN_NICKNAME, gStringVar2, species, gender, personality, ChangePokemonNickname_CB);
}

static void ChangePokemonNickname_CB(void)
{
    SetMonData(&gPlayerParty[gSpecialVar_0x8004], MON_DATA_NICKNAME, gStringVar2);
    CB2_ReturnToFieldContinueScriptPlayMapMusic();
}

void BufferMonNickname(void)
{
    GetMonData(&gPlayerParty[gSpecialVar_0x8004], MON_DATA_NICKNAME, gStringVar1);
    StringGet_Nickname(gStringVar1);
}

void IsMonOTIDNotPlayers(void)
{
    if (GetPlayerTrainerId() == GetMonData(&gPlayerParty[gSpecialVar_0x8004], MON_DATA_OT_ID, NULL))
        gSpecialVar_Result = FALSE;
    else
        gSpecialVar_Result = TRUE;
}

u32 GetPlayerTrainerId(void)
{
    return (gSaveBlock2Ptr->playerTrainerId[3] << 24) | (gSaveBlock2Ptr->playerTrainerId[2] << 16) | (gSaveBlock2Ptr->playerTrainerId[1] << 8) | gSaveBlock2Ptr->playerTrainerId[0];
}

u8 GetUnlockedSeviiAreas(void)
{
    u8 result = 0;
    if (FlagGet(FLAG_WORLD_MAP_ONE_ISLAND) == TRUE)
        result |= 1 << 0;
    if (FlagGet(FLAG_WORLD_MAP_TWO_ISLAND) == TRUE)
        result |= 1 << 1;
    if (FlagGet(FLAG_WORLD_MAP_THREE_ISLAND) == TRUE)
        result |= 1 << 2;
    if (FlagGet(FLAG_WORLD_MAP_FOUR_ISLAND) == TRUE)
        result |= 1 << 3;
    if (FlagGet(FLAG_WORLD_MAP_FIVE_ISLAND) == TRUE)
        result |= 1 << 4;
    if (FlagGet(FLAG_WORLD_MAP_SIX_ISLAND) == TRUE)
        result |= 1 << 5;
    if (FlagGet(FLAG_WORLD_MAP_SEVEN_ISLAND) == TRUE)
        result |= 1 << 6;
    return result;
}

void UpdateTrainerCardPhotoIcons(void)
{
    u16 species[PARTY_SIZE];
    u32 personality[PARTY_SIZE];
    u8 i;
    u8 partyCount;
    for (i = 0; i < PARTY_SIZE; i++)
        species[i] = SPECIES_NONE;
    partyCount = CalculatePlayerPartyCount();
    for (i = 0; i < partyCount; i++)
    {
        species[i] = GetMonData(&gPlayerParty[i], MON_DATA_SPECIES_OR_EGG, NULL);
        personality[i] = GetMonData(&gPlayerParty[i], MON_DATA_PERSONALITY, NULL);
    }
    VarSet(VAR_TRAINER_CARD_MON_ICON_1, SpeciesToMailSpecies(species[0], personality[0]));
    VarSet(VAR_TRAINER_CARD_MON_ICON_2, SpeciesToMailSpecies(species[1], personality[1]));
    VarSet(VAR_TRAINER_CARD_MON_ICON_3, SpeciesToMailSpecies(species[2], personality[2]));
    VarSet(VAR_TRAINER_CARD_MON_ICON_4, SpeciesToMailSpecies(species[3], personality[3]));
    VarSet(VAR_TRAINER_CARD_MON_ICON_5, SpeciesToMailSpecies(species[4], personality[4]));
    VarSet(VAR_TRAINER_CARD_MON_ICON_6, SpeciesToMailSpecies(species[5], personality[5]));
    VarSet(VAR_TRAINER_CARD_MON_ICON_TINT_IDX, gSpecialVar_0x8004);
}

u16 StickerManGetBragFlags(void)
{
    u16 result = 0;
    u32 numEggs;
    gSpecialVar_0x8004 = GetGameStat(GAME_STAT_ENTERED_HOF);
    numEggs = GetGameStat(GAME_STAT_HATCHED_EGGS);
    gSpecialVar_0x8006 = GetGameStat(GAME_STAT_LINK_BATTLE_WINS);
    if (numEggs > 0xFFFF)
        gSpecialVar_0x8005 = 0xFFFF;
    else
        gSpecialVar_0x8005 = numEggs;
    if (gSpecialVar_0x8004 != 0)
        result |= 1 << 0;
    if (gSpecialVar_0x8005 != 0)
        result |= 1 << 1;
    if (gSpecialVar_0x8006 != 0)
        result |= 1 << 2;
    return result;
}

u16 GetHiddenItemAttr(u32 hiddenItem, u8 attr)
{
    if (attr == HIDDEN_ITEM_ITEM)
        return GET_HIDDEN_ITEM_ITEM(hiddenItem);
    else if (attr == HIDDEN_ITEM_FLAG)
        return GET_HIDDEN_ITEM_FLAG(hiddenItem) + FLAG_HIDDEN_ITEMS_START;
    else if (attr == HIDDEN_ITEM_QUANTITY)
        return GET_HIDDEN_ITEM_QUANTITY(hiddenItem);
    else if (attr == HIDDEN_ITEM_UNDERFOOT)
        return GET_HIDDEN_ITEM_UNDERFOOT(hiddenItem);
    else // Invalid
        return 1;
}

bool8 DoesPlayerPartyContainSpecies(void)
{
    u8 partyCount = CalculatePlayerPartyCount();
    u8 i;
    for (i = 0; i < partyCount; i++)
    {
        if (GetMonData(&gPlayerParty[i], MON_DATA_SPECIES_OR_EGG, NULL) == gSpecialVar_0x8004)
            return TRUE;
    }
    return FALSE;
}

static const u8 sMartMaps[][3] = {
    {MAP(VIRIDIAN_CITY_MART),   1},
    {MAP(PEWTER_CITY_MART),     3},
    {MAP(CERULEAN_CITY_MART),   1},
    {MAP(LAVENDER_TOWN_MART),   1},
    {MAP(VERMILION_CITY_MART),  1},
    {MAP(FUCHSIA_CITY_MART),    1},
    {MAP(CINNABAR_ISLAND_MART), 1},
    {MAP(SAFFRON_CITY_MART),    1},
    {MAP(THREE_ISLAND_MART),    1},
    {MAP(FOUR_ISLAND_MART),     1},
    {MAP(SEVEN_ISLAND_MART),    1},
    {MAP(SIX_ISLAND_MART),      1}
};

u8 GetMartClerkObjectId(void)
{
    u8 i;
    for (i = 0; i < NELEMS(sMartMaps); i++)
    {
        if (gSaveBlock1Ptr->location.mapGroup == sMartMaps[i][0] && gSaveBlock1Ptr->location.mapNum == sMartMaps[i][1])
            return sMartMaps[i][2];
    }
    return 1;
}

void SetUsedPkmnCenterQuestLogEvent(void)
{
    SetQuestLogEvent(QL_EVENT_USED_PKMN_CENTER, NULL);
}

static const struct {
    u16 inside_grp;
    u16 inside_num;
    u16 outside_grp;
    u16 outside_num;
} sInsideOutsidePairs[] = {
    [QL_LOCATION_HOME]               = {MAP(PALLET_TOWN_PLAYERS_HOUSE_1F),          MAP(PALLET_TOWN)},
    [QL_LOCATION_OAKS_LAB]           = {MAP(PALLET_TOWN_PROFESSOR_OAKS_LAB),        MAP(PALLET_TOWN)},
    [QL_LOCATION_VIRIDIAN_GYM]       = {MAP(VIRIDIAN_CITY_GYM),                     MAP(VIRIDIAN_CITY)},
    [QL_LOCATION_LEAGUE_GATE_1]      = {MAP(ROUTE22_NORTH_ENTRANCE),                MAP(ROUTE22)},
    [QL_LOCATION_LEAGUE_GATE_2]      = {MAP(ROUTE22_NORTH_ENTRANCE),                MAP(ROUTE23)},
    [QL_LOCATION_VIRIDIAN_FOREST_1]  = {MAP(VIRIDIAN_FOREST),                       MAP(ROUTE2_VIRIDIAN_FOREST_SOUTH_ENTRANCE)},
    [QL_LOCATION_VIRIDIAN_FOREST_2]  = {MAP(VIRIDIAN_FOREST),                       MAP(ROUTE2_VIRIDIAN_FOREST_NORTH_ENTRANCE)},
    [QL_LOCATION_PEWTER_MUSEUM]      = {MAP(PEWTER_CITY_MUSEUM_1F),                 MAP(PEWTER_CITY)},
    [QL_LOCATION_PEWTER_GYM]         = {MAP(PEWTER_CITY_GYM),                       MAP(PEWTER_CITY)},
    [QL_LOCATION_MT_MOON_1]          = {MAP(MT_MOON_1F),                            MAP(ROUTE4)},
    [QL_LOCATION_MT_MOON_2]          = {MAP(MT_MOON_B1F),                           MAP(ROUTE4)},
    [QL_LOCATION_CERULEAN_GYM]       = {MAP(CERULEAN_CITY_GYM),                     MAP(CERULEAN_CITY)},
    [QL_LOCATION_BIKE_SHOP]          = {MAP(CERULEAN_CITY_BIKE_SHOP),               MAP(CERULEAN_CITY)},
    [QL_LOCATION_BILLS_HOUSE]        = {MAP(ROUTE25_SEA_COTTAGE),                   MAP(ROUTE25)},
    [QL_LOCATION_DAY_CARE]           = {MAP(ROUTE5_POKEMON_DAY_CARE),               MAP(ROUTE5)},
    [QL_LOCATION_UNDERGROUND_PATH_1] = {MAP(UNDERGROUND_PATH_NORTH_ENTRANCE),       MAP(ROUTE5)},
    [QL_LOCATION_UNDERGROUND_PATH_2] = {MAP(UNDERGROUND_PATH_SOUTH_ENTRANCE),       MAP(ROUTE6)},
    [QL_LOCATION_PKMN_FAN_CLUB]      = {MAP(VERMILION_CITY_POKEMON_FAN_CLUB),       MAP(VERMILION_CITY)},
    [QL_LOCATION_VERMILION_GYM]      = {MAP(VERMILION_CITY_GYM),                    MAP(VERMILION_CITY)},
    [QL_LOCATION_SS_ANNE]            = {MAP(SSANNE_1F_CORRIDOR),                    MAP(VERMILION_CITY)},
    [QL_LOCATION_DIGLETTS_CAVE_1]    = {MAP(DIGLETTS_CAVE_NORTH_ENTRANCE),          MAP(ROUTE2)},
    [QL_LOCATION_DIGLETTS_CAVE_2]    = {MAP(DIGLETTS_CAVE_SOUTH_ENTRANCE),          MAP(ROUTE11)},
    [QL_LOCATION_ROCK_TUNNEL_1]      = {MAP(ROCK_TUNNEL_1F),                        MAP(ROUTE10)},
    [QL_LOCATION_ROCK_TUNNEL_2]      = {MAP(ROCK_TUNNEL_1F),                        MAP(ROUTE10)},
    [QL_LOCATION_POWER_PLANT]        = {MAP(POWER_PLANT),                           MAP(ROUTE10)},
    [QL_LOCATION_PKMN_TOWER]         = {MAP(POKEMON_TOWER_1F),                      MAP(LAVENDER_TOWN)},
    [QL_LOCATION_VOLUNTEER_HOUSE]    = {MAP(LAVENDER_TOWN_VOLUNTEER_POKEMON_HOUSE), MAP(LAVENDER_TOWN)},
    [QL_LOCATION_NAME_RATERS_HOUSE]  = {MAP(LAVENDER_TOWN_HOUSE2),                  MAP(LAVENDER_TOWN)},
    [QL_LOCATION_UNDERGROUND_PATH_3] = {MAP(UNDERGROUND_PATH_EAST_ENTRANCE),        MAP(ROUTE8)},
    [QL_LOCATION_UNDERGROUND_PATH_4] = {MAP(UNDERGROUND_PATH_WEST_ENTRANCE),        MAP(ROUTE7)},
    [QL_LOCATION_CELADON_DEPT_STORE] = {MAP(CELADON_CITY_DEPARTMENT_STORE_1F),      MAP(CELADON_CITY)},
    [QL_LOCATION_CELADON_MANSION]    = {MAP(CELADON_CITY_CONDOMINIUMS_1F),          MAP(CELADON_CITY)},
    [QL_LOCATION_GAME_CORNER]        = {MAP(CELADON_CITY_GAME_CORNER),              MAP(CELADON_CITY)},
    [QL_LOCATION_CELADON_GYM]        = {MAP(CELADON_CITY_GYM),                      MAP(CELADON_CITY)},
    [QL_LOCATION_CELADON_RESTAURANT] = {MAP(CELADON_CITY_RESTAURANT),               MAP(CELADON_CITY)},
    [QL_LOCATION_ROCKET_HIDEOUT]     = {MAP(ROCKET_HIDEOUT_B1F),                    MAP(CELADON_CITY_GAME_CORNER)},
    [QL_LOCATION_SAFARI_ZONE]        = {MAP(SAFARI_ZONE_CENTER),                    MAP(FUCHSIA_CITY_SAFARI_ZONE_ENTRANCE)},
    [QL_LOCATION_FUCHSIA_GYM]        = {MAP(FUCHSIA_CITY_GYM),                      MAP(FUCHSIA_CITY)},
    [QL_LOCATION_WARDENS_HOME]       = {MAP(FUCHSIA_CITY_WARDENS_HOUSE),            MAP(FUCHSIA_CITY)},
    [QL_LOCATION_FIGHTING_DOJO]      = {MAP(SAFFRON_CITY_DOJO),                     MAP(SAFFRON_CITY)},
    [QL_LOCATION_SAFFRON_GYM]        = {MAP(SAFFRON_CITY_GYM),                      MAP(SAFFRON_CITY)},
    [QL_LOCATION_SILPH_CO]           = {MAP(SILPH_CO_1F),                           MAP(SAFFRON_CITY)},
    [QL_LOCATION_SEAFOAM_ISLANDS_1]  = {MAP(SEAFOAM_ISLANDS_1F),                    MAP(ROUTE20)},
    [QL_LOCATION_SEAFOAM_ISLANDS_2]  = {MAP(SEAFOAM_ISLANDS_1F),                    MAP(ROUTE20)},
    [QL_LOCATION_PKMN_MANSION]       = {MAP(POKEMON_MANSION_1F),                    MAP(CINNABAR_ISLAND)},
    [QL_LOCATION_CINNABAR_GYM]       = {MAP(CINNABAR_ISLAND_GYM),                   MAP(CINNABAR_ISLAND)},
    [QL_LOCATION_CINNABAR_LAB]       = {MAP(CINNABAR_ISLAND_POKEMON_LAB_ENTRANCE),  MAP(CINNABAR_ISLAND)},
    [QL_LOCATION_VICTORY_ROAD_1]     = {MAP(VICTORY_ROAD_1F),                       MAP(ROUTE23)},
    [QL_LOCATION_VICTORY_ROAD_2]     = {MAP(VICTORY_ROAD_2F),                       MAP(ROUTE23)},
    [QL_LOCATION_PKMN_LEAGUE]        = {MAP(INDIGO_PLATEAU_POKEMON_CENTER_1F),      MAP(INDIGO_PLATEAU_EXTERIOR)},
    [QL_LOCATION_CERULEAN_CAVE]      = {MAP(CERULEAN_CAVE_1F),                      MAP(CERULEAN_CITY)}
};

void QuestLog_CheckDepartingIndoorsMap(void)
{
    u8 i;
    for (i = 0; i < NELEMS(sInsideOutsidePairs); i++)
    {
        if (gSaveBlock1Ptr->location.mapGroup == sInsideOutsidePairs[i].inside_grp && gSaveBlock1Ptr->location.mapNum == sInsideOutsidePairs[i].inside_num)
        {
            if (VarGet(VAR_QL_ENTRANCE) != QL_LOCATION_ROCKET_HIDEOUT || i != QL_LOCATION_GAME_CORNER)
            {
                VarSet(VAR_QL_ENTRANCE, i);
                FlagSet(FLAG_SYS_QL_DEPARTED);
            }
            break;
        }
    }
}

void QuestLog_TryRecordDepartedLocation(void)
{
    s16 x, y;
    struct QuestLogEvent_Departed data;
    u16 locationId = VarGet(VAR_QL_ENTRANCE);
    data.mapSec = 0;
    data.locationId = 0;
    if (FlagGet(FLAG_SYS_QL_DEPARTED))
    {
        if (locationId == QL_LOCATION_VIRIDIAN_FOREST_1)
        {
            if (gSaveBlock1Ptr->location.mapGroup == MAP_GROUP(ROUTE2_VIRIDIAN_FOREST_SOUTH_ENTRANCE)
              && (gSaveBlock1Ptr->location.mapNum == MAP_NUM(ROUTE2_VIRIDIAN_FOREST_SOUTH_ENTRANCE)
               || gSaveBlock1Ptr->location.mapNum == MAP_NUM(ROUTE2_VIRIDIAN_FOREST_NORTH_ENTRANCE)))
            {
                data.mapSec = MAPSEC_ROUTE_2;
                if (gSaveBlock1Ptr->location.mapNum == MAP_NUM(ROUTE2_VIRIDIAN_FOREST_SOUTH_ENTRANCE))
                    data.locationId = locationId;
                else
                    data.locationId = locationId + 1;
                SetQuestLogEvent(QL_EVENT_DEPARTED, (const u16 *)&data);
                FlagClear(FLAG_SYS_QL_DEPARTED);
                return;
            }
        }
        else if (locationId == QL_LOCATION_LEAGUE_GATE_1)
        {
            if (gSaveBlock1Ptr->location.mapGroup == MAP_GROUP(ROUTE22) &&
                (gSaveBlock1Ptr->location.mapNum == MAP_NUM(ROUTE22)
              || gSaveBlock1Ptr->location.mapNum == MAP_NUM(ROUTE23)))
            {
                data.mapSec = Overworld_GetMapHeaderByGroupAndId(sInsideOutsidePairs[locationId].inside_grp, sInsideOutsidePairs[locationId].inside_num)->regionMapSectionId;
                if (gSaveBlock1Ptr->location.mapNum == MAP_NUM(ROUTE22))
                    data.locationId = locationId;
                else
                    data.locationId = locationId + 1;
                SetQuestLogEvent(QL_EVENT_DEPARTED, (const u16 *)&data);
                FlagClear(FLAG_SYS_QL_DEPARTED);
                return;
            }
        }
        if (gSaveBlock1Ptr->location.mapGroup == sInsideOutsidePairs[locationId].outside_grp
           && gSaveBlock1Ptr->location.mapNum == sInsideOutsidePairs[locationId].outside_num)
        {
            data.mapSec = Overworld_GetMapHeaderByGroupAndId(sInsideOutsidePairs[locationId].inside_grp, sInsideOutsidePairs[locationId].inside_num)->regionMapSectionId;
            data.locationId = locationId;
            if (locationId == QL_LOCATION_ROCK_TUNNEL_1)
            {
                PlayerGetDestCoords(&x, &y);
                if (x != 15 || y != 26)
                    data.locationId++;
            }
            else if (locationId == QL_LOCATION_SEAFOAM_ISLANDS_1)
            {
                PlayerGetDestCoords(&x, &y);
                if (x != 67 || y != 15)
                    data.locationId++;
            }
            SetQuestLogEvent(QL_EVENT_DEPARTED, (const u16 *)&data);
            FlagClear(FLAG_SYS_QL_DEPARTED);
            if (locationId == QL_LOCATION_ROCKET_HIDEOUT)
            {
                VarSet(VAR_QL_ENTRANCE, QL_LOCATION_GAME_CORNER);
                FlagSet(FLAG_SYS_QL_DEPARTED);
            }
        }
    }
}

u16 GetMysteryGiftCardStat(void)
{
    switch (gSpecialVar_Result)
    {
    case GET_NUM_STAMPS:
        return MysteryGift_GetCardStat(CARD_STAT_NUM_STAMPS);
    case GET_MAX_STAMPS:
        return MysteryGift_GetCardStat(CARD_STAT_MAX_STAMPS);
    case GET_CARD_BATTLES_WON:
        return MysteryGift_GetCardStat(CARD_STAT_BATTLES_WON);
    case GET_CARD_BATTLES_LOST:
        return MysteryGift_GetCardStat(CARD_STAT_BATTLES_LOST);
    case GET_CARD_NUM_TRADES:
        return MysteryGift_GetCardStat(CARD_STAT_NUM_TRADES);
    default:
        AGB_ASSERT_EX(0, ABSPATH("scr_tool.c"), 3873);
        return 0;
    }
}

void SetPCBoxToSendMon(u8 boxId)
{
    sPCBoxToSendMon = boxId;
}

u16 GetPCBoxToSendMon(void)
{
    return sPCBoxToSendMon;
}

bool8 ShouldShowBoxWasFullMessage(void)
{
    if (FlagGet(FLAG_SHOWN_BOX_WAS_FULL_MESSAGE))
        return FALSE;
    if (StorageGetCurrentBox() == VarGet(VAR_PC_BOX_TO_SEND_MON))
        return FALSE;
    FlagSet(FLAG_SHOWN_BOX_WAS_FULL_MESSAGE);
    return TRUE;
}

bool8 IsDestinationBoxFull(void)
{
    s32 i;
    s32 j;
    SetPCBoxToSendMon(VarGet(VAR_PC_BOX_TO_SEND_MON));
    i = StorageGetCurrentBox();
    do
    {
        for (j = 0; j < IN_BOX_COUNT; j++)
        {
            if (GetBoxMonData(GetBoxedMonPtr(i, j), MON_DATA_SPECIES, NULL) == SPECIES_NONE)
            {
                if (GetPCBoxToSendMon() != i)
                    FlagClear(FLAG_SHOWN_BOX_WAS_FULL_MESSAGE);
                VarSet(VAR_PC_BOX_TO_SEND_MON, i);
                return ShouldShowBoxWasFullMessage();
            }
        }
        i++;
        if (i == TOTAL_BOXES_COUNT)
            i = 0;
    } while (i != StorageGetCurrentBox());
    return FALSE;
}

const u16 sPokeCenter1FMaps[] = {
    MAP_VIRIDIAN_CITY_POKEMON_CENTER_1F,
    MAP_PEWTER_CITY_POKEMON_CENTER_1F,
    MAP_CERULEAN_CITY_POKEMON_CENTER_1F,
    MAP_LAVENDER_TOWN_POKEMON_CENTER_1F,
    MAP_VERMILION_CITY_POKEMON_CENTER_1F,
    MAP_CELADON_CITY_POKEMON_CENTER_1F,
    MAP_FUCHSIA_CITY_POKEMON_CENTER_1F,
    MAP_CINNABAR_ISLAND_POKEMON_CENTER_1F,
    MAP_INDIGO_PLATEAU_POKEMON_CENTER_1F,
    MAP_SAFFRON_CITY_POKEMON_CENTER_1F,
    MAP_ROUTE4_POKEMON_CENTER_1F,
    MAP_ROUTE10_POKEMON_CENTER_1F,
    MAP_ONE_ISLAND_POKEMON_CENTER_1F,
    MAP_TWO_ISLAND_POKEMON_CENTER_1F,
    MAP_THREE_ISLAND_POKEMON_CENTER_1F,
    MAP_FOUR_ISLAND_POKEMON_CENTER_1F,
    MAP_FIVE_ISLAND_POKEMON_CENTER_1F,
    MAP_SEVEN_ISLAND_POKEMON_CENTER_1F,
    MAP_SIX_ISLAND_POKEMON_CENTER_1F,
    MAP_UNION_ROOM,
    MAP_UNDEFINED
};

bool8 UsedPokemonCenterWarp(void)
{
    s32 i;
    u16 mapno = (gLastUsedWarp.mapGroup << 8) + gLastUsedWarp.mapNum;
    for (i = 0; sPokeCenter1FMaps[i] != MAP_UNDEFINED; i++)
    {
        if (sPokeCenter1FMaps[i] == mapno)
            return TRUE;
    }
    return FALSE;
}

bool8 BufferTMHMMoveName(void)
{
    // 8004 = item ID
    if (IsItemTMHM(gSpecialVar_0x8004))
    {
        StringCopy(gStringVar1, gMovesInfo[ItemIdToBattleMoveId(gSpecialVar_0x8004)].name);
        return TRUE;
    }
    else
        return FALSE;
}

void RunMassageCooldownStepCounter(void)
{
    u16 count = VarGet(VAR_MASSAGE_COOLDOWN_STEP_COUNTER);
    if (count < 500)
        VarSet(VAR_MASSAGE_COOLDOWN_STEP_COUNTER, count + 1);
}

void DaisyMassageServices(void)
{
    AdjustFriendship(&gPlayerParty[gSpecialVar_0x8004], FRIENDSHIP_EVENT_MASSAGE);
    VarSet(VAR_MASSAGE_COOLDOWN_STEP_COUNTER, 0);
}

static const u16 sEliteFourLightingPalettes[][16] = {
    INCBIN_U16("graphics/field_specials/elite_four_lighting_0.gbapal"),
    INCBIN_U16("graphics/field_specials/elite_four_lighting_1.gbapal"),
    INCBIN_U16("graphics/field_specials/elite_four_lighting_2.gbapal"),
    INCBIN_U16("graphics/field_specials/elite_four_lighting_3.gbapal"),
    INCBIN_U16("graphics/field_specials/elite_four_lighting_4.gbapal"),
    INCBIN_U16("graphics/field_specials/elite_four_lighting_5.gbapal"),
    INCBIN_U16("graphics/field_specials/elite_four_lighting_6.gbapal"),
    INCBIN_U16("graphics/field_specials/elite_four_lighting_7.gbapal"),
    INCBIN_U16("graphics/field_specials/elite_four_lighting_8.gbapal"),
    INCBIN_U16("graphics/field_specials/elite_four_lighting_9.gbapal"),
    INCBIN_U16("graphics/field_specials/elite_four_lighting_10.gbapal"),
    INCBIN_U16("graphics/field_specials/elite_four_lighting_11.gbapal")
};

static const u16 sChampionRoomLightingPalettes[][16] = {
    INCBIN_U16("graphics/field_specials/champion_room_lighting_0.gbapal"),
    INCBIN_U16("graphics/field_specials/champion_room_lighting_1.gbapal"),
    INCBIN_U16("graphics/field_specials/champion_room_lighting_2.gbapal"),
    INCBIN_U16("graphics/field_specials/champion_room_lighting_3.gbapal"),
    INCBIN_U16("graphics/field_specials/champion_room_lighting_4.gbapal"),
    INCBIN_U16("graphics/field_specials/champion_room_lighting_5.gbapal"),
    INCBIN_U16("graphics/field_specials/champion_room_lighting_6.gbapal"),
    INCBIN_U16("graphics/field_specials/champion_room_lighting_7.gbapal"),
    INCBIN_U16("graphics/field_specials/champion_room_lighting_8.gbapal")
};

static const u8 sEliteFourLightingTimers[] = {
    40,
    12,
    12,
    12,
    12,
    12,
    12,
    12,
    12,
    12,
    12
};

static const u8 sChampionRoomLightingTimers[] = {
    20,
     8,
     8,
     8,
     8,
     8,
     8,
     8
};

void DoPokemonLeagueLightingEffect(void)
{
    u8 taskId = CreateTask(Task_RunPokemonLeagueLightingEffect, 8);
    s16 *data = gTasks[taskId].data;
    if (FlagGet(FLAG_TEMP_3) == TRUE)
    {
        gTasks[taskId].func = Task_CancelPokemonLeagueLightingEffect;
    }
    else
    {
        if (gSaveBlock1Ptr->location.mapGroup == MAP_GROUP(POKEMON_LEAGUE_CHAMPIONS_ROOM) && gSaveBlock1Ptr->location.mapNum == MAP_NUM(POKEMON_LEAGUE_CHAMPIONS_ROOM))
        {
            data[0] = sChampionRoomLightingTimers[0];
            data[2] = 8;
            LoadPalette(sChampionRoomLightingPalettes[0], BG_PLTT_ID(7), PLTT_SIZE_4BPP);
        }
        else
        {
            data[0] = sEliteFourLightingTimers[0];
            data[2] = 11;
            LoadPalette(sEliteFourLightingPalettes[0], BG_PLTT_ID(7), PLTT_SIZE_4BPP);
        }
        data[1] = 0;
        ApplyGlobalTintToPaletteSlot(7, 1);
    }
}

static void Task_RunPokemonLeagueLightingEffect(u8 taskId)
{
    s16 *data = gTasks[taskId].data;
    if (!gPaletteFade.active
     && FlagGet(FLAG_TEMP_2) != FALSE
     && FlagGet(FLAG_TEMP_5) != TRUE
     && gGlobalFieldTintMode != QL_TINT_BACKUP_GRAYSCALE
     && --data[0] == 0
    )
    {
        if (++data[1] == data[2])
            data[1] = 0;

        if (gSaveBlock1Ptr->location.mapGroup == MAP_GROUP(POKEMON_LEAGUE_CHAMPIONS_ROOM) && gSaveBlock1Ptr->location.mapNum == MAP_NUM(POKEMON_LEAGUE_CHAMPIONS_ROOM))
        {
            data[0] = sChampionRoomLightingTimers[data[1]];
            LoadPalette(sChampionRoomLightingPalettes[data[1]], BG_PLTT_ID(7), PLTT_SIZE_4BPP);
        }
        else
        {
            data[0] = sEliteFourLightingTimers[data[1]];
            LoadPalette(sEliteFourLightingPalettes[data[1]], BG_PLTT_ID(7), PLTT_SIZE_4BPP);
        }
        ApplyGlobalTintToPaletteSlot(7, 1);
    }
}

static void Task_CancelPokemonLeagueLightingEffect(u8 taskId)
{
    if (FlagGet(FLAG_TEMP_4) != FALSE)
    {
        if (gSaveBlock1Ptr->location.mapGroup == MAP_GROUP(POKEMON_LEAGUE_CHAMPIONS_ROOM) && gSaveBlock1Ptr->location.mapNum == MAP_NUM(POKEMON_LEAGUE_CHAMPIONS_ROOM))
            LoadPalette(sChampionRoomLightingPalettes[8], BG_PLTT_ID(7), PLTT_SIZE_4BPP);
        else
            LoadPalette(sEliteFourLightingPalettes[11], BG_PLTT_ID(7), PLTT_SIZE_4BPP);
        ApplyGlobalTintToPaletteSlot(7, 1);
        if (gPaletteFade.active)
        {
            BlendPalettes(0x00000080, 16, RGB_BLACK);
        }
        DestroyTask(taskId);
    }
}

void StopPokemonLeagueLightingEffectTask(void)
{
    if (FuncIsActiveTask(Task_RunPokemonLeagueLightingEffect) == TRUE)
    {
        DestroyTask(FindTaskIdByFunc(Task_RunPokemonLeagueLightingEffect));
    }
}

bool8 CapeBrinkGetMoveToTeachLeadPokemon(void)
{
    // Returns:
    //   8005 = Move tutor index
    //   8006 = Num moves known by lead mon
    //   8007 = Index of lead mon
    //   to specialvar = whether a move can be taught in the first place
    u8 i, leadMonSlot, moveCount = 0;
    u16 moveId, tutorFlag; 
    struct Pokemon *leadMon;
    
    leadMonSlot = GetLeadMonIndex();
    leadMon = &gPlayerParty[leadMonSlot];
    
    if (GetMonData(leadMon, MON_DATA_FRIENDSHIP) != 255)
        return FALSE;

    moveId = GetFirstPartnerMove(GetMonData(leadMon, MON_DATA_SPECIES_OR_EGG));
    switch(moveId)
    {
        case MOVE_FRENZY_PLANT:
            tutorFlag = FLAG_TUTOR_FRENZY_PLANT;
            break;
        case MOVE_BLAST_BURN:
            tutorFlag = FLAG_TUTOR_BLAST_BURN;
            break;
        case MOVE_HYDRO_CANNON:
            tutorFlag = FLAG_TUTOR_HYDRO_CANNON;
            break;
        default:
            return FALSE;
    }
    
    StringCopy(gStringVar2, gMovesInfo[moveId].name);
    if (!I_REUSABLE_TMS && FlagGet(tutorFlag) == TRUE)
        return FALSE;
    
    for (i = 0; i < MAX_MON_MOVES; i++)
        moveCount += (GetMonData(leadMon, MON_DATA_MOVE1 + i) != MOVE_NONE);
    
    gSpecialVar_0x8005 = moveId;
    gSpecialVar_0x8006 = moveCount;
    gSpecialVar_0x8007 = leadMonSlot;

    return TRUE;
}

bool8 HasLearnedAllMovesFromCapeBrinkTutor(void)
{
    // 8005 is set by CapeBrinkGetMoveToTeachLeadPokemon
    if (I_REUSABLE_TMS)
    {
        return FALSE;
    }

    switch (gSpecialVar_0x8005)
    {
        case MOVE_FRENZY_PLANT:
            FlagSet(FLAG_TUTOR_FRENZY_PLANT);
            break;
        case MOVE_BLAST_BURN:
            FlagSet(FLAG_TUTOR_BLAST_BURN);
            break;
        case MOVE_HYDRO_CANNON:
            FlagSet(FLAG_TUTOR_HYDRO_CANNON);
            break;
    }

    return (FlagGet(FLAG_TUTOR_FRENZY_PLANT) == TRUE)
        && (FlagGet(FLAG_TUTOR_BLAST_BURN) == TRUE)
        && (FlagGet(FLAG_TUTOR_HYDRO_CANNON) == TRUE);
}

bool8 CutMoveRuinValleyCheck(void)
{
    if (FlagGet(FLAG_USED_CUT_ON_RUIN_VALLEY_BRAILLE) != TRUE
     && gSaveBlock1Ptr->location.mapGroup == MAP_GROUP(SIX_ISLAND_RUIN_VALLEY)
     && gSaveBlock1Ptr->location.mapNum == MAP_NUM(SIX_ISLAND_RUIN_VALLEY)
     && gSaveBlock1Ptr->pos.x == 24
     && gSaveBlock1Ptr->pos.y == 25
     && GetPlayerFacingDirection() == DIR_NORTH
    )
        return TRUE;
    else
        return FALSE;
}

void CutMoveOpenDottedHoleDoor(void)
{
    MapGridSetMetatileIdAt(31, 31, METATILE_SeviiIslands67_DottedHoleDoor_Open);
    DrawWholeMapView();
    PlaySE(SE_BANG);
    FlagSet(FLAG_USED_CUT_ON_RUIN_VALLEY_BRAILLE);
    UnlockPlayerFieldControls();
}

static const u16 sDeoxysObjectPals[][16] = {
    INCBIN_U16("graphics/field_specials/deoxys_rock_0.gbapal"),
    INCBIN_U16("graphics/field_specials/deoxys_rock_1.gbapal"),
    INCBIN_U16("graphics/field_specials/deoxys_rock_2.gbapal"),
    INCBIN_U16("graphics/field_specials/deoxys_rock_3.gbapal"),
    INCBIN_U16("graphics/field_specials/deoxys_rock_4.gbapal"),
    INCBIN_U16("graphics/field_specials/deoxys_rock_5.gbapal"),
    INCBIN_U16("graphics/field_specials/deoxys_rock_6.gbapal"),
    INCBIN_U16("graphics/field_specials/deoxys_rock_7.gbapal"),
    INCBIN_U16("graphics/field_specials/deoxys_rock_8.gbapal"),
    INCBIN_U16("graphics/field_specials/deoxys_rock_9.gbapal"),
    INCBIN_U16("graphics/field_specials/deoxys_rock_10.gbapal")
};

static const u8 sDeoxysCoords[][2] = {
    {15, 12},
    {11, 14},
    {15,  8},
    {19, 14},
    {12, 11},
    {18, 11},
    {15, 14},
    {11, 14},
    {19, 14},
    {15, 15},
    {15, 10}
};

static const u8 sDeoxysStepCaps[] = {
    4,
    8,
    8,
    8,
    4,
    4,
    4,
    6,
    3,
    3
};

void DoDeoxysTriangleInteraction(void)
{
    CreateTask(Task_DoDeoxysTriangleInteraction, 8);
}

static void Task_DoDeoxysTriangleInteraction(u8 taskId)
{
    u16 r5;
    u16 r6;
    if (FlagGet(FLAG_SYS_DEOXYS_AWAKENED) == TRUE)
    {
        gSpecialVar_Result = 3;
        ScriptContext_Enable();
        DestroyTask(taskId);
    }
    else
    {
        r5 = VarGet(VAR_DEOXYS_INTERACTION_NUM);
        r6 = VarGet(VAR_DEOXYS_INTERACTION_STEP_COUNTER);
        VarSet(VAR_DEOXYS_INTERACTION_STEP_COUNTER, 0);
        if (r5 != 0 && sDeoxysStepCaps[r5 - 1] < r6)
        {
            MoveDeoxysObject(0);
            VarSet(VAR_DEOXYS_INTERACTION_NUM, 0);
            gSpecialVar_Result = 0;
            DestroyTask(taskId);
        }
        else if (r5 == 10)
        {
            FlagSet(FLAG_SYS_DEOXYS_AWAKENED);
            gSpecialVar_Result = 2;
            ScriptContext_Enable();
            DestroyTask(taskId);
        }
        else
        {
            r5++;
            MoveDeoxysObject(r5);
            VarSet(VAR_DEOXYS_INTERACTION_NUM, r5);
            gSpecialVar_Result = 1;
            DestroyTask(taskId);
        }
    }
}

static void MoveDeoxysObject(u8 num)
{
    u8 mapObjId;
    LoadPalette(sDeoxysObjectPals[num], OBJ_PLTT_ID(10), PLTT_SIZEOF(4));
    UpdateSpritePaletteWithWeather(10, FALSE);
    ApplyGlobalFieldPaletteTint(10);
    TryGetObjectEventIdByLocalIdAndMap(1, gSaveBlock1Ptr->location.mapNum, gSaveBlock1Ptr->location.mapGroup, &mapObjId);
    if (num == 0)
        PlaySE(SE_M_CONFUSE_RAY);
    else
        PlaySE(SE_DEOXYS_MOVE);
    CreateTask(Task_WaitDeoxysFieldEffect, 8);
    gFieldEffectArguments[0] = 1;
    gFieldEffectArguments[1] = 56;
    gFieldEffectArguments[2] = 2;
    gFieldEffectArguments[3] = sDeoxysCoords[num][0];
    gFieldEffectArguments[4] = sDeoxysCoords[num][1];
    if (num == 0)
        gFieldEffectArguments[5] = 60;
    else
        gFieldEffectArguments[5] = 5;
    FieldEffectStart(FLDEFF_MOVE_DEOXYS_ROCK);
    SetObjEventTemplateCoords(1, sDeoxysCoords[num][0], sDeoxysCoords[num][1]);
}

static void Task_WaitDeoxysFieldEffect(u8 taskId)
{
    if (!FieldEffectActiveListContains(FLDEFF_MOVE_DEOXYS_ROCK))
    {
        ScriptContext_Enable();
        DestroyTask(taskId);
    }
}

void IncrementBirthIslandRockStepCount(void)
{
    u16 count = VarGet(VAR_DEOXYS_INTERACTION_STEP_COUNTER);
    if (gSaveBlock1Ptr->location.mapGroup == MAP_GROUP(BIRTH_ISLAND_EXTERIOR) && gSaveBlock1Ptr->location.mapNum == MAP_NUM(BIRTH_ISLAND_EXTERIOR))
    {
        count++;
        if (count > 99)
            VarSet(VAR_DEOXYS_INTERACTION_STEP_COUNTER, 0);
        else
            VarSet(VAR_DEOXYS_INTERACTION_STEP_COUNTER, count);
    }
}

void SetDeoxysTrianglePalette(void)
{
    u8 num = VarGet(VAR_DEOXYS_INTERACTION_NUM);
    LoadPalette(sDeoxysObjectPals[num], OBJ_PLTT_ID(10), PLTT_SIZEOF(4));
    ApplyGlobalFieldPaletteTint(10);
}

bool8 IsBadEggInParty(void)
{
    u8 partyCount = CalculatePlayerPartyCount();
    u8 i;
    for (i = 0; i < partyCount; i++)
    {
        if (GetMonData(&gPlayerParty[i], MON_DATA_SANITY_IS_BAD_EGG) == TRUE)
            return TRUE;
    }
    return FALSE;
}

bool8 IsPlayerNotInTrainerTowerLobby(void)
{
    if (gSaveBlock1Ptr->location.mapGroup == MAP_GROUP(TRAINER_TOWER_LOBBY) && gSaveBlock1Ptr->location.mapNum == MAP_NUM(TRAINER_TOWER_LOBBY))
        return FALSE;
    else
        return TRUE;
}

void BrailleCursorToggle(void)
{
    // 8004 = x - 27
    // 8005 = y
    // 8006 = action (0 = create, 1 = delete)
    u16 x;
    if (gQuestLogState != QL_STATE_PLAYBACK)
    {
        x = gSpecialVar_0x8004 + 27;
        if (gSpecialVar_0x8006 == 0)
            sBrailleTextCursorSpriteID = CreateTextCursorSprite(0, x, gSpecialVar_0x8005, 0, 0);
        else
            DestroyTextCursorSprite(sBrailleTextCursorSpriteID);
    }
}

bool8 PlayerPartyContainsSpeciesWithPlayerID(void)
{
    // 8004 = species
    u8 playerCount = CalculatePlayerPartyCount();
    u8 i;
    for (i = 0; i < playerCount; i++)
    {
        if (GetMonData(&gPlayerParty[i], MON_DATA_SPECIES_OR_EGG, NULL) == gSpecialVar_0x8004 
            && GetPlayerTrainerId() == GetMonData(&gPlayerParty[i], MON_DATA_OT_ID, NULL))
            return TRUE;
    }
    return FALSE;
}

/*
 * Determines which of Lorelei's doll collection to show
 * based on how many times you've entered the Hall of Fame.
 */
void UpdateLoreleiDollCollection(void)
{
    u32 numHofClears = GetGameStat(GAME_STAT_ENTERED_HOF);
    if (numHofClears >= 25)
    {
        FlagClear(FLAG_HIDE_LORELEI_HOUSE_MEOWTH_DOLL);
        if (numHofClears >= 50)
            FlagClear(FLAG_HIDE_LORELEI_HOUSE_CHANSEY_DOLL);
        if (numHofClears >= 75)
            FlagClear(FLAG_HIDE_LORELEIS_HOUSE_NIDORAN_F_DOLL);
        if (numHofClears >= 100)
            FlagClear(FLAG_HIDE_LORELEI_HOUSE_JIGGLYPUFF_DOLL);
        if (numHofClears >= 125)
            FlagClear(FLAG_HIDE_LORELEIS_HOUSE_NIDORAN_M_DOLL);
        if (numHofClears >= 150)
            FlagClear(FLAG_HIDE_LORELEIS_HOUSE_FEAROW_DOLL);
        if (numHofClears >= 175)
            FlagClear(FLAG_HIDE_LORELEIS_HOUSE_PIDGEOT_DOLL);
        if (numHofClears >= 200)
            FlagClear(FLAG_HIDE_LORELEIS_HOUSE_LAPRAS_DOLL);
    }
}

void LoopWingFlapSound(void)
{
    // 8004 = Num flaps
    // 8005 = Frame delay between flaps
    CreateTask(Task_WingFlapSound, 8);
    PlaySE(SE_M_WING_ATTACK);
}

static void Task_WingFlapSound(u8 taskId)
{
    s16 *data = gTasks[taskId].data;
    data[1]++;
    if (data[1] == gSpecialVar_0x8005)
    {
        data[0]++;
        data[1] = 0;
        PlaySE(SE_M_WING_ATTACK);
    }
    if (data[0] == gSpecialVar_0x8004 - 1)
        DestroyTask(taskId);
}

bool8 InPokemonCenter(void)
{
    static const u16 sPokemonCenters[] =
    {
        MAP_VIRIDIAN_CITY_POKEMON_CENTER_1F,
        MAP_PEWTER_CITY_POKEMON_CENTER_1F,
        MAP_ROUTE4_POKEMON_CENTER_1F,
        MAP_CERULEAN_CITY_POKEMON_CENTER_1F,
        MAP_VERMILION_CITY_POKEMON_CENTER_1F,
        MAP_ROUTE10_POKEMON_CENTER_1F,
        MAP_LAVENDER_TOWN_POKEMON_CENTER_1F,
        MAP_CELADON_CITY_POKEMON_CENTER_1F,
        MAP_FUCHSIA_CITY_POKEMON_CENTER_1F,
        MAP_SAFFRON_CITY_POKEMON_CENTER_1F,
        MAP_CINNABAR_ISLAND_POKEMON_CENTER_1F,
        MAP_INDIGO_PLATEAU_POKEMON_CENTER_1F,
        MAP_ONE_ISLAND_POKEMON_CENTER_1F,
        MAP_TWO_ISLAND_POKEMON_CENTER_1F,
        MAP_THREE_ISLAND_POKEMON_CENTER_1F,
        MAP_FOUR_ISLAND_POKEMON_CENTER_1F,
        MAP_FIVE_ISLAND_POKEMON_CENTER_1F,
        MAP_SIX_ISLAND_POKEMON_CENTER_1F,
        MAP_SEVEN_ISLAND_POKEMON_CENTER_1F,
        MAP_BATTLE_COLOSSEUM_2P,
        MAP_TRADE_CENTER,
        MAP_RECORD_CORNER,
        MAP_BATTLE_COLOSSEUM_4P,
        MAP_UNDEFINED
    };

    int i;
    u16 map = (gSaveBlock1Ptr->location.mapGroup << 8) + gSaveBlock1Ptr->location.mapNum;

    for (i = 0; sPokemonCenters[i] != MAP_UNDEFINED; i++)
    {
        if (sPokemonCenters[i] == map)
            return TRUE;
    }
    return FALSE;
}

static void PreparePartyForSkyBattle(void)
{
    int i, participatingPokemonSlot = 0;
    u8 partyCount = CalculatePlayerPartyCount();

    FlagSet(B_FLAG_SKY_BATTLE);
    SavePlayerParty();

    for (i = 0; i < partyCount; i++)
    {
        struct Pokemon* pokemon = &gPlayerParty[i];

        if (CanMonParticipateInSkyBattle(pokemon))
            participatingPokemonSlot += 1 << i;
        else
            ZeroMonData(pokemon);
    }
    VarSet(B_VAR_SKY_BATTLE,participatingPokemonSlot);
    CompactPartySlots();
}

void TrySkyBattle(void)
{
    int i;

    if (B_VAR_SKY_BATTLE == 0 || B_FLAG_SKY_BATTLE == 0)
    {
        LockPlayerFieldControls();
        ScriptContext_SetupScript(Debug_FlagsAndVarNotSetBattleConfigMessage);
        return;
    }
    for (i = 0; i < CalculatePlayerPartyCount(); i++)
    {
        struct Pokemon* pokemon = &gPlayerParty[i];
        if (CanMonParticipateInSkyBattle(pokemon) && GetMonData(pokemon, MON_DATA_HP, NULL) > 0)
        {
            PreparePartyForSkyBattle();
            gSpecialVar_Result = TRUE;
            return;
        }
    }
    gSpecialVar_Result = FALSE;
}
