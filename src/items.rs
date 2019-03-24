#![allow(unused_must_use)]
#![allow(dead_code)]


use std::convert::TryFrom;
use regex::Regex;

#[derive(Debug)]
pub enum ItemParseError {
    UnknownItem(String),
    UnknownSpecial(String),
    UnknownAttribute(String),
    UnknownTech(String),
    ParseIntError(std::num::ParseIntError),
    HexError(hex::FromHexError),
}

impl From<std::num::ParseIntError> for ItemParseError {
    fn from(err: std::num::ParseIntError) -> ItemParseError {
        ItemParseError::ParseIntError(err)
    }
}

impl From<hex::FromHexError> for ItemParseError {
    fn from(err: hex::FromHexError) -> ItemParseError {
        ItemParseError::HexError(err)
    }
}

pub trait ItemData: std::fmt::Debug {
    fn row1(&self) -> u32;
    fn row2(&self) -> u32;
    fn row3(&self) -> u32;
    fn row4(&self) -> u32;
}

#[derive(Debug)]
pub enum Attribute {
    Native,
    ABeast,
    Machine,
    Dark,
    Hit
}

#[derive(Debug)]
pub struct WeaponAttribute {
    attr: Attribute,
    value: u8,
}

impl TryFrom<&str> for WeaponAttribute {
    type Error = ItemParseError;
    fn try_from(attr: &str) -> Result<WeaponAttribute, ItemParseError> {
        let re = Regex::new(r"(?P<value>\d{1,3})(?P<attr>[namdh])").unwrap();
        if let Some(cap) = re.captures(attr) {
            let strvalue = &cap["value"];
            let value = if let Ok(v) = strvalue.parse() {
                v
            }
            else {
                0
            };
                
            let attr = match &cap["attr"] {
                "n" => Attribute::Native,
                "a" => Attribute::ABeast,
                "m" => Attribute::Machine,
                "d" => Attribute::Dark,
                "h" => Attribute::Hit,
                _ => unreachable!()
            };
            Ok(WeaponAttribute {
                attr: attr,
                value: value,
            })
        }
        else {
            Err(ItemParseError::UnknownAttribute(String::from(attr)))
        }
    }
}

impl WeaponAttribute {
    fn as_value(&self) -> u16 {
        let attr = match self.attr {
            Attribute::Native => 1,
            Attribute::ABeast => 2,
            Attribute::Machine => 3,
            Attribute::Dark => 4,
            Attribute::Hit => 5,
        };

        ((attr as u16) << 8) | self.value as u16
    }
}



#[derive(Debug)]
pub enum WeaponSpecial {
    Draw,
    Drain,
    Fill,
    Gush,
    Heart,
    Mind,
    Soul,
    Geist,
    Masters,
    Lords,
    Kings,
    Charge,
    Spirit,
    Berserk,
    Ice,
    Frost,
    Freeze,
    Blizzard,
    Bind,
    Hold,
    Seize,
    Arrest,
    Heat,
    Fire,
    Flame,
    Burning,
    Shock,
    Thunder,
    Storm,
    Tempest,
    Dim,
    Shadow,
    Dark,
    Hell,
    Panic,
    Riot,
    Havoc,
    Chaos,
    Devils,
    Demons,
}

impl TryFrom<&str> for WeaponSpecial {
    type Error = ItemParseError;
    fn try_from(special: &str) -> Result<WeaponSpecial, ItemParseError> {
        match special {
            "draw" => Ok(WeaponSpecial::Draw),
            "drain" => Ok(WeaponSpecial::Drain),
            "fill" => Ok(WeaponSpecial::Fill),
            "gush" => Ok(WeaponSpecial::Gush),
            "heart" => Ok(WeaponSpecial::Heart),
            "mind" => Ok(WeaponSpecial::Mind),
            "soul" => Ok(WeaponSpecial::Soul),
            "geist" => Ok(WeaponSpecial::Geist),
            "masters" => Ok(WeaponSpecial::Masters),
            "lords" => Ok(WeaponSpecial::Lords),
            "kings" => Ok(WeaponSpecial::Kings),
            "charge" => Ok(WeaponSpecial::Charge),
            "spirit" => Ok(WeaponSpecial::Spirit),
            "berserk" => Ok(WeaponSpecial::Berserk),
            "ice" => Ok(WeaponSpecial::Ice),
            "frost" => Ok(WeaponSpecial::Frost),
            "freeze" => Ok(WeaponSpecial::Freeze),
            "blizzard" => Ok(WeaponSpecial::Blizzard),
            "bind" => Ok(WeaponSpecial::Bind),
            "hold" => Ok(WeaponSpecial::Hold),
            "seize" => Ok(WeaponSpecial::Seize),
            "arrest" => Ok(WeaponSpecial::Arrest),
            "heat" => Ok(WeaponSpecial::Heat),
            "fire" => Ok(WeaponSpecial::Fire),
            "flame" => Ok(WeaponSpecial::Flame),
            "burning" => Ok(WeaponSpecial::Burning),
            "shock" => Ok(WeaponSpecial::Shock),
            "thunder" => Ok(WeaponSpecial::Thunder),
            "storm" => Ok(WeaponSpecial::Storm),
            "tempest" => Ok(WeaponSpecial::Tempest),
            "dim" => Ok(WeaponSpecial::Dim),
            "shadow" => Ok(WeaponSpecial::Shadow),
            "dark" => Ok(WeaponSpecial::Dark),
            "hell" => Ok(WeaponSpecial::Hell),
            "panic" => Ok(WeaponSpecial::Panic),
            "riot" => Ok(WeaponSpecial::Riot),
            "havoc" => Ok(WeaponSpecial::Havoc),
            "chaos" => Ok(WeaponSpecial::Chaos),
            "devils" => Ok(WeaponSpecial::Devils),
            "demons" => Ok(WeaponSpecial::Demons),
            _ => Err(ItemParseError::UnknownSpecial(String::from(special)))
        }
    }
}

