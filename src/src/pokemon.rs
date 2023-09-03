use std::fs;
//An... Exhastive list of every single pokemon
use Type::*;
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;
use LegendStatus::*;
use csv::{self, Reader};
use try_catch::catch;

#[derive(Copy, Clone)]
#[derive(TryFromPrimitive)]
#[derive(PartialEq)]
#[repr(i32)]
pub enum Pokemon{
Bulbasaur,
Ivysaur,
Venusaur,
Venusaur_Mega,
Charmander,
Charmeleon,
Charizard,
Charizard_Mega_X,
Charizard_Mega_Y,
Squirtle,
Wartortle,
Blastoise,
Blastoise_Mega,
Caterpie,
Metapod,
Butterfree,
Weedle,
Kakuna,
Beedrill,
Beedrill_Mega,
Pidgey,
Pidgeotto,
Pidgeot,
Pidgeot_Mega,
Rattata,
Rattata_Alolan,
Raticate,
Raticate_Alolan,
Spearow,
Fearow,
Ekans,
Arbok,
Pikachu,
Raichu,
Raichu_Alolan,
Sandshrew,
Sandshrew_Alolan,
Sandslash,
Sandslash_Alolan,
Nidoran_F,
Nidorina,
Nidoqueen,
Nidoran_M,
Nidorino,
Nidoking,
Clefairy,
Clefable,
Vulpix,
Vulpix_Alolan,
Ninetales,
Ninetales_Alolan,
Jigglypuff,
Wigglytuff,
Zubat,
Golbat,
Oddish,
Gloom,
Vileplume,
Paras,
Parasect,
Venonat,
Venomoth,
Diglett,
Diglett_Alolan,
Dugtrio,
Dugtrio_Alolan,
Meowth,
Meowth_Alolan,
Meowth_Galarian,
Persian,
Persian_Alolan,
Psyduck,
Golduck,
Mankey,
Primeape,
Growlithe,
Growlithe_Hisuian,
Arcanine,
Arcanine_Hisuian,
Poliwag,
Poliwhirl,
Poliwrath,
Abra,
Kadabra,
Alakazam,
Alakazam_Mega,
Machop,
Machoke,
Machamp,
Bellsprout,
Weepinbell,
Victreebel,
Tentacool,
Tentacruel,
Geodude,
Geodude_Alolan,
Graveler,
Graveler_Alolan,
Golem,
Golem_Alolan,
Ponyta,
Ponyta_Galarian,
Rapidash,
Rapidash_Galarian,
Slowpoke,
Slowpoke_Galarian,
Slowbro,
Slowbro_Galarian,
Slowbro_Mega,
Magnemite,
Magneton,
Farfetchd,
Farfetchd_Galarian,
Doduo,
Dodrio,
Seel,
Dewgong,
Grimer,
Grimer_Alolan,
Muk,
Muk_Alolan,
Shellder,
Cloyster,
Gastly,
Haunter,
Gengar,
Gengar_Mega,
Onix,
Drowzee,
Hypno,
Krabby,
Kingler,
Voltorb,
Voltorb_Hisuian,
Electrode,
Electrode_Hisuian,
Exeggcute,
Exeggutor,
Exeggutor_Alolan,
Cubone,
Marowak,
Marowak_Alolan,
Hitmonlee,
Hitmonchan,
Lickitung,
Koffing,
Weezing,
Weezing_Galarian,
Rhyhorn,
Rhydon,
Chansey,
Tangela,
Kangaskhan,
Kangaskhan_Mega,
Horsea,
Seadra,
Goldeen,
Seaking,
Staryu,
Starmie,
Mr_Mime,
Mr_Mime_Galarian,
Scyther,
Jynx,
Electabuzz,
Magmar,
Pinsir,
Pinsir_Mega,
Tauros,
Magikarp,
Gyarados,
Gyarados_Mega,
Lapras,
Ditto,
Eevee,
Vaporeon,
Jolteon,
Flareon,
Porygon,
Omanyte,
Omastar,
Kabuto,
Kabutops,
Aerodactyl,
Aerodactyl_Mega,
Snorlax,
Articuno,
Articuno_Galarian,
Zapdos,
Zapdos_Galarian,
Moltres,
Moltres_Galarian,
Dratini,
Dragonair,
Dragonite,
MewTwo,
MewTwo_Mega_X,
MewTwo_Mega_Y,
Mew,
Chikorita,
Bayleef,
Meganium,
Cyndaquil,
Quilava,
Typhlosion,
Typhlosion_Hisuian,
Totodile,
Croconaw,
Feraligatr,
Sentret,
Furret,
Hoothoot,
Noctowl,
Ledyba,
Ledian,
Spinarak,
Ariados,
Crobat,
Chinchou,
Lanturn,
Pichu,
Cleffa,
Igglybuff,
Togepi,
Togetic,
Natu,
Xatu,
Mareep,
Flaaffy,
Ampharos,
Ampharos_Mega,
Bellossom,
Marill,
Azumarill,
Sudowoodo,
Politoed,
Hoppip,
Skiploom,
Jumpluff,
Aipom,
Sunkern,
Sunflora,
Yanma,
Wooper,
Quagsire,
Espeon,
Umbreon,
Murkrow,
Slowking,
Slowking_Galarian,
Misdreavus,
Unown,
Wobbuffet,
Girafarig,
Pineco,
Forretress,
Dunsparce,
Gligar,
Steelix,
Steelix_Mega,
Snubbull,
Granbull,
Qwilfish,
Quilfish_Hisuian,
Scizor,
Scizor_Mega,
Shuckle,
Heracross,
Heracross_Mega,
Sneasel,
Sneasel_Hisuian,
Teddiursa,
Ursaring,
Slugma,
Magcargo,
Swinub,
Piloswine,
Corsola,
Corsola_Galarian,
Remoraid,
Octillery,
Delibird,
Mantine,
Skarmory,
Houndour,
Houndoom,
Houndoom_Mega,
Kingdra,
Phanpy,
Donphan,
Porygon2,
Stantler,
Smeargle,
Tyrogue,
Hitmontop,
Smoochum,
Elekid,
Magby,
Miltank,
Blissey,
Raikou,
Entei,
Suicune,
Larvitar,
Pupitar,
Tyranitar,
Tyranitar_Mega,
Lugia,
Ho_oh,
Celebi,
Treecko,
Grovyle,
Sceptile,
Sceptile_Mega,
Torchic,
Combusken,
Blaziken,
Blaziken_Mega,
Mudkip,
Marshtomp,
Swampert,
Swampert_Mega,
Poochyena,
Mightyena,
Zigzagoon,
Zigzagoon_Galarian,
Linoone,
Linoone_Galarian,
Wurmple,
Silcoon,
Beautifly,
Cascoon,
Dustox,
Lotad,
Lombre,
Ludicolo,
Seedot,
Nuzleaf,
Shiftry,
Taillow,
Swellow,
Wingull,
Pelipper,
Ralts,
Kirlia,
Gardevoir,
Gardevoir_Mega,
Surskit,
Masquerain,
Shroomish,
Breloom,
Slakoth,
Vigoroth,
Slaking,
Nincada,
Ninjask,
Shedinja,
Whismur,
Loudred,
Exploud,
Makuhita,
Hariyama,
Azurill,
Nosepass,
Skitty,
Delcatty,
Sableye,
Sableye_Mega,
Mawile,
Mawile_Mega,
Aron,
Lairon,
Aggron,
Aggron_Mega,
Meditite,
Medicham,
Medicham_Mega,
Electrike,
Manectric,
Manectric_Mega,
Plusle,
Minun,
Volbeat,
Illumise,
Roselia,
Gulpin,
Swalot,
Carvanha,
Sharpedo,
Sharpedo_Mega,
Wailmer,
Wailord,
Numel,
Camerupt,
Camerupt_Mega,
Torkoal,
Spoink,
Grumpig,
Spinda,
Trapinch,
Vibrava,
Flygon,
Cacnea,
Cacturne,
Swablu,
Altaria,
Altaria_Mega,
Zangoose,
Seviper,
Lunatone,
Solrock,
Barboach,
Whiscash,
Corphish,
Crawdaunt,
Baltoy,
Claydol,
Lileep,
Cradily,
Anorith,
Armaldo,
Feebas,
Milotic,
Castform,
Kecleon,
Shuppet,
Banette,
Banette_Mega,
Duskull,
Dusclops,
Tropius,
Chimecho,
Absol,
Absol_Mega,
Wynaut,
Snorunt,
Glalie,
Glalie_Mega,
Spheal,
Sealeo,
Walrein,
Clamperl,
Huntail,
Gorebyss,
Relicanth,
Luvdisc,
Bagon,
Shelgon,
Salamence,
Salamence_Mega,
Beldum,
Metang,
Metagross,
Metagross_Mega,
Regirock,
Regice,
Registeel,
Latias,
Latias_Mega,
Latios,
Latios_Mega,
Kyogre,
Kyogre_Primal,
Groudon,
Groudon_Primal,
Rayquaza,
Rayquaza_Mega,
Jirachi,
Deoxys,
Turtwig,
Grotle,
Torterra,
Chimchar,
Monferno,
Infernape,
Piplup,
Prinplup,
Empoleon,
Starly,
Staravia,
Staraptor,
Bidoof,
Bibarel,
Kricketot,
Kricketune,
Shinx,
Luxio,
Luxray,
Budew,
Roserade,
Cranidos,
Rampardos,
Shieldon,
Bastiodon,
Burmy,
Wormadam,
Mothim,
Combee,
Vespiquen,
Pachirisu,
Buizel,
Floatzel,
Cherubi,
Cherrim,
Shellos,
Gastrodon,
Ambipom,
Drifloon,
Drifblim,
Buneary,
Lopunny,
Mismagius,
Honchkrow,
Glameow,
Purugly,
Chingling,
Stunky,
Skuntank,
Bronzor,
Bronzong,
Bonsly,
Mime_Jr,
Happiny,
Chatot,
Spiritomb,
Gible,
Gabite,
Garchomp,
Garchomp_Mega,
Munchlax,
Riolu,
Lucario,
Lucario_Mega,
Hippopotas,
Hippowdon,
Skorupi,
Drapion,
Croagunk,
Toxicroak,
Carnivine,
Finneon,
Lumineon,
Mantyke,
Snover,
Abomasnow,
Abomasnow_Mega,
Weavile,
Magnezone,
Lickilicky,
Rhyperior,
Tangrowth,
Electivire,
Magmortar,
Togekiss,
Yanmega,
Leafeon,
Glaceon,
Gliscor,
Mamoswine,
Porygon_Z,
Gallade,
Gallade_Mega,
Probopass,
Dusknoir,
Froslass,
Rotom,
Uxie,
Mesprit,
Azelf,
Dialga,
Dialga_Origin,
Palkia,
Palkia_Origin,
Heatran,
Regigigas,
Giratina,
Giratina_Origin,
Cresselia,
Phione,
Manaphy,
Darkrai,
Shaymin,
Shaymin_Sky,
Arceus,
Victini,
Snivy,
Servine,
Serperior,
Tepig,
Pignite,
Emboar,
Oshawott,
Dewott,
Samurott,
Samurott_Hisuian,
Patrat,
Watchog,
Lillipup,
Herdier,
Stoutland,
Purrloin,
Liepard,
Pansage,
Simisage,
Pansear,
Simisear,
Panpour,
Simipour,
Munna,
Musharna,
Pidove,
Tranquill,
Unfezant,
Blitzle,
Zebstrika,
Roggenrola,
Boldore,
Gigalith,
Woobat,
Swoobat,
Drilbur,
Excadrill,
Audino,
Audino_Mega,
Timburr,
Gurdurr,
Conkeldurr,
Tympole,
Palpitoad,
Seismitoad,
Throh,
Sawk,
Sewaddle,
Swadloon,
Leavanny,
Venipede,
Whirlipede,
Scolipede,
Cottonee,
Whimsicott,
Petilil,
Lilligant,
Lilligant_Hisuian,
Basculin,
Sandile,
Krokorok,
Krookodile,
Darumaka,
Darumaka_Galarian,
Darmanitan,
Darmanitan_Galarian,
Maractus,
Dwebble,
Crustle,
Scraggy,
Scrafty,
Sigilyph,
Yamask,
Yamask_Galarian,
Cofagrigus,
Tirtouga,
Carracosta,
Archen,
Archeops,
Trubbish,
Garbodor,
Zorua,
Zorua_Hisuian,
Zoroark,
Zoroark_Hisuian,
Minccino,
Cinccino,
Gothita,
Gothorita,
Gothitelle,
Solosis,
Duosion,
Reuniclus,
Ducklett,
Swanna,
Vanillite,
Vanillish,
Vanilluxe,
Deerling,
Sawsbuck,
Emolga,
Karrablast,
Escavalier,
Foongus,
Amoonguss,
Frillish,
Jellicent,
Alomomola,
Joltik,
Galvantula,
Ferroseed,
Ferrothorn,
Klink,
Klang,
Klinklang,
Tynamo,
Eelektrik,
Eelektross,
Elgyem,
Beheeyem,
Litwick,
Lampent,
Chandelure,
Axew,
Fraxure,
Haxorus,
Cubchoo,
Beartic,
Cryogonal,
Shelmet,
Accelgor,
Stunfisk,
Stunfisk_Galarian,
Mienfoo,
Mienshao,
Druddigon,
Golett,
Golurk,
Pawniard,
Bisharp,
Bouffalant,
Rufflet,
Braviary,
Braviary_Hisuian,
Vullaby,
Mandibuzz,
Heatmor,
Durant,
Deino,
Zweilous,
Hydreigon,
Larvesta,
Volcarona,
Cobalion,
Terrakion,
Virizion,
Tornadus,
Tornadus_Therian,
Thundurus,
Thundurus_Therian,
Reshiram,
Zekrom,
Landorus,
Landorus_Therian,
Kyurem,
Keldeo,
Keldeo_Resolute,
Meloetta,
Genesect,
Chespin,
Quilladin,
Chesnaught,
Fennekin,
Braixen,
Delphox,
Froakie,
Frogadier,
Greninja,
Bunnelby,
Diggersby,
Fletchling,
Fletchinder,
Talonflame,
Scatterbug,
Spewpa,
Vivillon,
Litleo,
Pyroar,
Flabebe,
Floette,
Florges,
Skiddo,
Gogoat,
Pancham,
Pangoro,
Furfrou,
Espurr,
Meowstic,
Meowstic_Female,
Honedge,
Doublade,
Aegislash,
Spritzee,
Aromatisse,
Swirlix,
Slurpuff,
Inkay,
Malamar,
Binacle,
Barbaracle,
Skrelp,
Dragalge,
Clauncher,
Clawitzer,
Helioptile,
Heliolisk,
Tyrunt,
Tyrantrum,
Amaura,
Aurorus,
Sylveon,
Hawlucha,
Dedenne,
Carbink,
Goomy,
Sliggoo,
Sliggoo_Hisuian,
Goodra,
Goodra_Hisuian,
Klefki,
Phantump,
Trevenant,
Pumpkaboo,
Gourgeist,
Bergmite,
Avalugg,
Avalugg_Hisuian,
Noibat,
Noivern,
Xerneas,
Yveltal,
Zygarde,
Diancie,
Diancie_Mega,
Hoopa,
Hoopa_Unbound,
Volcanion,
Rowlet,
Dartrix,
Decidueye,
Decidueye_Hisuian,
Litten,
Torracat,
Incineroar,
Popplio,
Brionne,
Primarina,
Pikipek,
Trumbeak,
Toucannon,
Yungoos,
Gumshoos,
Grubbin,
Charjabug,
Vikavolt,
Crabrawler,
Crabominable,
Oricorio,
Cutiefly,
Ribombee,
Rockruff,
Lycanroc,
Lycanroc_Midnight,
Lycanroc_Dusk,
Wishiwashi,
Mareanie,
Toxapex,
Mudbray,
Mudsdale,
Dewpider,
Araquanid,
Fomantis,
Lurantis,
Morelull,
Shiinotic,
Salandit,
Salazzle,
Stufful,
Bewear,
Bounsweet,
Steenee,
Tsareena,
Comfey,
Oranguru,
Passimian,
Wimpod,
Golisopod,
Sandygast,
Palossand,
Pyukumuku,
Type_Null,
Silvally,
Minior,
Komala,
Turtonator,
Togedemaru,
Mimikyu,
Bruxish,
Drampa,
Dhelmise,
Jangmo_o,
Hakamo_o,
Kommo_o,
Tapu_Koko,
Tapu_Lele,
Tapu_Bulu,
Tapu_Fini,
Cosmog,
Cosmoen,
Solgaleo,
Lunala,
Nihilego,
Buzzwole,
Pheromosa,
Xurkitree,
Celesteela,
Kartana,
Guzzlord,
Necrozma,
Necrozma_Dusk_Mane,
Necrozma_Dawn_Wings,
Necrozma_Ultra,
Magearna,
Marshadow,
Poipole,
Naganadel,
Stakataka,
Blacephalon,
Zeraora,
Meltan,
Melmetal,
Grookey,
Thwackey,
Rillaboom,
Scorbunny,
Raboot,
Cinderace,
Sobble,
Drizzile,
Inteleon,
Skwovet,
Greedent,
Rookidee,
Corvisquire,
Corviknight,
Blipbug,
Dottler,
Orbeetle,
Nickit,
Thievul,
Gossifleur,
Eldegoss,
Wooloo,
Dubwool,
Chewtle,
Drednaw,
Tamper,
Boutund,
Rolycoly,
Charkol,
Colossal,
Applin,
Flapple,
Appletun,
Silicobra,
Sandaconda,
Cramorant,
Arrokuda,
Barraskewda,
Toxel,
Toxtricity,
Sizzlipede,
Centiskorch,
Clobbopus,
Grapploct,
Sinistea,
Polteageist,
Hatenna,
Hattrem,
Hatterene,
Impidimp,
Morgrem,
Grimmsnarl,
Obstagoon,
Perrserker,
Cursola,
Sirfetchd,
Mr_Rime,
Runerigus,
Milcery,
Alcremie,
Falinks,
Pincurchin,
Snom,
Frosmoth,
Stonjourner,
Eiscue,
Indeedee,
Morpeko,
Cufant,
Copperajah,
Dracozolt,
Arctozolt,
Dracovish,
Arctovish,
Duraludon,
Dreepy,
Drakloak,
Dragapult,
Zacian,
Zamazenta,
Eternatus,
Eternatus_Eternamax,
Kubfu,
Urshifu,
Zarude,
Regieleki,
Regidrago,
Glastrier,
Spectrier,
Calyrex,
Calyrex_Ice_Rider,
Calyrex_Shadow_Rider,
Wyrdeer,
Kleaver,
Ursaluna,
Basculegion,
Sneasler,
Overquil,
Enamorus,
Enamorus_Therian
}

