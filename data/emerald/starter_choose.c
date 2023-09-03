#include "global.h"
#include "bg.h"
#include "data.h"
#include "decompress.h"
#include "event_data.h"
#include "gpu_regs.h"
#include "international_string_util.h"
#include "main.h"
#include "menu.h"
#include "palette.h"
#include "pokedex.h"
#include "pokemon.h"
#include "scanline_effect.h"
#include "sound.h"
#include "sprite.h"
#include "starter_choose.h"
#include "strings.h"
#include "task.h"
#include "text.h"
#include "text_window.h"
#include "trainer_pokemon_sprites.h"
#include "trig.h"
#include "window.h"
#include "constants/songs.h"
#include "constants/rgb.h"

#define STARTER_MON_COUNT   3

// Position of the sprite of the selected starter Pokemon
#define STARTER_PKMN_POS_X (DISPLAY_WIDTH / 2)
#define STARTER_PKMN_POS_Y 64

#define TAG_POKEBALL_SELECT 0x1000
#define TAG_STARTER_CIRCLE  0x1001

static void CB2_StarterChoose(void);
static void ClearStarterLabel(void);
static void Task_StarterChoose(u8 taskId);
static void Task_HandleStarterChooseInput(u8 taskId);
static void Task_WaitForStarterSprite(u8 taskId);
static void Task_AskConfirmStarter(u8 taskId);
static void Task_HandleConfirmStarterInput(u8 taskId);
static void Task_DeclineStarter(u8 taskId);
static void Task_MoveStarterChooseCursor(u8 taskId);
static void Task_CreateStarterLabel(u8 taskId);
static void CreateStarterPokemonLabel(u8 selection);
static u8 CreatePokemonFrontSprite(u16 species, u8 x, u8 y);
static void SpriteCB_SelectionHand(struct Sprite *sprite);
static void SpriteCB_Pokeball(struct Sprite *sprite);
static void SpriteCB_StarterPokemon(struct Sprite *sprite);

static u16 sStarterLabelWindowId;

const u16 gBirchBagGrass_Pal[] = INCBIN_U16("graphics/starter_choose/tiles.gbapal");
static const u16 sPokeballSelection_Pal[] = INCBIN_U16("graphics/starter_choose/pokeball_selection.gbapal");
static const u16 sStarterCircle_Pal[] = INCBIN_U16("graphics/starter_choose/starter_circle.gbapal");
const u32 gBirchBagTilemap[] = INCBIN_U32("graphics/starter_choose/birch_bag.bin.lz");
const u32 gBirchGrassTilemap[] = INCBIN_U32("graphics/starter_choose/birch_grass.bin.lz");
const u32 gBirchBagGrass_Gfx[] = INCBIN_U32("graphics/starter_choose/tiles.4bpp.lz");
const u32 gPokeballSelection_Gfx[] = INCBIN_U32("graphics/starter_choose/pokeball_selection.4bpp.lz");
static const u32 sStarterCircle_Gfx[] = INCBIN_U32("graphics/starter_choose/starter_circle.4bpp.lz");

static const struct WindowTemplate sWindowTemplates[] =
{
    {
        .bg = 0,
        .tilemapLeft = 3,
        .tilemapTop = 15,
        .width = 24,
        .height = 4,
        .paletteNum = 14,
        .baseBlock = 0x0200
    },
    DUMMY_WIN_TEMPLATE,
};

static const struct WindowTemplate sWindowTemplate_ConfirmStarter =
{
    .bg = 0,
    .tilemapLeft = 24,
    .tilemapTop = 9,
    .width = 5,
    .height = 4,
    .paletteNum = 14,
    .baseBlock = 0x0260
};

static const struct WindowTemplate sWindowTemplate_StarterLabel =
{
    .bg = 0,
    .tilemapLeft = 0,
    .tilemapTop = 0,
    .width = 13,
    .height = 4,
    .paletteNum = 14,
    .baseBlock = 0x0274
};

static const u8 sPokeballCoords[STARTER_MON_COUNT][2] =
{
    {60, 64},
    {120, 88},
    {180, 64},
};

static const u8 sStarterLabelCoords[STARTER_MON_COUNT][2] =
{
    {0, 9},
    {16, 10},
    {8, 4},
};

static const u16 sStarterMon[STARTER_MON_COUNT] =
{