impl WeaponSpecial {
    fn as_value(&self) -> u8 {
        match self {
            WeaponSpecial::Draw => 0x01,
            WeaponSpecial::Drain => 0x02,
            WeaponSpecial::Fill => 0x03,
            WeaponSpecial::Gush => 0x04,
            WeaponSpecial::Heart => 0x05,
            WeaponSpecial::Mind => 0x06,
            WeaponSpecial::Soul => 0x07,
            WeaponSpecial::Geist => 0x08,
            WeaponSpecial::Masters => 0x09,
            WeaponSpecial::Lords => 0x0A,
            WeaponSpecial::Kings => 0x0b,
            WeaponSpecial::Charge => 0x0C,
            WeaponSpecial::Spirit => 0x0D,
            WeaponSpecial::Berserk => 0x0E,
            WeaponSpecial::Ice => 0x0F,
            WeaponSpecial::Frost => 0x10,
            WeaponSpecial::Freeze => 0x11,
            WeaponSpecial::Blizzard => 0x12,
            WeaponSpecial::Bind => 0x13,
            WeaponSpecial::Hold => 0x14,
            WeaponSpecial::Seize => 0x15,
            WeaponSpecial::Arrest => 0x16,
            WeaponSpecial::Heat => 0x17,
            WeaponSpecial::Fire => 0x18,
            WeaponSpecial::Flame => 0x19,
            WeaponSpecial::Burning => 0x1A,
            WeaponSpecial::Shock => 0x1B,
            WeaponSpecial::Thunder => 0x1C,
            WeaponSpecial::Storm => 0x1D,
            WeaponSpecial::Tempest => 0x1E,
            WeaponSpecial::Dim => 0x1F,
            WeaponSpecial::Shadow => 0x20,
            WeaponSpecial::Dark => 0x21,
            WeaponSpecial::Hell => 0x22,
            WeaponSpecial::Panic => 0x23,
            WeaponSpecial::Riot => 0x24,
            WeaponSpecial::Havoc => 0x25,
            WeaponSpecial::Chaos => 0x26,
            WeaponSpecial::Devils => 0x27,
            WeaponSpecial::Demons => 0x28,
        }
    }
}

#[derive(Debug)]
pub enum WeaponType {
    Saber,
    Brand,
    Buster,
    Pallasch,
    Gladius,
    DBsSaber,
    Kaladbolg,
    Durandal,
    Sword,
    Gigush,
    Breaker,
    Claymore,
    Calibur,
    FlowensSword,
    LastSurvivor,
    DragonSlayer,
    Dagger,
    Knife,
    Blade,
    Edge,
    Ripper,
    BladeDance,
    BloodyArt,
    CrossScar,
    Partisan,
    Halbert,
    Glaive,
    Berdys,
    Gungnir,
    Brionac,
    Vjaya,
    GaeBolg,
    Slicer,
    Spinner,
    Cutter,
    Sawcer,
    Diska,
    SlicerofAssassin,
    DiskaofLiberator,
    DiskaofBraveman,
    Handgun,
    Autogun,
    Lockgun,
    Railgun,
    Raygun,
    Varista,
    CustomRayverOO,
    Bravace,
    Rifle,
    Sniper,
    Blaster,
    Beam,
    Laser,
    Visk235W,
    WalsMK2,
    Justy23ST,
    Mechgun,
    Assault,
    Repeater,
    Gatling,
    Vulcan,
    MA60Vise,
    HS25Justice,
    LK14Combat,
    Shot,
    Spread,
    Cannon,
    Launcher,
    Arms,
    CrushBullet,
    MeteorSmash,
    FinalImpact,
    Cane,
    Stick,
    Mace,
    Club,
    ClubofLaconium,
    MaceofAdaman,
    ClubofZumiuran,
    Rod,
    Pole,
    Pillar,
    Striker,
    BattleVerge,
    BraveHammer,
    AliveAqhu,
    Wand,
    Staff,
    Baton,
    Scepter,
    FireScepterAgni,
    IceStaffDagon,
    StormWandIndra,
    PhotonClaw,
    SilenceClaw,
    NeisClaw1,
    DoubleSaber,
    StagCutlery,
    TwinBrand,
    BraveKnuckle,
    AngryFist,
    GodHand,
    Orotiagito,
    Agito1,
    Agito2,
    Agito3,
    Agito4,
    Agito5,
    Agito6,
    Raikiri,
    SoulEater,
    SoulBanish,
    SpreadNeedle,
    HolyRay,
    InfernoBazooka,
    FlameVisit,
    BurningVisit,
    AkikosFryingPan,
    SorcerersCane,
    SBeatsBlade,
    PArmsBlade,
    DelsabersBuster,
    BringersRifle,
    EggBlaster,
    PsychoWand,
    HeavenPunisher,
    LavisCannon,
    VictorAxe,
    LaconiumAxe,
    ChainSawd,
    Caduceus,
    StingTip,
    MagicalPiece,
    TechnicalCrozier,
    SuppressedGun,
    AncientSaber,
    HarisenBattleFan,
    Yamigarasu,
    AkikosWok,
    ToyHammer,
    Elysion,
    RedSaber,
    MeteorCudgel,
    MonkeyKingBar,
    BlackKingBar,
    DoubleCannon,
    HugeBattleFan,
    TsumikiriJSword,
    SealedJSword,
    RedSword,
    CrazyTune,
    TwinChakram,
    WokofAkikosShop,
    LavisBlade,
    RedDagger,
    MadamsParasol,
    MadamsUmbrella,
    ImperialPick,
    Berdysh,
    RedPartisan,
    FlightCutter,
    FlightFan,
    RedSlicer,
    HandgunGuld,
    HandgunMilla,
    RedHandgun,
    FrozenShooter,
    SnowQueen,
    AntiAndroidRifle,
    RocketPunch,
    SambaMaracas,
    TwinPsychogun,
    DrillLauncher,
    GuldMilla,
    RedMechgun,
    BelraCannon,
    PanzerFaust,
    IronFaust,
    SummitMoon,
    Windmill,
    EvilCurst,
    FlowerCane,
    HildebearsCane,
    HildebluesCane,
    RabbitWand,
    PlantainLeaf,
    Fatsia,
    DemonicFork,
    StrikerofChao,
    Broom,
    ProphetsofMotav,
    TheSighofaGod,
    TwinkleStar,
    PlantainFan,
    TwinBlaze,
    MarinasBag,
    DragonsClaw,
    PanthersClaw,
    SRedsBlade,
    PlantainHugeFan,
    ChameleonScythe,
    Yasminkov3000R,
    AnoRifle,
    BaranzLauncher,
    BranchofPakupaku,
    HeartofPoumn,
    Yasminkov2000H,
    Yasminkov7000V,
    Yasminkov9000M,
    MaserBeam,
    PowerMaser,
    GameMagazine,
    FlowerBouquet,
    Musashi,
    Yamato,
    Asuka,
    SangeYasha,
    Sange,
    Yasha,
    PhotonLauncher,
    GuiltyLight,
    RedScorpio,
    Talis,
    Mahu,
    Hitogata,
    DancingHitogata,
    Nug2000Bazooka,
    SBerillsHands0,
    SBerillsHands1,
    FlowensSword1,
    FlowensSword2,
    FlowensSword3,
    FlowensSword4,
    FlowensSword5,
    FlowensSword6,
    FlowensSword7,
    FlowensSword8,
    FlowensSword9,
    DBsSaber1,
    DBsSaber2,
    DBsSaber3,
    DBsSaber4,
    DBsSaber5,
    DBsSaber6,
    DBsSaber7,
    DBsSaber8,
    DBsSaber9,
    GiGueBazooka,
    Guardianna,
    ViridiaCard,
    GreenillCard,
    SkylyCard,
    BluefullCard,
    PurplenumCard,
    PinkalCard,
    RedriaCard,
    OranCard,
    YellowbozeCard,
    WhitillCard,
    MorningGlory,
    PartisanofLightning,
    GalWind,
    Zanba,
    RikasClaw,
    AngelHarp,
    DemolitionComet,
    NeisClaw2,
    RainbowBaton,
    DarkFlow,
    DarkMeteor,
    DarkBridge,
    GAssassinsSabers,
    RappysFan,
    BoomasClaw,
    GoboomasClaw,
    GigoboomasClaw,
    RubyBullet,
    AmoreRose,
}