#[derive(Clone)]
pub struct PokemonStats{
    pub pokemon_id: Pokemon,
    pub pokemon_name: String,
    pub type1: Type,
    pub type2: Type,
    pub generation: i16,
    pub status: LegendStatus,
    pub min_level: i16,
}

#[derive(PartialEq)]
#[derive(Copy, Clone)]
pub enum Type{
    Normal,
    Fighting,
    Flying,
    Poison,
    Ground,
    Rock,
    Bug,
    Ghost,
    Steel,
    Fire,
    Water,
    Grass,
    Electric,
    Psychic,
    Ice,
    Dragon,
    Dark,
    Fairy,
    None
}
#[derive(PartialEq)]
#[derive(Copy, Clone)]
pub enum LegendStatus{
    Standard,
    Legendary,
    Mythical,
    Mega,
}

const LEGEND_OFFSET : i32 = 50; //Level above "possible" level where they can start appearing
const MEGA_OFFSET : i32 = 15; //Level above final stage evo where it is possible to find Mega forms in wild
const MYTHICAL_OFFSET: i32 = 55;

pub fn format_pokemon_name(pkmn_name: String) -> String{
    return format!("SPECIES_{}",pkmn_name.to_uppercase());
}
pub fn get_pokemon_data(pokemon: Pokemon, all_stats: &Vec<PokemonStats>) -> PokemonStats{
    all_stats[pokemon as usize].clone()
}
pub fn get_pokemon_data_integer(pokemon: i32,all_stats: &Vec<PokemonStats>) -> PokemonStats{
    all_stats[pokemon as usize].clone()
}
pub fn get_pokemon_from_name(pokemon_name: String,all_stats: &Vec<PokemonStats>) -> Pokemon{
    for i in 0..all_stats.len() {
        if all_stats[i].pokemon_name == pokemon_name{
            return all_stats[i].pokemon_id;
        }
        if format_pokemon_name(all_stats[i].pokemon_name.clone()) == pokemon_name{
            return  all_stats[i].pokemon_id;
        }
    }
    return Pokemon::Bulbasaur;
}
pub fn read_all_pokemon() -> Vec<PokemonStats>{
    let csv: String = fs::read_to_string("data/emerald/pokemon.csv").unwrap();
    let mut reader = csv::Reader::from_reader(csv.as_bytes());
    let mut all_stats: Vec<PokemonStats> = Vec::new();
    let mut cur_num = 0;
    for cur_pokemon in reader.records(){
        if cur_num == Pokemon::Volcanion as i32{
            println!("End of file");
            break;
        }
        let cur_pokemon = cur_pokemon.unwrap();
        let min_level_string = cur_pokemon[5].to_string();
        let min_level = if min_level_string == "Other".to_string() || min_level_string == "Mega"{
            0
        }
        else{
            min_level_string.parse::<i16>().unwrap()
        };
        // println!("{}",cur_pokemon[0].to_string());
        let add_pokemon = PokemonStats{
            pokemon_id: Pokemon::try_from(cur_num).unwrap(),
            pokemon_name: cur_pokemon[0].to_string(),
            type1: string_to_type(cur_pokemon[1].to_string()),
            type2: string_to_type(cur_pokemon[2].to_string()),
            generation: cur_pokemon[3].to_string().parse::<i16>().unwrap(),
            status: string_to_legend_status(cur_pokemon[4].to_string()),
            min_level: min_level
        };
        all_stats.push(add_pokemon);
        cur_num += 1;
    }
    all_stats
}

fn string_to_type(pkmn_type: String) -> Type{
    match pkmn_type.as_str(){
        "Normal" => Type::Grass,
        "Fighting" => Type::Fighting,
        "Flying" => Type::Flying,
        "Poison" => Type::Poison,
        "Ground" => Type::Ground,
        "Rock" => Type::Rock,
        "Bug" => Type::Bug,
        "Ghost" => Type::Ghost,
        "Steel" => Type::Steel,
        "Fire" => Type::Fire,
        "Water" => Type::Water,
        "Grass" => Type::Grass,
        "Electric" => Type::Electric,
        "Psychic" => Type::Psychic,
        "Ice" => Type::Ice,
        "Dragon" => Type::Dragon,
        "Dark" => Type::Dark,
        "Fairy" => Type::Fairy,
        _ => Type::None
    }
}
fn string_to_legend_status(pkmn_stats: String) -> LegendStatus{
    match pkmn_stats.as_str(){
        "TRUE" => LegendStatus::Legendary,
        "FALSE" => LegendStatus::Standard,
        "MEGA" => LegendStatus::Mega,
        _ => LegendStatus::Standard
    }
}