impl TryFrom<&str> for WeaponType {
    type Error = ItemParseError;
    fn try_from(value: &str) -> Result<WeaponType, Self::Error> {
        match value {
            "saber" => Ok(WeaponType::Saber),
            "brand" => Ok(WeaponType::Brand),
            "buster" => Ok(WeaponType::Buster),
            "pallasch" => Ok(WeaponType::Pallasch),
            "gladius" => Ok(WeaponType::Gladius),
            "dbssaber" => Ok(WeaponType::DBsSaber),
            "kaladbolg" => Ok(WeaponType::Kaladbolg),
            "durandal" => Ok(WeaponType::Durandal),
            "sword" => Ok(WeaponType::Sword),
            "gigush" => Ok(WeaponType::Gigush),
            "breaker" => Ok(WeaponType::Breaker),
            "claymore" => Ok(WeaponType::Claymore),
            "calibur" => Ok(WeaponType::Calibur),
            "flowenssword" => Ok(WeaponType::FlowensSword),
            "lastsurvivor" => Ok(WeaponType::LastSurvivor),
            "dragonslayer" => Ok(WeaponType::DragonSlayer),
            "dagger" => Ok(WeaponType::Dagger),
            "knife" => Ok(WeaponType::Knife),
            "blade" => Ok(WeaponType::Blade),
            "edge" => Ok(WeaponType::Edge),
            "ripper" => Ok(WeaponType::Ripper),
            "bladedance" => Ok(WeaponType::BladeDance),
            "bloodyart" => Ok(WeaponType::BloodyArt),
            "crossscar" => Ok(WeaponType::CrossScar),
            "partisan" => Ok(WeaponType::Partisan),
            "halbert" => Ok(WeaponType::Halbert),
            "glaive" => Ok(WeaponType::Glaive),
            "berdys" => Ok(WeaponType::Berdys),
            "gungnir" => Ok(WeaponType::Gungnir),
            "brionac" => Ok(WeaponType::Brionac),
            "vjaya" => Ok(WeaponType::Vjaya),
            "gaebolg" => Ok(WeaponType::GaeBolg),
            "slicer" => Ok(WeaponType::Slicer),
            "spinner" => Ok(WeaponType::Spinner),
            "cutter" => Ok(WeaponType::Cutter),
            "sawcer" => Ok(WeaponType::Sawcer),
            "diska" => Ok(WeaponType::Diska),
            "slicerofassassin" => Ok(WeaponType::SlicerofAssassin),
            "diskaofliberator" => Ok(WeaponType::DiskaofLiberator),
            "diskaofbraveman" => Ok(WeaponType::DiskaofBraveman),
            "handgun" => Ok(WeaponType::Handgun),
            "autogun" => Ok(WeaponType::Autogun),
            "lockgun" => Ok(WeaponType::Lockgun),
            "railgun" => Ok(WeaponType::Railgun),
            "raygun" | "rg" => Ok(WeaponType::Raygun),
            "varista" => Ok(WeaponType::Varista),
            "customrayveroo" => Ok(WeaponType::CustomRayverOO),
            "bravace" => Ok(WeaponType::Bravace),
            "rifle" => Ok(WeaponType::Rifle),
            "sniper" => Ok(WeaponType::Sniper),
            "blaster" => Ok(WeaponType::Blaster),
            "beam" => Ok(WeaponType::Beam),
            "laser" => Ok(WeaponType::Laser),
            "visk235w" => Ok(WeaponType::Visk235W),
            "walsmk2" => Ok(WeaponType::WalsMK2),
            "justy23st" => Ok(WeaponType::Justy23ST),
            "mechgun" => Ok(WeaponType::Mechgun),
            "assault" => Ok(WeaponType::Assault),
            "repeater" => Ok(WeaponType::Repeater),
            "gatling" => Ok(WeaponType::Gatling),
            "vulcan" | "vulc" => Ok(WeaponType::Vulcan),
            "ma60vise" => Ok(WeaponType::MA60Vise),
            "hs25justice" => Ok(WeaponType::HS25Justice),
            "lk14combat" => Ok(WeaponType::LK14Combat),
            "shot" => Ok(WeaponType::Shot),
            "spread" => Ok(WeaponType::Spread),
            "cannon" => Ok(WeaponType::Cannon),
            "launcher" => Ok(WeaponType::Launcher),
            "arms" => Ok(WeaponType::Arms),
            "crushbullet" => Ok(WeaponType::CrushBullet),
            "meteorsmash" => Ok(WeaponType::MeteorSmash),
            "finalimpact" => Ok(WeaponType::FinalImpact),
            "cane" => Ok(WeaponType::Cane),
            "stick" => Ok(WeaponType::Stick),
            "mace" => Ok(WeaponType::Mace),
            "club" => Ok(WeaponType::Club),
            "cluboflaconium" => Ok(WeaponType::ClubofLaconium),
            "maceofadaman" => Ok(WeaponType::MaceofAdaman),
            "clubofzumiuran" => Ok(WeaponType::ClubofZumiuran),
            "rod" => Ok(WeaponType::Rod),
            "pole" => Ok(WeaponType::Pole),
            "pillar" => Ok(WeaponType::Pillar),
            "striker" => Ok(WeaponType::Striker),
            "battleverge" => Ok(WeaponType::BattleVerge),
            "bravehammer" => Ok(WeaponType::BraveHammer),
            "aliveaqhu" => Ok(WeaponType::AliveAqhu),
            "wand" => Ok(WeaponType::Wand),
            "staff" => Ok(WeaponType::Staff),
            "baton" => Ok(WeaponType::Baton),
            "scepter" => Ok(WeaponType::Scepter),
            "firescepteragni" | "agni" => Ok(WeaponType::FireScepterAgni),
            "icestaffdagon" | "dagon" => Ok(WeaponType::IceStaffDagon),
            "stormwandindra" | "indra" => Ok(WeaponType::StormWandIndra),
            "photonclaw" => Ok(WeaponType::PhotonClaw),
            "silenceclaw" => Ok(WeaponType::SilenceClaw),
            "neisclaw1" => Ok(WeaponType::NeisClaw1),
            "doublesaber" => Ok(WeaponType::DoubleSaber),
            "stagcutlery" => Ok(WeaponType::StagCutlery),
            "twinbrand" => Ok(WeaponType::TwinBrand),
            "braveknuckle" => Ok(WeaponType::BraveKnuckle),
            "angryfist" => Ok(WeaponType::AngryFist),
            "godhand" => Ok(WeaponType::GodHand),
            "orotiagito" => Ok(WeaponType::Orotiagito),
            "agito1" => Ok(WeaponType::Agito1),
            "agito2" => Ok(WeaponType::Agito2),
            "agito3" => Ok(WeaponType::Agito3),
            "agito4" => Ok(WeaponType::Agito4),
            "agito5" => Ok(WeaponType::Agito5),
            "agito6" => Ok(WeaponType::Agito6),
            "raikiri" => Ok(WeaponType::Raikiri),
            "souleater" => Ok(WeaponType::SoulEater),
            "soulbanish" => Ok(WeaponType::SoulBanish),
            "spreadneedle" | "sn" => Ok(WeaponType::SpreadNeedle),
            "holyray" => Ok(WeaponType::HolyRay),
            "infernobazooka" => Ok(WeaponType::InfernoBazooka),
            "flamevisit" => Ok(WeaponType::FlameVisit),
            "burningvisit" => Ok(WeaponType::BurningVisit),
            "akikosfryingpan" => Ok(WeaponType::AkikosFryingPan),
            "sorcererscane" => Ok(WeaponType::SorcerersCane),
            "sbeatsblade" => Ok(WeaponType::SBeatsBlade),
            "parmsblade" => Ok(WeaponType::PArmsBlade),
            "delsabersbuster" => Ok(WeaponType::DelsabersBuster),
            "bringersrifle" => Ok(WeaponType::BringersRifle),
            "eggblaster" => Ok(WeaponType::EggBlaster),
            "psychowand" | "pwand" => Ok(WeaponType::PsychoWand),
            "heavenpunisher" => Ok(WeaponType::HeavenPunisher),
            "laviscannon" => Ok(WeaponType::LavisCannon),
            "victoraxe" => Ok(WeaponType::VictorAxe),
            "laconiumaxe" => Ok(WeaponType::LaconiumAxe),
            "chainsawd" => Ok(WeaponType::ChainSawd),
            "caduceus" => Ok(WeaponType::Caduceus),
            "stingtip" => Ok(WeaponType::StingTip),
            "magicalpiece" => Ok(WeaponType::MagicalPiece),
            "technicalcrozier" => Ok(WeaponType::TechnicalCrozier),
            "suppressedgun" => Ok(WeaponType::SuppressedGun),
            "ancientsaber" => Ok(WeaponType::AncientSaber),
            "harisenbattlefan" => Ok(WeaponType::HarisenBattleFan),
            "yamigarasu" => Ok(WeaponType::Yamigarasu),
            "akikoswok" => Ok(WeaponType::AkikosWok),
            "toyhammer" => Ok(WeaponType::ToyHammer),
            "elysion" => Ok(WeaponType::Elysion),
            "redsaber" => Ok(WeaponType::RedSaber),
            "meteorcudgel" => Ok(WeaponType::MeteorCudgel),
            "monkeykingbar" => Ok(WeaponType::MonkeyKingBar),
            "blackkingbar" => Ok(WeaponType::BlackKingBar),
            "doublecannon" => Ok(WeaponType::DoubleCannon),
            "hugebattlefan" => Ok(WeaponType::HugeBattleFan),
            "tsumikirijsword" | "tjs" => Ok(WeaponType::TsumikiriJSword),
            "sealedjsword" | "sjs" => Ok(WeaponType::SealedJSword),
            "redsword" => Ok(WeaponType::RedSword),
            "crazytune" => Ok(WeaponType::CrazyTune),
            "twinchakram" => Ok(WeaponType::TwinChakram),
            "wokofakikosshop" => Ok(WeaponType::WokofAkikosShop),
            "lavisblade" => Ok(WeaponType::LavisBlade),
            "reddagger" => Ok(WeaponType::RedDagger),
            "madamsparasol" => Ok(WeaponType::MadamsParasol),
            "madamsumbrella" => Ok(WeaponType::MadamsUmbrella),
            "imperialpick" => Ok(WeaponType::ImperialPick),
            "berdysh" => Ok(WeaponType::Berdysh),
            "redpartisan" => Ok(WeaponType::RedPartisan),
            "flightcutter" => Ok(WeaponType::FlightCutter),
            "flightfan" => Ok(WeaponType::FlightFan),
            "redslicer" => Ok(WeaponType::RedSlicer),
            "handgunguld" => Ok(WeaponType::HandgunGuld),
            "handgunmilla" => Ok(WeaponType::HandgunMilla),
            "redhandgun" => Ok(WeaponType::RedHandgun),
            "frozenshooter" | "fs" => Ok(WeaponType::FrozenShooter),
            "snowqueen" | "sq" => Ok(WeaponType::SnowQueen),
            "antiandroidrifle" => Ok(WeaponType::AntiAndroidRifle),
            "rocketpunch" => Ok(WeaponType::RocketPunch),
            "sambamaracas" => Ok(WeaponType::SambaMaracas),
            "twinpsychogun" => Ok(WeaponType::TwinPsychogun),
            "drilllauncher" => Ok(WeaponType::DrillLauncher),
            "guldmilla" => Ok(WeaponType::GuldMilla),
            "redmechgun" => Ok(WeaponType::RedMechgun),
            "belracannon" => Ok(WeaponType::BelraCannon),
            "panzerfaust" => Ok(WeaponType::PanzerFaust),
            "ironfaust" => Ok(WeaponType::IronFaust),
            "summitmoon" => Ok(WeaponType::SummitMoon),
            "windmill" => Ok(WeaponType::Windmill),
            "evilcurst" => Ok(WeaponType::EvilCurst),
            "flowercane" => Ok(WeaponType::FlowerCane),
            "hildebearscane" => Ok(WeaponType::HildebearsCane),
            "hildebluescane" => Ok(WeaponType::HildebluesCane),
            "rabbitwand" => Ok(WeaponType::RabbitWand),
            "plantainleaf" => Ok(WeaponType::PlantainLeaf),
            "fatsia" => Ok(WeaponType::Fatsia),
            "demonicfork" => Ok(WeaponType::DemonicFork),
            "strikerofchao" => Ok(WeaponType::StrikerofChao),
            "broom" => Ok(WeaponType::Broom),
            "prophetsofmotav" => Ok(WeaponType::ProphetsofMotav),
            "thesighofagod" => Ok(WeaponType::TheSighofaGod),
            "twinklestar" => Ok(WeaponType::TwinkleStar),
            "plantainfan" => Ok(WeaponType::PlantainFan),
            "twinblaze" => Ok(WeaponType::TwinBlaze),
            "marinasbag" => Ok(WeaponType::MarinasBag),
            "dragonsclaw" => Ok(WeaponType::DragonsClaw),
            "panthersclaw" => Ok(WeaponType::PanthersClaw),
            "sredsblade" => Ok(WeaponType::SRedsBlade),
            "plantainhugefan" => Ok(WeaponType::PlantainHugeFan),
            "chameleonscythe" => Ok(WeaponType::ChameleonScythe),
            "yasminkov3000r" => Ok(WeaponType::Yasminkov3000R),
            "anorifle" => Ok(WeaponType::AnoRifle),
            "baranzlauncher" => Ok(WeaponType::BaranzLauncher),
            "branchofpakupaku" => Ok(WeaponType::BranchofPakupaku),
            "heartofpoumn" => Ok(WeaponType::HeartofPoumn),
            "yasminkov2000h" => Ok(WeaponType::Yasminkov2000H),
            "yasminkov7000v" => Ok(WeaponType::Yasminkov7000V),
            "yasminkov9000m" | "yas9k" => Ok(WeaponType::Yasminkov9000M),
            "maserbeam" => Ok(WeaponType::MaserBeam),
            "powermaser" => Ok(WeaponType::PowerMaser),
            "gamemagazine" => Ok(WeaponType::GameMagazine),
            "flowerbouquet" => Ok(WeaponType::FlowerBouquet),
            "musashi" => Ok(WeaponType::Musashi),
            "yamato" => Ok(WeaponType::Yamato),
            "asuka" => Ok(WeaponType::Asuka),
            "sangeyasha" => Ok(WeaponType::SangeYasha),
            "sange" => Ok(WeaponType::Sange),
            "yasha" => Ok(WeaponType::Yasha),
            "photonlauncher" => Ok(WeaponType::PhotonLauncher),
            "guiltylight" => Ok(WeaponType::GuiltyLight),
            "redscorpio" => Ok(WeaponType::RedScorpio),
            "talis" => Ok(WeaponType::Talis),
            "mahu" => Ok(WeaponType::Mahu),
            "hitogata" => Ok(WeaponType::Hitogata),
            "dancinghitogata" => Ok(WeaponType::DancingHitogata),
            "nug2000bazooka" => Ok(WeaponType::Nug2000Bazooka),
            "sberillshands0" => Ok(WeaponType::SBerillsHands0),
            "sberillshands1" => Ok(WeaponType::SBerillsHands1),
            "flowenssword1" => Ok(WeaponType::FlowensSword1),
            "flowenssword2" => Ok(WeaponType::FlowensSword2),
            "flowenssword3" => Ok(WeaponType::FlowensSword3),
            "flowenssword4" => Ok(WeaponType::FlowensSword4),
            "flowenssword5" => Ok(WeaponType::FlowensSword5),
            "flowenssword6" => Ok(WeaponType::FlowensSword6),
            "flowenssword7" => Ok(WeaponType::FlowensSword7),
            "flowenssword8" => Ok(WeaponType::FlowensSword8),
            "flowenssword9" => Ok(WeaponType::FlowensSword9),
            "dbssaber1" => Ok(WeaponType::DBsSaber1),
            "dbssaber2" => Ok(WeaponType::DBsSaber2),
            "dbssaber3" => Ok(WeaponType::DBsSaber3),
            "dbssaber4" => Ok(WeaponType::DBsSaber4),
            "dbssaber5" => Ok(WeaponType::DBsSaber5),
            "dbssaber6" => Ok(WeaponType::DBsSaber6),
            "dbssaber7" => Ok(WeaponType::DBsSaber7),
            "dbssaber8" => Ok(WeaponType::DBsSaber8),
            "dbssaber9" => Ok(WeaponType::DBsSaber9),
            "giguebazooka" => Ok(WeaponType::GiGueBazooka),
            "guardianna" => Ok(WeaponType::Guardianna),
            "viridiacard" => Ok(WeaponType::ViridiaCard),
            "greenillcard" => Ok(WeaponType::GreenillCard),
            "skylycard" => Ok(WeaponType::SkylyCard),
            "bluefullcard" => Ok(WeaponType::BluefullCard),
            "purplenumcard" => Ok(WeaponType::PurplenumCard),
            "pinkalcard" => Ok(WeaponType::PinkalCard),
            "redriacard" => Ok(WeaponType::RedriaCard),
            "orancard" => Ok(WeaponType::OranCard),
            "yellowbozecard" => Ok(WeaponType::YellowbozeCard),
            "whitillcard" => Ok(WeaponType::WhitillCard),
            "morningglory" => Ok(WeaponType::MorningGlory),
            "partisanoflightning" => Ok(WeaponType::PartisanofLightning),
            "galwind" => Ok(WeaponType::GalWind),
            "zanba" => Ok(WeaponType::Zanba),
            "rikasclaw" => Ok(WeaponType::RikasClaw),
            "angelharp" => Ok(WeaponType::AngelHarp),
            "demolitioncomet" => Ok(WeaponType::DemolitionComet),
            "neisclaw2" => Ok(WeaponType::NeisClaw2),
            "rainbowbaton" => Ok(WeaponType::RainbowBaton),
            "darkflow" | "df" => Ok(WeaponType::DarkFlow),
            "darkmeteor" => Ok(WeaponType::DarkMeteor),
            "darkbridge" => Ok(WeaponType::DarkBridge),
            "gassassinssabers" => Ok(WeaponType::GAssassinsSabers),
            "rappysfan" => Ok(WeaponType::RappysFan),
            "boomasclaw" => Ok(WeaponType::BoomasClaw),
            "goboomasclaw" => Ok(WeaponType::GoboomasClaw),
            "gigoboomasclaw" => Ok(WeaponType::GigoboomasClaw),
            "rubybullet" => Ok(WeaponType::RubyBullet),
            "amorerose" => Ok(WeaponType::AmoreRose),
            _ => Err(ItemParseError::UnknownItem(String::from(value)))
        }
    }
}

impl WeaponType {
    pub fn as_value(&self) -> u32 {
        match self {
            WeaponType::Saber => 0x000100,
            WeaponType::Brand => 0x000101,
            WeaponType::Buster => 0x000102,
            WeaponType::Pallasch => 0x000103,
            WeaponType::Gladius => 0x000104,
            WeaponType::DBsSaber => 0x000105,
            WeaponType::Kaladbolg => 0x000106,
            WeaponType::Durandal => 0x000107,
            WeaponType::Sword => 0x000200,
            WeaponType::Gigush => 0x000201,
            WeaponType::Breaker => 0x000202,
            WeaponType::Claymore => 0x000203,
            WeaponType::Calibur => 0x000204,
            WeaponType::FlowensSword => 0x000205,
            WeaponType::LastSurvivor => 0x000206,
            WeaponType::DragonSlayer => 0x000207,
            WeaponType::Dagger => 0x000300,
            WeaponType::Knife => 0x000301,
            WeaponType::Blade => 0x000302,
            WeaponType::Edge => 0x000303,
            WeaponType::Ripper => 0x000304,
            WeaponType::BladeDance => 0x000305,
            WeaponType::BloodyArt => 0x000306,
            WeaponType::CrossScar => 0x000307,
            WeaponType::Partisan => 0x000400,
            WeaponType::Halbert => 0x000401,
            WeaponType::Glaive => 0x000402,
            WeaponType::Berdys => 0x000403,
            WeaponType::Gungnir => 0x000404,
            WeaponType::Brionac => 0x000405,
            WeaponType::Vjaya => 0x000406,
            WeaponType::GaeBolg => 0x000407,
            WeaponType::Slicer => 0x000500,
            WeaponType::Spinner => 0x000501,
            WeaponType::Cutter => 0x000502,
            WeaponType::Sawcer => 0x000503,
            WeaponType::Diska => 0x000504,
            WeaponType::SlicerofAssassin => 0x000505,
            WeaponType::DiskaofLiberator => 0x000506,
            WeaponType::DiskaofBraveman => 0x000507,
            WeaponType::Handgun => 0x000600,
            WeaponType::Autogun => 0x000601,
            WeaponType::Lockgun => 0x000602,
            WeaponType::Railgun => 0x000603,
            WeaponType::Raygun => 0x000604,
            WeaponType::Varista => 0x000605,
            WeaponType::CustomRayverOO => 0x000606,
            WeaponType::Bravace => 0x000607,
            WeaponType::Rifle => 0x000700,
            WeaponType::Sniper => 0x000701,
            WeaponType::Blaster => 0x000702,
            WeaponType::Beam => 0x000703,
            WeaponType::Laser => 0x000704,
            WeaponType::Visk235W => 0x000705,
            WeaponType::WalsMK2 => 0x000706,
            WeaponType::Justy23ST => 0x000707,
            WeaponType::Mechgun => 0x000800,
            WeaponType::Assault => 0x000801,
            WeaponType::Repeater => 0x000802,
            WeaponType::Gatling => 0x000803,
            WeaponType::Vulcan => 0x000804,
            WeaponType::MA60Vise => 0x000805,
            WeaponType::HS25Justice => 0x000806,
            WeaponType::LK14Combat => 0x000807,
            WeaponType::Shot => 0x000900,
            WeaponType::Spread => 0x000901,
            WeaponType::Cannon => 0x000902,
            WeaponType::Launcher => 0x000903,
            WeaponType::Arms => 0x000904,
            WeaponType::CrushBullet => 0x000905,
            WeaponType::MeteorSmash => 0x000906,
            WeaponType::FinalImpact => 0x000907,
            WeaponType::Cane => 0x000A00,
            WeaponType::Stick => 0x000A01,
            WeaponType::Mace => 0x000A02,
            WeaponType::Club => 0x000A03,
            WeaponType::ClubofLaconium => 0x000A04,
            WeaponType::MaceofAdaman => 0x000A05,
            WeaponType::ClubofZumiuran => 0x000A06,
            WeaponType::Rod => 0x000B00,
            WeaponType::Pole => 0x000B01,
            WeaponType::Pillar => 0x000B02,
            WeaponType::Striker => 0x000B03,
            WeaponType::BattleVerge => 0x000B04,
            WeaponType::BraveHammer => 0x000B05,
            WeaponType::AliveAqhu => 0x000B06,
            WeaponType::Wand => 0x000C00,
            WeaponType::Staff => 0x000C01,
            WeaponType::Baton => 0x000C02,
            WeaponType::Scepter => 0x000C03,
            WeaponType::FireScepterAgni => 0x000C04,
            WeaponType::IceStaffDagon => 0x000C05,
            WeaponType::StormWandIndra => 0x000C06,
            WeaponType::PhotonClaw => 0x000D00,
            WeaponType::SilenceClaw => 0x000D01,
            WeaponType::NeisClaw1 => 0x000D02,
            WeaponType::DoubleSaber => 0x000E00,
            WeaponType::StagCutlery => 0x000E01,
            WeaponType::TwinBrand => 0x000E02,
            WeaponType::BraveKnuckle => 0x000F00,
            WeaponType::AngryFist => 0x000F01,
            WeaponType::GodHand => 0x000F02,
            WeaponType::Orotiagito => 0x001000,
            WeaponType::Agito1 => 0x001001,
            WeaponType::Agito2 => 0x001002,
            WeaponType::Agito3 => 0x001003,
            WeaponType::Agito4 => 0x001004,
            WeaponType::Agito5 => 0x001005,
            WeaponType::Agito6 => 0x001006,
            WeaponType::Raikiri => 0x001007,
            WeaponType::SoulEater => 0x001100,
            WeaponType::SoulBanish => 0x001101,
            WeaponType::SpreadNeedle => 0x001200,
            WeaponType::HolyRay => 0x001300,
            WeaponType::InfernoBazooka => 0x001400,
            WeaponType::FlameVisit => 0x001500,
            WeaponType::BurningVisit => 0x001501,
            WeaponType::AkikosFryingPan => 0x001600,
            WeaponType::SorcerersCane => 0x001700,
            WeaponType::SBeatsBlade => 0x001800,
            WeaponType::PArmsBlade => 0x001900,
            WeaponType::DelsabersBuster => 0x001A00,
            WeaponType::BringersRifle => 0x001B00,
            WeaponType::EggBlaster => 0x001C00,
            WeaponType::PsychoWand => 0x001D00,
            WeaponType::HeavenPunisher => 0x001E00,
            WeaponType::LavisCannon => 0x001F00,
            WeaponType::VictorAxe => 0x002000,
            WeaponType::LaconiumAxe => 0x002001,
            WeaponType::ChainSawd => 0x002100,
            WeaponType::Caduceus => 0x002200,
            WeaponType::StingTip => 0x002300,
            WeaponType::MagicalPiece => 0x002400,
            WeaponType::TechnicalCrozier => 0x002500,
            WeaponType::SuppressedGun => 0x002600,
            WeaponType::AncientSaber => 0x002700,
            WeaponType::HarisenBattleFan => 0x002800,
            WeaponType::Yamigarasu => 0x002900,
            WeaponType::AkikosWok => 0x002A00,
            WeaponType::ToyHammer => 0x002B00,
            WeaponType::Elysion => 0x002C00,
            WeaponType::RedSaber => 0x002D00,
            WeaponType::MeteorCudgel => 0x002E00,
            WeaponType::MonkeyKingBar => 0x002F00,
            WeaponType::BlackKingBar => 0x002F01,
            WeaponType::DoubleCannon => 0x003000,
            WeaponType::HugeBattleFan => 0x003100,
            WeaponType::TsumikiriJSword => 0x003200,
            WeaponType::SealedJSword => 0x003300,
            WeaponType::RedSword => 0x003400,
            WeaponType::CrazyTune => 0x003500,
            WeaponType::TwinChakram => 0x003600,
            WeaponType::WokofAkikosShop => 0x003700,
            WeaponType::LavisBlade => 0x003800,
            WeaponType::RedDagger => 0x003900,
            WeaponType::MadamsParasol => 0x003A00,
            WeaponType::MadamsUmbrella => 0x003B00,
            WeaponType::ImperialPick => 0x003C00,
            WeaponType::Berdysh => 0x003D00,
            WeaponType::RedPartisan => 0x003E00,
            WeaponType::FlightCutter => 0x003F00,
            WeaponType::FlightFan => 0x004000,
            WeaponType::RedSlicer => 0x004100,
            WeaponType::HandgunGuld => 0x004200,
            WeaponType::HandgunMilla => 0x004300,
            WeaponType::RedHandgun => 0x004400,
            WeaponType::FrozenShooter => 0x004500,
            WeaponType::SnowQueen => 0x004501,
            WeaponType::AntiAndroidRifle => 0x004600,
            WeaponType::RocketPunch => 0x004700,
            WeaponType::SambaMaracas => 0x004800,
            WeaponType::TwinPsychogun => 0x004900,
            WeaponType::DrillLauncher => 0x004A00,
            WeaponType::GuldMilla => 0x004B00,
            WeaponType::RedMechgun => 0x004C00,
            WeaponType::BelraCannon => 0x004D00,
            WeaponType::PanzerFaust => 0x004E00,
            WeaponType::IronFaust => 0x004E01,
            WeaponType::SummitMoon => 0x004F00,
            WeaponType::Windmill => 0x005000,
            WeaponType::EvilCurst => 0x005100,
            WeaponType::FlowerCane => 0x005200,
            WeaponType::HildebearsCane => 0x005300,
            WeaponType::HildebluesCane => 0x005400,
            WeaponType::RabbitWand => 0x005500,
            WeaponType::PlantainLeaf => 0x005600,
            WeaponType::Fatsia => 0x005601,
            WeaponType::DemonicFork => 0x005700,
            WeaponType::StrikerofChao => 0x005800,
            WeaponType::Broom => 0x005900,
            WeaponType::ProphetsofMotav => 0x005A00,
            WeaponType::TheSighofaGod => 0x005B00,
            WeaponType::TwinkleStar => 0x005C00,
            WeaponType::PlantainFan => 0x005D00,
            WeaponType::TwinBlaze => 0x005E00,
            WeaponType::MarinasBag => 0x005F00,
            WeaponType::DragonsClaw => 0x006000,
            WeaponType::PanthersClaw => 0x006100,
            WeaponType::SRedsBlade => 0x006200,
            WeaponType::PlantainHugeFan => 0x006300,
            WeaponType::ChameleonScythe => 0x006400,
            WeaponType::Yasminkov3000R => 0x006500,
            WeaponType::AnoRifle => 0x006600,
            WeaponType::BaranzLauncher => 0x006700,
            WeaponType::BranchofPakupaku => 0x006800,
            WeaponType::HeartofPoumn => 0x006900,
            WeaponType::Yasminkov2000H => 0x006A00,
            WeaponType::Yasminkov7000V => 0x006B00,
            WeaponType::Yasminkov9000M => 0x006C00,
            WeaponType::MaserBeam => 0x006D00,
            WeaponType::PowerMaser => 0x006D01,
            WeaponType::GameMagazine => 0x006E00,
            WeaponType::FlowerBouquet => 0x006F00,
            WeaponType::Musashi => 0x008900,
            WeaponType::Yamato => 0x008901,
            WeaponType::Asuka => 0x008902,
            WeaponType::SangeYasha => 0x008903,
            WeaponType::Sange => 0x008A00,
            WeaponType::Yasha => 0x008A01,
            WeaponType::PhotonLauncher => 0x008B00,
            WeaponType::GuiltyLight => 0x008B01,
            WeaponType::RedScorpio => 0x008B02,
            WeaponType::Talis => 0x008C00,
            WeaponType::Mahu => 0x008C01,
            WeaponType::Hitogata => 0x008C02,
            WeaponType::DancingHitogata => 0x008C03,
            WeaponType::Nug2000Bazooka => 0x008D00,
            WeaponType::SBerillsHands0 => 0x008E00,
            WeaponType::SBerillsHands1 => 0x008E01,
            WeaponType::FlowensSword1 => 0x008F00,
            WeaponType::FlowensSword2 => 0x008F01,
            WeaponType::FlowensSword3 => 0x008F02,
            WeaponType::FlowensSword4 => 0x008F03,
            WeaponType::FlowensSword5 => 0x008F04,
            WeaponType::FlowensSword6 => 0x008F05,
            WeaponType::FlowensSword7 => 0x008F06,
            WeaponType::FlowensSword8 => 0x008F07,
            WeaponType::FlowensSword9 => 0x008F08,
            WeaponType::DBsSaber1 => 0x009000,
            WeaponType::DBsSaber2 => 0x009001,
            WeaponType::DBsSaber3 => 0x009002,
            WeaponType::DBsSaber4 => 0x009003,
            WeaponType::DBsSaber5 => 0x009004,
            WeaponType::DBsSaber6 => 0x009005,
            WeaponType::DBsSaber7 => 0x009006,
            WeaponType::DBsSaber8 => 0x009007,
            WeaponType::DBsSaber9 => 0x009008,
            WeaponType::GiGueBazooka => 0x009100,
            WeaponType::Guardianna => 0x009200,
            WeaponType::ViridiaCard => 0x009300,
            WeaponType::GreenillCard => 0x009301,
            WeaponType::SkylyCard => 0x009302,
            WeaponType::BluefullCard => 0x009303,
            WeaponType::PurplenumCard => 0x009304,
            WeaponType::PinkalCard => 0x009305,
            WeaponType::RedriaCard => 0x009306,
            WeaponType::OranCard => 0x009307,
            WeaponType::YellowbozeCard => 0x009308,
            WeaponType::WhitillCard => 0x009309,
            WeaponType::MorningGlory => 0x009400,
            WeaponType::PartisanofLightning => 0x009500,
            WeaponType::GalWind => 0x009600,
            WeaponType::Zanba => 0x009700,
            WeaponType::RikasClaw => 0x009800,
            WeaponType::AngelHarp => 0x009900,
            WeaponType::DemolitionComet => 0x009A00,
            WeaponType::NeisClaw2 => 0x009B00,
            WeaponType::RainbowBaton => 0x009C00,
            WeaponType::DarkFlow => 0x009D00,
            WeaponType::DarkMeteor => 0x009E00,
            WeaponType::DarkBridge => 0x009F00,
            WeaponType::GAssassinsSabers => 0x00A000,
            WeaponType::RappysFan => 0x00A100,
            WeaponType::BoomasClaw => 0x00A200,
            WeaponType::GoboomasClaw => 0x00A201,
            WeaponType::GigoboomasClaw => 0x00A202,
            WeaponType::RubyBullet => 0x00A300,
            WeaponType::AmoreRose => 0x00A400,
        }
    }
}

#[derive(Debug)]
pub struct Weapon {
    pub weapon: WeaponType,
    pub special: Option<WeaponSpecial>,
    pub grind: u8,
    pub attrs: [Option<WeaponAttribute>; 3],
}

impl ItemData for Weapon {
    fn row1(&self) -> u32 {
        (self.weapon.as_value() << 8) | (self.grind as u32)
    }
    
    fn row2(&self) -> u32 {
        let mut row2 = 0;
        if let Some(ref special) = self.special {
            row2 |= (special.as_value() as u32) << 24;
        };
        if let Some(ref attr) = self.attrs[0] {
            row2 |= attr.as_value() as u32;
        };
        row2
    }
    
    fn row3(&self) -> u32 {
        let mut row3 = 0;
        if let Some(ref attr) = self.attrs[1] {
            row3 |= (attr.as_value() as u32 ) << 16
        };
        if let Some(ref attr) = self.attrs[2] {
            row3 |= attr.as_value() as u32
        };
        row3
    }

    fn row4(&self) -> u32 {
        0
    }
}

#[derive(Debug)]
pub enum ESWeaponType {
}

struct ESWeapon {
    weapon: ESWeaponType,
    name: String,
}

// armor
// shield
// unit

pub enum ToolType {
    
}

pub struct Tool {
    tool: ToolType,
    stack: u8,
}

#[derive(Debug)]
pub enum TechType {
    Shifta,
}

impl TryFrom<&str> for TechType {
    type Error = ItemParseError;
    fn try_from(tech: &str) -> Result<TechType, ItemParseError> {
        match tech {
            "shifta" => Ok(TechType::Shifta),
            _ => Err(ItemParseError::UnknownTech(String::from(tech)))
        }
    }
}

impl TechType {
    fn as_value(&self) -> u8 {
        match self {
            TechType::Shifta => 0x0D,
        }
    }
}

#[derive(Debug)]
pub struct Tech {
    pub tech: TechType,
    pub level: u8
}

impl ItemData for Tech {
    fn row1(&self) -> u32 {
        0x03020000 | ((self.level as u32) << 8)
    }
    
    fn row2(&self) -> u32 {
        (self.tech.as_value() as u32) << 24
    }

    fn row3(&self) -> u32 {
        0
    }

    fn row4(&self) -> u32 {
        0
    }
}


pub enum MagType {
}

pub enum PhotonBlast {
    Farlla,
    Estlla,
    Leilla,
    Pilla,
    Golla,
    MyllaYoulla,
}

pub struct Mag {
    mag: MagType,
    iq: u8,
    sync: u8,
    def: u16,
    pow: u16,
    dex: u16,
    min: u16,
    pbs: [Option<PhotonBlast>; 3],
}

pub struct Meseta {
    amount: u32,
}

#[derive(Debug)]
pub struct RawItemData {
    pub data: Vec<u8>,
}

impl RawItemData {
    fn get_row(&self, row: usize) -> u32 {
        let mut result = 0;
        for i in 0..4 {
            result |= match self.data.get(row*4 + i) {
                Some(v) => (*v as u32) << (i*8),
                None => 0,
            };
        }
        result
    }
}

impl ItemData for RawItemData {
    fn row1(&self) -> u32 {
        self.get_row(0)
    }
    
    fn row2(&self) -> u32 {
        self.get_row(1)
    }

    fn row3(&self) -> u32 {
        self.get_row(2)
    }

    fn row4(&self) -> u32 {
        self.get_row(3)
    }
}

pub enum Item {
    Weapon(Weapon),
    //armor
    //shield
    Tool(Tool),
    //tech
    Mag(Mag),
    Meseta(Meseta),
    RawItemData(RawItemData),
}
