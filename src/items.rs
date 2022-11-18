#![allow(unused_must_use)]
#![allow(dead_code)]


use std::convert::TryFrom;
use regex::Regex;

#[derive(Debug)]
pub enum ItemParseError {
    MissingParameter,
    UnknownItem(String),
    UnknownSpecial(String),
    UnknownAttribute(String),
    UnknownTech(String),
    UnknownPhotonBlast(String),
    ParseIntError(std::num::ParseIntError),
    HexError(hex::FromHexError),
    UnknownValue(u32),
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
            "diskaofbraveman" | "dob" => Ok(WeaponType::DiskaofBraveman),
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
    Saber = 0,
    Sword,
    Blade,
    Partisan,
    Slicer,
    Gun,
    Rifle,
    Mechgun,
    Shot,
    Cane,
    Rod,
    Wand,
    Twin,
    Claw,
    Bazooka,
    Needle,
    Scythe,
    Hammer,
    Moon,
    Psychogun,
    Punch,
    Windmill,
    Harisen,
    Katana,
    JCutter,
    Swords = 0x35,
    Launcher,
    Cards,
    Knuckle,
    Axe,
}

impl TryFrom<&str> for ESWeaponType {
    type Error = ItemParseError;
    fn try_from(special: &str) -> Result<ESWeaponType, ItemParseError> {
        match special {
            "saber" => Ok(ESWeaponType::Saber),
            "sword" => Ok(ESWeaponType::Sword),
            "blade" => Ok(ESWeaponType::Blade),
            "partisan" => Ok(ESWeaponType::Partisan),
            "slicer" => Ok(ESWeaponType::Slicer),
            "gun" => Ok(ESWeaponType::Gun),
            "rifle" => Ok(ESWeaponType::Rifle),
            "mechgun" => Ok(ESWeaponType::Mechgun),
            "shot" => Ok(ESWeaponType::Shot),
            "cane" => Ok(ESWeaponType::Cane),
            "rod" => Ok(ESWeaponType::Rod),
            "wand" => Ok(ESWeaponType::Wand),
            "twin" => Ok(ESWeaponType::Twin),
            "claw" => Ok(ESWeaponType::Claw),
            "bazooka" => Ok(ESWeaponType::Bazooka),
            "needle" => Ok(ESWeaponType::Needle),
            "scythe" => Ok(ESWeaponType::Scythe),
            "hammer" => Ok(ESWeaponType::Hammer),
            "moon" => Ok(ESWeaponType::Moon),
            "psychogun" => Ok(ESWeaponType::Psychogun),
            "punch" => Ok(ESWeaponType::Punch),
            "windmill" => Ok(ESWeaponType::Windmill),
            "harisen" => Ok(ESWeaponType::Harisen),
            "katana" => Ok(ESWeaponType::Katana),
            "jcutter" => Ok(ESWeaponType::JCutter),
            "swords" => Ok(ESWeaponType::Swords),
            "launcher" => Ok(ESWeaponType::Launcher),
            "cards" => Ok(ESWeaponType::Cards),
            "knuckle" => Ok(ESWeaponType::Knuckle),
            "axe" => Ok(ESWeaponType::Axe),
            _ => Err(ItemParseError::UnknownSpecial(String::from(special)))
        }
    }
}


#[derive(Debug)]
pub enum ESWeaponSpecial {
    Jellen = 1,
    Zalure,
    HPRegen,
    TPRegen,
    Burning,
    Tempest,
    Blizzard,
    Arrest,
    Chaos,
    Hell,
    Spirit,
    Berserk,
    Demons,
    Gush,
    Geist,
    Kings,
}

impl TryFrom<&str> for ESWeaponSpecial {
    type Error = ItemParseError;
    fn try_from(special: &str) -> Result<ESWeaponSpecial, ItemParseError> {
        match special {
            "jellen" => Ok(ESWeaponSpecial::Jellen),
            "zalure" => Ok(ESWeaponSpecial::Zalure),
            "hpregen" => Ok(ESWeaponSpecial::HPRegen),
            "tpregen" => Ok(ESWeaponSpecial::TPRegen),
            "burning" => Ok(ESWeaponSpecial::Burning),
            "tempest" => Ok(ESWeaponSpecial::Tempest),
            "blizzard" => Ok(ESWeaponSpecial::Blizzard),
            "arrest" => Ok(ESWeaponSpecial::Arrest),
            "chaos" => Ok(ESWeaponSpecial::Chaos),
            "hell" => Ok(ESWeaponSpecial::Hell),
            "spirit" => Ok(ESWeaponSpecial::Spirit),
            "berserk" => Ok(ESWeaponSpecial::Berserk),
            "demons" => Ok(ESWeaponSpecial::Demons),
            "gush" => Ok(ESWeaponSpecial::Gush),
            "geist" => Ok(ESWeaponSpecial::Geist),
            "kings" => Ok(ESWeaponSpecial::Kings),
            _ => Err(ItemParseError::UnknownSpecial(String::from(special)))
        }
    }
}


#[derive(Debug)]
pub struct ESWeapon {
    pub weapon: ESWeaponType,
    pub special: Option<ESWeaponSpecial>,
    pub name: [u8; 8],
    pub grind: u8,
}

impl ItemData for ESWeapon {
    fn row1(&self) -> u32 {
        let special = match &self.special {
            Some(special) => *special as u8,
            None => 0,
        };
        u32::from_be_bytes([0, 0x70 + self.weapon as u8, special, self.grind])
    }

    fn row2(&self) -> u32 {
        let name1: u16 = 0x8000 + (0x20 * (self.name[0] as u16 & 0x3F)) + (self.name[1] as u16 & 0x3F) ;
        let bytes = name1.to_be_bytes();
        u32::from_be_bytes([0, 0, bytes[0], bytes[1]])
    }

    fn row3(&self) -> u32 {
        let name2: u16 = 0x8000 + (0x400 * (self.name[2] as u16 & 0x3F)) + (0x20 * (self.name[3] as u16 & 0x3F)) + (self.name[4] as u16 & 0x3F) ;
        let name3: u16 = 0x8000 + (0x400 * (self.name[5] as u16 & 0x3F)) + (0x20 * (self.name[6] as u16 & 0x3F)) + (self.name[7] as u16 & 0x3F) ;

        let bytes2 = name2.to_be_bytes();
        let bytes3 = name3.to_be_bytes();
        u32::from_be_bytes([bytes2[0], bytes2[1], bytes3[0], bytes3[1]])

    }

    fn row4(&self) -> u32 {
        0
    }
}


#[derive(Debug)]
pub enum ArmorType {
    Frame,
    Armor,
    PsyArmor,
    GigaFrame,
    SoulFrame,
    CrossArmor,
    SolidFrame,
    BraveArmor,
    HyperFrame,
    GrandArmor,
    ShockFrame,
    KingsFrame,
    DragonFrame,
    AbsorbArmor,
    ProtectFrame,
    GeneralArmor,
    PerfectFrame,
    ValiantFrame,
    ImperialArmor,
    HolinessArmor,
    GuardianArmor,
    DivinityArmor,
    UltimateFrame,
    CelestialArmor,
    HunterField,
    RangerField,
    ForceField,
    RevivalGarment,
    SpiritGarment,
    StinkFrame,
    DPartsver101,
    DPartsver210,
    ParasiteWearDeRol,
    ParasiteWearNelgal,
    ParasiteWearVajulla,
    SensePlate,
    GravitonPlate,
    AttributePlate,
    FlowensFrame,
    CustomFrameverOO,
    DBsArmor,
    GuardWave,
    DFField,
    LuminousField,
    ChuChuFever,
    LoveHeart,
    FlameGarment,
    VirusArmorLafuteria,
    BrightnessCircle,
    AuraField,
    ElectroFrame,
    SacredCloth,
    SmokingPlate,
}

impl TryFrom<&str> for ArmorType {
    type Error = ItemParseError;
    fn try_from(value: &str) -> Result<ArmorType, Self::Error> {
        match value {
            "frame" => Ok(ArmorType::Frame),
            "armor" => Ok(ArmorType::Armor),
            "psyarmor" => Ok(ArmorType::PsyArmor),
            "gigaframe" => Ok(ArmorType::GigaFrame),
            "soulframe" => Ok(ArmorType::SoulFrame),
            "crossarmor" => Ok(ArmorType::CrossArmor),
            "solidframe" => Ok(ArmorType::SolidFrame),
            "bravearmor" => Ok(ArmorType::BraveArmor),
            "hyperframe" => Ok(ArmorType::HyperFrame),
            "grandarmor" => Ok(ArmorType::GrandArmor),
            "shockframe" => Ok(ArmorType::ShockFrame),
            "kingsframe" => Ok(ArmorType::KingsFrame),
            "dragonframe" => Ok(ArmorType::DragonFrame),
            "absorbarmor" => Ok(ArmorType::AbsorbArmor),
            "protectframe" => Ok(ArmorType::ProtectFrame),
            "generalarmor" => Ok(ArmorType::GeneralArmor),
            "perfectframe" => Ok(ArmorType::PerfectFrame),
            "valiantframe" => Ok(ArmorType::ValiantFrame),
            "imperialarmor" => Ok(ArmorType::ImperialArmor),
            "holinessarmor" => Ok(ArmorType::HolinessArmor),
            "guardianarmor" => Ok(ArmorType::GuardianArmor),
            "divinityarmor" => Ok(ArmorType::DivinityArmor),
            "ultimateframe" => Ok(ArmorType::UltimateFrame),
            "celestialarmor" => Ok(ArmorType::CelestialArmor),
            "hunterfield" => Ok(ArmorType::HunterField),
            "rangerfield" => Ok(ArmorType::RangerField),
            "forcefield" => Ok(ArmorType::ForceField),
            "revivalgarment" => Ok(ArmorType::RevivalGarment),
            "spiritgarment" => Ok(ArmorType::SpiritGarment),
            "stinkframe" => Ok(ArmorType::StinkFrame),
            "dpartsver101" => Ok(ArmorType::DPartsver101),
            "dpartsver210" => Ok(ArmorType::DPartsver210),
            "parasitewearderol" => Ok(ArmorType::ParasiteWearDeRol),
            "parasitewearnelgal" => Ok(ArmorType::ParasiteWearNelgal),
            "parasitewearvajulla" => Ok(ArmorType::ParasiteWearVajulla),
            "senseplate" => Ok(ArmorType::SensePlate),
            "gravitonplate" => Ok(ArmorType::GravitonPlate),
            "attributeplate" => Ok(ArmorType::AttributePlate),
            "flowensframe" => Ok(ArmorType::FlowensFrame),
            "customframeveroo" => Ok(ArmorType::CustomFrameverOO),
            "dbsarmor" => Ok(ArmorType::DBsArmor),
            "guardwave" => Ok(ArmorType::GuardWave),
            "dffield" => Ok(ArmorType::DFField),
            "luminousfield" => Ok(ArmorType::LuminousField),
            "chuchufever" => Ok(ArmorType::ChuChuFever),
            "loveheart" => Ok(ArmorType::LoveHeart),
            "flamegarment" => Ok(ArmorType::FlameGarment),
            "virusarmorlafuteria" => Ok(ArmorType::VirusArmorLafuteria),
            "brightnesscircle" => Ok(ArmorType::BrightnessCircle),
            "aurafield" => Ok(ArmorType::AuraField),
            "electroframe" => Ok(ArmorType::ElectroFrame),
            "sacredcloth" => Ok(ArmorType::SacredCloth),
            "smokingplate" => Ok(ArmorType::SmokingPlate),
            _ => Err(ItemParseError::UnknownItem(String::from(value)))
        }
    }
}

impl ArmorType {
    fn as_value(&self) -> u32 {
        match self {
            ArmorType::Frame => 0x010100,
            ArmorType::Armor => 0x010101,
            ArmorType::PsyArmor => 0x010102,
            ArmorType::GigaFrame => 0x010103,
            ArmorType::SoulFrame => 0x010104,
            ArmorType::CrossArmor => 0x010105,
            ArmorType::SolidFrame => 0x010106,
            ArmorType::BraveArmor => 0x010107,
            ArmorType::HyperFrame => 0x010108,
            ArmorType::GrandArmor => 0x010109,
            ArmorType::ShockFrame => 0x01010A,
            ArmorType::KingsFrame => 0x01010B,
            ArmorType::DragonFrame => 0x01010C,
            ArmorType::AbsorbArmor => 0x01010D,
            ArmorType::ProtectFrame => 0x01010E,
            ArmorType::GeneralArmor => 0x01010F,
            ArmorType::PerfectFrame => 0x010110,
            ArmorType::ValiantFrame => 0x010111,
            ArmorType::ImperialArmor => 0x010112,
            ArmorType::HolinessArmor => 0x010113,
            ArmorType::GuardianArmor => 0x010114,
            ArmorType::DivinityArmor => 0x010115,
            ArmorType::UltimateFrame => 0x010116,
            ArmorType::CelestialArmor => 0x010117,
            ArmorType::HunterField => 0x010118,
            ArmorType::RangerField => 0x010119,
            ArmorType::ForceField => 0x01011A,
            ArmorType::RevivalGarment => 0x01011B,
            ArmorType::SpiritGarment => 0x01011C,
            ArmorType::StinkFrame => 0x01011D,
            ArmorType::DPartsver101 => 0x01011E,
            ArmorType::DPartsver210 => 0x01011F,
            ArmorType::ParasiteWearDeRol => 0x010120,
            ArmorType::ParasiteWearNelgal => 0x010121,
            ArmorType::ParasiteWearVajulla => 0x010122,
            ArmorType::SensePlate => 0x010123,
            ArmorType::GravitonPlate => 0x010124,
            ArmorType::AttributePlate => 0x010125,
            ArmorType::FlowensFrame => 0x010126,
            ArmorType::CustomFrameverOO => 0x010127,
            ArmorType::DBsArmor => 0x010128,
            ArmorType::GuardWave => 0x010129,
            ArmorType::DFField => 0x01012A,
            ArmorType::LuminousField => 0x01012B,
            ArmorType::ChuChuFever => 0x01012C,
            ArmorType::LoveHeart => 0x01012D,
            ArmorType::FlameGarment => 0x01012E,
            ArmorType::VirusArmorLafuteria => 0x01012F,
            ArmorType::BrightnessCircle => 0x010130,
            ArmorType::AuraField => 0x010131,
            ArmorType::ElectroFrame => 0x010132,
            ArmorType::SacredCloth => 0x010133,
            ArmorType::SmokingPlate => 0x010134,
        }
    }
}

#[derive(Debug)]
pub struct Armor {
    pub armor: ArmorType,
    pub dfp: u8,
    pub evp: u8,
    pub slots: u8,
}

impl ItemData for Armor {
    fn row1(&self) -> u32 {
        self.armor.as_value() << 8
    }

    fn row2(&self) -> u32 {
        u32::from_be_bytes([0, self.slots, self.dfp, 0])
    }

    fn row3(&self) -> u32 {
        u32::from_be_bytes([self.evp, 0, 0, 0])
    }

    fn row4(&self) -> u32 {
        0
    }
}

#[derive(Debug)]
pub enum ShieldType {
    Barrier,
    Shield,
    CoreShield,
    GigaShield,
    SoulBarrier,
    HardShield,
    BraveBarrier,
    SolidShield,
    FlameBarrier,
    PlasmaBarrier,
    FreezeBarrier,
    PsychicBarrier,
    GeneralShield,
    ProtectBarrier,
    GloriousShield,
    ImperialBarrier,
    GuardianShield,
    DivinityBarrier,
    UltimateShield,
    SpiritualShield,
    CelestialShield,
    InvisibleGuard,
    SacredGuard,
    SPartsver116,
    SPartsver201,
    LightRelief,
    ShieldofDelsaber,
    ForceWall,
    RangerWall,
    HunterWall,
    AttributeWall,
    SecretGear,
    CombatGear,
    ProtoRegeneGear,
    RegenerateGear,
    RegeneGearAdv,
    FlowensShield,
    CustomBarrierverOO,
    DBsShield,
    RedRing,
    TripolicShield,
    StandstillShield,
    SafetyHeart,
    KasamiBracer,
    GodsShieldSuzaku,
    GodsShieldGenbu,
    GodsShieldByakko,
    GodsShieldSeiryu,
    HuntersShell,
    RicosGlasses,
    RicosEarring,
    BlueRing,
    SecureFeet,
    RestaMerge,
    AntiMerge,
    ShiftaMerge,
    DebandMerge,
    FoieMerge,
    GifoieMerge,
    RafoieMerge,
    RedMerge,
    BartaMerge,
    GibartaMerge,
    RabartaMerge,
    BlueMerge,
    ZondeMerge,
    GizondeMerge,
    RazondeMerge,
    YellowMerge,
    RecoveryBarrier,
    AssistBarrier,
    RedBarrier,
    BlueBarrier,
    YellowBarrier,
}


impl TryFrom<&str> for ShieldType {
    type Error = ItemParseError;
    fn try_from(value: &str) -> Result<ShieldType, Self::Error> {
        match value {
            "barrier" => Ok(ShieldType::Barrier),
            "shield" => Ok(ShieldType::Shield),
            "coreshield" => Ok(ShieldType::CoreShield),
            "gigashield" => Ok(ShieldType::GigaShield),
            "soulbarrier" => Ok(ShieldType::SoulBarrier),
            "hardshield" => Ok(ShieldType::HardShield),
            "bravebarrier" => Ok(ShieldType::BraveBarrier),
            "solidshield" => Ok(ShieldType::SolidShield),
            "flamebarrier" => Ok(ShieldType::FlameBarrier),
            "plasmabarrier" => Ok(ShieldType::PlasmaBarrier),
            "freezebarrier" => Ok(ShieldType::FreezeBarrier),
            "psychicbarrier" => Ok(ShieldType::PsychicBarrier),
            "generalshield" => Ok(ShieldType::GeneralShield),
            "protectbarrier" => Ok(ShieldType::ProtectBarrier),
            "gloriousshield" => Ok(ShieldType::GloriousShield),
            "imperialbarrier" => Ok(ShieldType::ImperialBarrier),
            "guardianshield" => Ok(ShieldType::GuardianShield),
            "divinitybarrier" => Ok(ShieldType::DivinityBarrier),
            "ultimateshield" => Ok(ShieldType::UltimateShield),
            "spiritualshield" => Ok(ShieldType::SpiritualShield),
            "celestialshield" => Ok(ShieldType::CelestialShield),
            "invisibleguard" => Ok(ShieldType::InvisibleGuard),
            "sacredguard" => Ok(ShieldType::SacredGuard),
            "spartsver116" => Ok(ShieldType::SPartsver116),
            "spartsver201" => Ok(ShieldType::SPartsver201),
            "lightrelief" => Ok(ShieldType::LightRelief),
            "shieldofdelsaber" => Ok(ShieldType::ShieldofDelsaber),
            "forcewall" => Ok(ShieldType::ForceWall),
            "rangerwall" => Ok(ShieldType::RangerWall),
            "hunterwall" => Ok(ShieldType::HunterWall),
            "attributewall" => Ok(ShieldType::AttributeWall),
            "secretgear" => Ok(ShieldType::SecretGear),
            "combatgear" => Ok(ShieldType::CombatGear),
            "protoregenegear" => Ok(ShieldType::ProtoRegeneGear),
            "regenerategear" => Ok(ShieldType::RegenerateGear),
            "regenegearadv" => Ok(ShieldType::RegeneGearAdv),
            "flowensshield" => Ok(ShieldType::FlowensShield),
            "custombarrierveroo" => Ok(ShieldType::CustomBarrierverOO),
            "dbsshield" => Ok(ShieldType::DBsShield),
            "redring" => Ok(ShieldType::RedRing),
            "tripolicshield" => Ok(ShieldType::TripolicShield),
            "standstillshield" => Ok(ShieldType::StandstillShield),
            "safetyheart" => Ok(ShieldType::SafetyHeart),
            "kasamibracer" => Ok(ShieldType::KasamiBracer),
            "godsshieldsuzaku" => Ok(ShieldType::GodsShieldSuzaku),
            "godsshieldgenbu" => Ok(ShieldType::GodsShieldGenbu),
            "godsshieldbyakko" => Ok(ShieldType::GodsShieldByakko),
            "godsshieldseiryu" => Ok(ShieldType::GodsShieldSeiryu),
            "huntersshell" => Ok(ShieldType::HuntersShell),
            "ricosglasses" => Ok(ShieldType::RicosGlasses),
            "ricosearring" => Ok(ShieldType::RicosEarring),
            "bluering" => Ok(ShieldType::BlueRing),
            "securefeet" => Ok(ShieldType::SecureFeet),
            "restamerge" => Ok(ShieldType::RestaMerge),
            "antimerge" => Ok(ShieldType::AntiMerge),
            "shiftamerge" => Ok(ShieldType::ShiftaMerge),
            "debandmerge" => Ok(ShieldType::DebandMerge),
            "foiemerge" => Ok(ShieldType::FoieMerge),
            "gifoiemerge" => Ok(ShieldType::GifoieMerge),
            "rafoiemerge" => Ok(ShieldType::RafoieMerge),
            "redmerge" => Ok(ShieldType::RedMerge),
            "bartamerge" => Ok(ShieldType::BartaMerge),
            "gibartamerge" => Ok(ShieldType::GibartaMerge),
            "rabartamerge" => Ok(ShieldType::RabartaMerge),
            "bluemerge" => Ok(ShieldType::BlueMerge),
            "zondemerge" => Ok(ShieldType::ZondeMerge),
            "gizondemerge" => Ok(ShieldType::GizondeMerge),
            "razondemerge" => Ok(ShieldType::RazondeMerge),
            "yellowmerge" => Ok(ShieldType::YellowMerge),
            "recoverybarrier" => Ok(ShieldType::RecoveryBarrier),
            "assistbarrier" => Ok(ShieldType::AssistBarrier),
            "redbarrier" => Ok(ShieldType::RedBarrier),
            "bluebarrier" => Ok(ShieldType::BlueBarrier),
            "yellowbarrier" => Ok(ShieldType::YellowBarrier),
            _ => Err(ItemParseError::UnknownItem(String::from(value)))
        }
    }
}

impl ShieldType {
    fn as_value(&self) -> u32 {
        match self {
            ShieldType::Barrier => 0x010200,
            ShieldType::Shield => 0x010201,
            ShieldType::CoreShield => 0x010202,
            ShieldType::GigaShield => 0x010203,
            ShieldType::SoulBarrier => 0x010204,
            ShieldType::HardShield => 0x010205,
            ShieldType::BraveBarrier => 0x010206,
            ShieldType::SolidShield => 0x010207,
            ShieldType::FlameBarrier => 0x010208,
            ShieldType::PlasmaBarrier => 0x010209,
            ShieldType::FreezeBarrier => 0x01020A,
            ShieldType::PsychicBarrier => 0x01020B,
            ShieldType::GeneralShield => 0x01020C,
            ShieldType::ProtectBarrier => 0x01020D,
            ShieldType::GloriousShield => 0x01020E,
            ShieldType::ImperialBarrier => 0x01020F,
            ShieldType::GuardianShield => 0x010210,
            ShieldType::DivinityBarrier => 0x010211,
            ShieldType::UltimateShield => 0x010212,
            ShieldType::SpiritualShield => 0x010213,
            ShieldType::CelestialShield => 0x010214,
            ShieldType::InvisibleGuard => 0x010215,
            ShieldType::SacredGuard => 0x010216,
            ShieldType::SPartsver116 => 0x010217,
            ShieldType::SPartsver201 => 0x010218,
            ShieldType::LightRelief => 0x010219,
            ShieldType::ShieldofDelsaber => 0x01021A,
            ShieldType::ForceWall => 0x01021B,
            ShieldType::RangerWall => 0x01021C,
            ShieldType::HunterWall => 0x01021D,
            ShieldType::AttributeWall => 0x01021E,
            ShieldType::SecretGear => 0x01021F,
            ShieldType::CombatGear => 0x010220,
            ShieldType::ProtoRegeneGear => 0x010221,
            ShieldType::RegenerateGear => 0x010222,
            ShieldType::RegeneGearAdv => 0x010223,
            ShieldType::FlowensShield => 0x010224,
            ShieldType::CustomBarrierverOO => 0x010225,
            ShieldType::DBsShield => 0x010226,
            ShieldType::RedRing => 0x010227,
            ShieldType::TripolicShield => 0x010228,
            ShieldType::StandstillShield => 0x010229,
            ShieldType::SafetyHeart => 0x01022A,
            ShieldType::KasamiBracer => 0x01022B,
            ShieldType::GodsShieldSuzaku => 0x01022C,
            ShieldType::GodsShieldGenbu => 0x01022D,
            ShieldType::GodsShieldByakko => 0x01022E,
            ShieldType::GodsShieldSeiryu => 0x01022F,
            ShieldType::HuntersShell => 0x010230,
            ShieldType::RicosGlasses => 0x010231,
            ShieldType::RicosEarring => 0x010232,
            ShieldType::BlueRing => 0x010233,
            ShieldType::SecureFeet => 0x010235,
            ShieldType::RestaMerge => 0x01023A,
            ShieldType::AntiMerge => 0x01023B,
            ShieldType::ShiftaMerge => 0x01023C,
            ShieldType::DebandMerge => 0x01023D,
            ShieldType::FoieMerge => 0x01023E,
            ShieldType::GifoieMerge => 0x01023F,
            ShieldType::RafoieMerge => 0x010240,
            ShieldType::RedMerge => 0x010241,
            ShieldType::BartaMerge => 0x010242,
            ShieldType::GibartaMerge => 0x010243,
            ShieldType::RabartaMerge => 0x010244,
            ShieldType::BlueMerge => 0x010245,
            ShieldType::ZondeMerge => 0x010246,
            ShieldType::GizondeMerge => 0x010247,
            ShieldType::RazondeMerge => 0x010248,
            ShieldType::YellowMerge => 0x010249,
            ShieldType::RecoveryBarrier => 0x01024A,
            ShieldType::AssistBarrier => 0x01024B,
            ShieldType::RedBarrier => 0x01024C,
            ShieldType::BlueBarrier => 0x01024D,
            ShieldType::YellowBarrier => 0x01024E,
        }
    }
}

#[derive(Debug)]
pub struct Shield {
    pub shield: ShieldType,
    pub dfp: u8,
    pub evp: u8,
}

impl ItemData for Shield {
    fn row1(&self) -> u32 {
        self.shield.as_value() << 8
    }

    fn row2(&self) -> u32 {
        u32::from_be_bytes([0, 0, self.dfp, 0])
    }

    fn row3(&self) -> u32 {
        u32::from_be_bytes([self.evp, 0, 0, 0])
    }

    fn row4(&self) -> u32 {
        0
    }
}

#[derive(Debug)]
pub enum UnitType {
    KnightPower,
    GeneralPower,
    OgrePower,
    GodPower,
    PriestMind,
    GeneralMind,
    AngelMind,
    GodMind,
    MarksmanArm,
    GeneralArm,
    ElfArm,
    GodArm,
    ThiefLegs,
    GeneralLegs,
    ElfLegs,
    GodLegs,
    DiggerHP,
    GeneralHP,
    DragonHP,
    GodHP,
    MagicianTP,
    GeneralTP,
    AngelTP,
    GodTP,
    WarriorBody,
    GeneralBody,
    MetalBody,
    GodBody,
    AngelLuck,
    GodLuck,
    MasterAbility,
    HeroAbility,
    GodAbility,
    ResistFire,
    ResistFlame,
    ResistBurning,
    ResistCold,
    ResistFreeze,
    ResistBlizzard,
    ResistShock,
    ResistThunder,
    ResistStorm,
    ResistLight,
    ResistSaint,
    ResistHoly,
    ResistDark,
    ResistEvil,
    ResistDevil,
    AllResist,
    SuperResist,
    PerfectResist,
    HPRestorate,
    HPGenerate,
    HPRevival,
    TPRestorate,
    TPGenerate,
    TPRevival,
    PBAmplifier,
    PBGenerate,
    PBCreate,
    WizardTechnique,
    DevilTechnique,
    GodTechnique,
    GeneralBattle,
    DevilBattle,
    GodBattle,
    CurePoison,
    CureParalysis,
    CureSlow,
    CureConfuse,
    CureFreeze,
    CureShock,
}

impl TryFrom<&str> for UnitType {
    type Error = ItemParseError;
    fn try_from(value: &str) -> Result<UnitType, Self::Error> {
        match value {
            "knightpower" => Ok(UnitType::KnightPower),
            "generalpower" => Ok(UnitType::GeneralPower),
            "ogrepower" => Ok(UnitType::OgrePower),
            "godpower" => Ok(UnitType::GodPower),
            "priestmind" => Ok(UnitType::PriestMind),
            "generalmind" => Ok(UnitType::GeneralMind),
            "angelmind" => Ok(UnitType::AngelMind),
            "godmind" => Ok(UnitType::GodMind),
            "marksmanarm" => Ok(UnitType::MarksmanArm),
            "generalarm" => Ok(UnitType::GeneralArm),
            "elfarm" => Ok(UnitType::ElfArm),
            "godarm" => Ok(UnitType::GodArm),
            "thieflegs" => Ok(UnitType::ThiefLegs),
            "generallegs" => Ok(UnitType::GeneralLegs),
            "elflegs" => Ok(UnitType::ElfLegs),
            "godlegs" => Ok(UnitType::GodLegs),
            "diggerhp" => Ok(UnitType::DiggerHP),
            "generalhp" => Ok(UnitType::GeneralHP),
            "dragonhp" => Ok(UnitType::DragonHP),
            "godhp" => Ok(UnitType::GodHP),
            "magiciantp" => Ok(UnitType::MagicianTP),
            "generaltp" => Ok(UnitType::GeneralTP),
            "angeltp" => Ok(UnitType::AngelTP),
            "godtp" => Ok(UnitType::GodTP),
            "warriorbody" => Ok(UnitType::WarriorBody),
            "generalbody" => Ok(UnitType::GeneralBody),
            "metalbody" => Ok(UnitType::MetalBody),
            "godbody" => Ok(UnitType::GodBody),
            "angelluck" => Ok(UnitType::AngelLuck),
            "godluck" => Ok(UnitType::GodLuck),
            "masterability" => Ok(UnitType::MasterAbility),
            "heroability" => Ok(UnitType::HeroAbility),
            "godability" => Ok(UnitType::GodAbility),
            "resistfire" => Ok(UnitType::ResistFire),
            "resistflame" => Ok(UnitType::ResistFlame),
            "resistburning" => Ok(UnitType::ResistBurning),
            "resistcold" => Ok(UnitType::ResistCold),
            "resistfreeze" => Ok(UnitType::ResistFreeze),
            "resistblizzard" => Ok(UnitType::ResistBlizzard),
            "resistshock" => Ok(UnitType::ResistShock),
            "resistthunder" => Ok(UnitType::ResistThunder),
            "resiststorm" => Ok(UnitType::ResistStorm),
            "resistlight" => Ok(UnitType::ResistLight),
            "resistsaint" => Ok(UnitType::ResistSaint),
            "resistholy" => Ok(UnitType::ResistHoly),
            "resistdark" => Ok(UnitType::ResistDark),
            "resistevil" => Ok(UnitType::ResistEvil),
            "resistdevil" => Ok(UnitType::ResistDevil),
            "allresist" => Ok(UnitType::AllResist),
            "superresist" => Ok(UnitType::SuperResist),
            "perfectresist" => Ok(UnitType::PerfectResist),
            "hprestorate" => Ok(UnitType::HPRestorate),
            "hpgenerate" => Ok(UnitType::HPGenerate),
            "hprevival" => Ok(UnitType::HPRevival),
            "tprestorate" => Ok(UnitType::TPRestorate),
            "tpgenerate" => Ok(UnitType::TPGenerate),
            "tprevival" => Ok(UnitType::TPRevival),
            "pbamplifier" => Ok(UnitType::PBAmplifier),
            "pbgenerate" => Ok(UnitType::PBGenerate),
            "pbcreate" => Ok(UnitType::PBCreate),
            "wizardtechnique" => Ok(UnitType::WizardTechnique),
            "deviltechnique" => Ok(UnitType::DevilTechnique),
            "godtechnique" => Ok(UnitType::GodTechnique),
            "generalbattle" => Ok(UnitType::GeneralBattle),
            "devilbattle" => Ok(UnitType::DevilBattle),
            "godbattle" => Ok(UnitType::GodBattle),
            "curepoison" => Ok(UnitType::CurePoison),
            "cureparalysis" => Ok(UnitType::CureParalysis),
            "cureslow" => Ok(UnitType::CureSlow),
            "cureconfuse" => Ok(UnitType::CureConfuse),
            "curefreeze" => Ok(UnitType::CureFreeze),
            "cureshock" => Ok(UnitType::CureShock),
            _ => Err(ItemParseError::UnknownItem(String::from(value)))
        }
    }
}

impl UnitType {
    fn as_value(&self) -> u32 {
        match self {
            UnitType::KnightPower => 0x010300,
            UnitType::GeneralPower => 0x010301,
            UnitType::OgrePower => 0x010302,
            UnitType::GodPower => 0x010303,
            UnitType::PriestMind => 0x010304,
            UnitType::GeneralMind => 0x010305,
            UnitType::AngelMind => 0x010306,
            UnitType::GodMind => 0x010307,
            UnitType::MarksmanArm => 0x010308,
            UnitType::GeneralArm => 0x010309,
            UnitType::ElfArm => 0x01030A,
            UnitType::GodArm => 0x01030B,
            UnitType::ThiefLegs => 0x01030C,
            UnitType::GeneralLegs => 0x01030D,
            UnitType::ElfLegs => 0x01030E,
            UnitType::GodLegs => 0x01030F,
            UnitType::DiggerHP => 0x010310,
            UnitType::GeneralHP => 0x010311,
            UnitType::DragonHP => 0x010312,
            UnitType::GodHP => 0x010313,
            UnitType::MagicianTP => 0x010314,
            UnitType::GeneralTP => 0x010315,
            UnitType::AngelTP => 0x010316,
            UnitType::GodTP => 0x010317,
            UnitType::WarriorBody => 0x010318,
            UnitType::GeneralBody => 0x010319,
            UnitType::MetalBody => 0x01031A,
            UnitType::GodBody => 0x01031B,
            UnitType::AngelLuck => 0x01031C,
            UnitType::GodLuck => 0x01031D,
            UnitType::MasterAbility => 0x01031E,
            UnitType::HeroAbility => 0x01031F,
            UnitType::GodAbility => 0x010320,
            UnitType::ResistFire => 0x010321,
            UnitType::ResistFlame => 0x010322,
            UnitType::ResistBurning => 0x010323,
            UnitType::ResistCold => 0x010324,
            UnitType::ResistFreeze => 0x010325,
            UnitType::ResistBlizzard => 0x010326,
            UnitType::ResistShock => 0x010327,
            UnitType::ResistThunder => 0x010328,
            UnitType::ResistStorm => 0x010329,
            UnitType::ResistLight => 0x01032A,
            UnitType::ResistSaint => 0x01032B,
            UnitType::ResistHoly => 0x01032C,
            UnitType::ResistDark => 0x01032D,
            UnitType::ResistEvil => 0x01032E,
            UnitType::ResistDevil => 0x01032F,
            UnitType::AllResist => 0x010330,
            UnitType::SuperResist => 0x010331,
            UnitType::PerfectResist => 0x010332,
            UnitType::HPRestorate => 0x010333,
            UnitType::HPGenerate => 0x010334,
            UnitType::HPRevival => 0x010335,
            UnitType::TPRestorate => 0x010336,
            UnitType::TPGenerate => 0x010337,
            UnitType::TPRevival => 0x010338,
            UnitType::PBAmplifier => 0x010339,
            UnitType::PBGenerate => 0x01033A,
            UnitType::PBCreate => 0x01033B,
            UnitType::WizardTechnique => 0x01033C,
            UnitType::DevilTechnique => 0x01033D,
            UnitType::GodTechnique => 0x01033E,
            UnitType::GeneralBattle => 0x01033F,
            UnitType::DevilBattle => 0x010340,
            UnitType::GodBattle => 0x010341,
            UnitType::CurePoison => 0x010342,
            UnitType::CureParalysis => 0x010343,
            UnitType::CureSlow => 0x010344,
            UnitType::CureConfuse => 0x010345,
            UnitType::CureFreeze => 0x010346,
            UnitType::CureShock => 0x010347,
        }
    }
}



#[derive(Debug)]
pub enum UnitModifier {
    PlusPlus,
    Plus,
    Minus,
    MinusMinus,
}

#[derive(Debug)]
pub struct Unit {
    pub unit: UnitType,
    pub umod: Option<UnitModifier>,
}

impl ItemData for Unit {
    fn row1(&self) -> u32 {
        self.unit.as_value() << 8
    }

    fn row2(&self) -> u32 {
        u32::from_be_bytes(
            [0,0,
             self.umod.as_ref().map(|umod| {
                 match umod {
                     UnitModifier::PlusPlus => 3,
                     UnitModifier::Plus => 1,
                     UnitModifier::Minus => 0xFF,
                     UnitModifier::MinusMinus => 0xFE,
                 }
             }).unwrap_or(0),
             self.umod.as_ref().map(|umod| {
                 match umod {
                     UnitModifier::PlusPlus => 0,
                     UnitModifier::Plus => 0,
                     UnitModifier::Minus => 0xFF,
                     UnitModifier::MinusMinus => 0xFF,
                 }
             }).unwrap_or(0)
            ]
        )
    }

    fn row3(&self) -> u32 {
        0
    }

    fn row4(&self) -> u32 {
        0
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ToolType {
    Monomate,
    Dimate,
    Trimate,
    Monofluid,
    Difluid,
    Trifluid,
    SolAtomizer,
    MoonAtomizer,
    StarAtomizer,
    Antidote,
    Antiparalysis,
    Telepipe,
    TrapVision,
    ScapeDoll,
    Monogrinder,
    Digrinder,
    Trigrinder,
    PowerMaterial,
    MindMaterial,
    EvadeMaterial,
    HPMaterial,
    TPMaterial,
    DefMaterial,
    LuckMaterial,
    CellofMag502,
    CellofMag213,
    PartsofRoboChao,
    HeartofOpaOpa,
    HeartofPian,
    HeartofChao,
    SorcerersRightArm,
    SbeatsArms,
    ParmsArms,
    DelsabersRightArm,
    BringersRightArm,
    DelsabersLeftArm,
    SredsArms,
    DragonsClaw,
    HildebearsHead,
    HildebluesHead,
    PartsofBaranz,
    BelrasRightArm,
    GiGuesbody,
    SinowBerillsArms,
    GrassAssassinsArms,
    BoomasRightArm,
    GoboomasRightArm,
    GigoboomasRightArm,
    GalGryphonsWing,
    RappysWing,
    CladdingofEpsilon,
    DeRolLeShell,
    BerillPhoton,
    ParasiticgeneFlow,
    MagicStoneIritista,
    Blueblackstone,
    Syncesta,
    MagicWater,
    ParasiticcellTypeD,
    MagicrockHeartKey,
    MagicrockMoola,
    StarAmplifier,
    BookofHitogata,
    HeartofChuChu,
    PartsofEggBlaster,
    HeartofAngel,
    HeartofDevil,
    KitofHamburger,
    PanthersSpirit,
    KitofMarkIII,
    KitofMasterSystem,
    KitofGenesis,
    KitofSegaSaturn,
    KitofDreamcast,
    AmplifierofResta,
    AmplifierofAnti,
    AmplifierofShifta,
    AmplifierofDeband,
    AmplifierofFoie,
    AmplifierofGifoie,
    AmplifierofRafoie,
    AmplifierofBarta,
    AmplifierofGibarta,
    AmplifierofRabarta,
    AmplifierofZonde,
    AmplifierofGizonde,
    AmplifierofRazonde,
    AmplifierofRed,
    AmplifierofBlue,
    AmplifierofYellow,
    HeartofKapuKapu,
    PhotonBooster,
    AddSlot,
    PhotonDrop,
    PhotonSphere,
    PhotonCrystal,
    ChristmasPresent,
    EasterEgg,
    JackOLantern,
    HuntersReportA,
    HuntersReportB,
    HuntersReportC,
    HuntersReportD,
    HuntersReportF,
}

impl TryFrom<&str> for ToolType {
    type Error = ItemParseError;
    fn try_from(value: &str) -> Result<ToolType, ItemParseError> {
        match value {
            "monomate" => Ok(ToolType::Monomate),
            "dimate" => Ok(ToolType::Dimate),
            "trimate" => Ok(ToolType::Trimate),
            "monofluid" => Ok(ToolType::Monofluid),
            "difluid" => Ok(ToolType::Difluid),
            "trifluid" => Ok(ToolType::Trifluid),
            "solatomizer" => Ok(ToolType::SolAtomizer),
            "moonatomizer" => Ok(ToolType::MoonAtomizer),
            "staratomizer" => Ok(ToolType::StarAtomizer),
            "antidote" => Ok(ToolType::Antidote),
            "antiparalysis" => Ok(ToolType::Antiparalysis),
            "telepipe" => Ok(ToolType::Telepipe),
            "trapvision" => Ok(ToolType::TrapVision),
            "scapedoll" => Ok(ToolType::ScapeDoll),
            "monogrinder" => Ok(ToolType::Monogrinder),
            "digrinder" => Ok(ToolType::Digrinder),
            "trigrinder" => Ok(ToolType::Trigrinder),
            "powermaterial" => Ok(ToolType::PowerMaterial),
            "mindmaterial" => Ok(ToolType::MindMaterial),
            "evadematerial" => Ok(ToolType::EvadeMaterial),
            "hpmaterial" => Ok(ToolType::HPMaterial),
            "tpmaterial" => Ok(ToolType::TPMaterial),
            "defmaterial" => Ok(ToolType::DefMaterial),
            "luckmaterial" => Ok(ToolType::LuckMaterial),
            "cellofmag502" => Ok(ToolType::CellofMag502),
            "cellofmag213" => Ok(ToolType::CellofMag213),
            "partsofrobochao" => Ok(ToolType::PartsofRoboChao),
            "heartofopaopa" => Ok(ToolType::HeartofOpaOpa),
            "heartofpian" => Ok(ToolType::HeartofPian),
            "heartofchao" => Ok(ToolType::HeartofChao),
            "sorcerersrightarm" => Ok(ToolType::SorcerersRightArm),
            "sbeatsarms" => Ok(ToolType::SbeatsArms),
            "parmsarms" => Ok(ToolType::ParmsArms),
            "delsabersrightarm" => Ok(ToolType::DelsabersRightArm),
            "bringersrightarm" => Ok(ToolType::BringersRightArm),
            "delsabersleftarm" => Ok(ToolType::DelsabersLeftArm),
            "sredsarms" => Ok(ToolType::SredsArms),
            "dragonsclaw" => Ok(ToolType::DragonsClaw),
            "hildebearshead" => Ok(ToolType::HildebearsHead),
            "hildeblueshead" => Ok(ToolType::HildebluesHead),
            "partsofbaranz" => Ok(ToolType::PartsofBaranz),
            "belrasrightarm" => Ok(ToolType::BelrasRightArm),
            "giguesbody" => Ok(ToolType::GiGuesbody),
            "sinowberillsarms" => Ok(ToolType::SinowBerillsArms),
            "grassassassinsarms" => Ok(ToolType::GrassAssassinsArms),
            "boomasrightarm" => Ok(ToolType::BoomasRightArm),
            "goboomasrightarm" => Ok(ToolType::GoboomasRightArm),
            "gigoboomasrightarm" => Ok(ToolType::GigoboomasRightArm),
            "galgryphonswing" => Ok(ToolType::GalGryphonsWing),
            "rappyswing" => Ok(ToolType::RappysWing),
            "claddingofepsilon" => Ok(ToolType::CladdingofEpsilon),
            "derolleshell" => Ok(ToolType::DeRolLeShell),
            "berillphoton" => Ok(ToolType::BerillPhoton),
            "parasiticgeneflow" => Ok(ToolType::ParasiticgeneFlow),
            "magicstoneiritista" => Ok(ToolType::MagicStoneIritista),
            "blueblackstone" => Ok(ToolType::Blueblackstone),
            "syncesta" => Ok(ToolType::Syncesta),
            "magicwater" => Ok(ToolType::MagicWater),
            "parasiticcelltyped" => Ok(ToolType::ParasiticcellTypeD),
            "magicrockheartkey" => Ok(ToolType::MagicrockHeartKey),
            "magicrockmoola" => Ok(ToolType::MagicrockMoola),
            "staramplifier" => Ok(ToolType::StarAmplifier),
            "bookofhitogata" => Ok(ToolType::BookofHitogata),
            "heartofchuchu" => Ok(ToolType::HeartofChuChu),
            "partsofeggblaster" => Ok(ToolType::PartsofEggBlaster),
            "heartofangel" => Ok(ToolType::HeartofAngel),
            "heartofdevil" => Ok(ToolType::HeartofDevil),
            "kitofhamburger" => Ok(ToolType::KitofHamburger),
            "panthersspirit" => Ok(ToolType::PanthersSpirit),
            "kitofmarkiii" => Ok(ToolType::KitofMarkIII),
            "kitofmastersystem" => Ok(ToolType::KitofMasterSystem),
            "kitofgenesis" => Ok(ToolType::KitofGenesis),
            "kitofsegasaturn" => Ok(ToolType::KitofSegaSaturn),
            "kitofdreamcast" => Ok(ToolType::KitofDreamcast),
            "amplifierofresta" => Ok(ToolType::AmplifierofResta),
            "amplifierofanti" => Ok(ToolType::AmplifierofAnti),
            "amplifierofshifta" => Ok(ToolType::AmplifierofShifta),
            "amplifierofdeband" => Ok(ToolType::AmplifierofDeband),
            "amplifieroffoie" => Ok(ToolType::AmplifierofFoie),
            "amplifierofgifoie" => Ok(ToolType::AmplifierofGifoie),
            "amplifierofrafoie" => Ok(ToolType::AmplifierofRafoie),
            "amplifierofbarta" => Ok(ToolType::AmplifierofBarta),
            "amplifierofgibarta" => Ok(ToolType::AmplifierofGibarta),
            "amplifierofrabarta" => Ok(ToolType::AmplifierofRabarta),
            "amplifierofzonde" => Ok(ToolType::AmplifierofZonde),
            "amplifierofgizonde" => Ok(ToolType::AmplifierofGizonde),
            "amplifierofrazonde" => Ok(ToolType::AmplifierofRazonde),
            "amplifierofred" => Ok(ToolType::AmplifierofRed),
            "amplifierofblue" => Ok(ToolType::AmplifierofBlue),
            "amplifierofyellow" => Ok(ToolType::AmplifierofYellow),
            "heartofkapukapu" => Ok(ToolType::HeartofKapuKapu),
            "photonbooster" => Ok(ToolType::PhotonBooster),
            "addslot" => Ok(ToolType::AddSlot),
            "photondrop" => Ok(ToolType::PhotonDrop),
            "photonsphere" => Ok(ToolType::PhotonSphere),
            "photoncrystal" => Ok(ToolType::PhotonCrystal),
            "christmaspresent" => Ok(ToolType::ChristmasPresent),
            "easteregg" => Ok(ToolType::EasterEgg),
            "jackolantern" => Ok(ToolType::JackOLantern),
            "huntersreporta" => Ok(ToolType::HuntersReportA),
            "huntersreportb" => Ok(ToolType::HuntersReportB),
            "huntersreportc" => Ok(ToolType::HuntersReportC),
            "huntersreportd" => Ok(ToolType::HuntersReportD),
            "huntersreportf" => Ok(ToolType::HuntersReportF),
            _ => Err(ItemParseError::UnknownItem(String::from(value)))
        }
    }
}

impl TryFrom<u32> for ToolType {
    type Error = ItemParseError;
    fn try_from(value: u32) -> Result<ToolType, ItemParseError> {
        match value {
            0x030000 => Ok(ToolType::Monomate),
            0x030001 => Ok(ToolType::Dimate),
            0x030002 => Ok(ToolType::Trimate),
            0x030100 => Ok(ToolType::Monofluid),
            0x030101 => Ok(ToolType::Difluid),
            0x030102 => Ok(ToolType::Trifluid),
            0x030300 => Ok(ToolType::SolAtomizer),
            0x030400 => Ok(ToolType::MoonAtomizer),
            0x030500 => Ok(ToolType::StarAtomizer),
            0x030600 => Ok(ToolType::Antidote),
            0x030601 => Ok(ToolType::Antiparalysis),
            0x030700 => Ok(ToolType::Telepipe),
            0x030800 => Ok(ToolType::TrapVision),
            0x030900 => Ok(ToolType::ScapeDoll),
            0x030A00 => Ok(ToolType::Monogrinder),
            0x030A01 => Ok(ToolType::Digrinder),
            0x030A02 => Ok(ToolType::Trigrinder),
            0x030B00 => Ok(ToolType::PowerMaterial),
            0x030B01 => Ok(ToolType::MindMaterial),
            0x030B02 => Ok(ToolType::EvadeMaterial),
            0x030B03 => Ok(ToolType::HPMaterial),
            0x030B04 => Ok(ToolType::TPMaterial),
            0x030B05 => Ok(ToolType::DefMaterial),
            0x030B06 => Ok(ToolType::LuckMaterial),
            0x030C00 => Ok(ToolType::CellofMag502),
            0x030C01 => Ok(ToolType::CellofMag213),
            0x030C02 => Ok(ToolType::PartsofRoboChao),
            0x030C03 => Ok(ToolType::HeartofOpaOpa),
            0x030C04 => Ok(ToolType::HeartofPian),
            0x030C05 => Ok(ToolType::HeartofChao),
            0x030D00 => Ok(ToolType::SorcerersRightArm),
            0x030D01 => Ok(ToolType::SbeatsArms),
            0x030D02 => Ok(ToolType::ParmsArms),
            0x030D03 => Ok(ToolType::DelsabersRightArm),
            0x030D04 => Ok(ToolType::BringersRightArm),
            0x030D05 => Ok(ToolType::DelsabersLeftArm),
            0x030D06 => Ok(ToolType::SredsArms),
            0x030D07 => Ok(ToolType::DragonsClaw),
            0x030D08 => Ok(ToolType::HildebearsHead),
            0x030D09 => Ok(ToolType::HildebluesHead),
            0x030D0A => Ok(ToolType::PartsofBaranz),
            0x030D0B => Ok(ToolType::BelrasRightArm),
            0x030D0C => Ok(ToolType::GiGuesbody),
            0x030D0D => Ok(ToolType::SinowBerillsArms),
            0x030D0E => Ok(ToolType::GrassAssassinsArms),
            0x030D0F => Ok(ToolType::BoomasRightArm),
            0x030D10 => Ok(ToolType::GoboomasRightArm),
            0x030D11 => Ok(ToolType::GigoboomasRightArm),
            0x030D12 => Ok(ToolType::GalGryphonsWing),
            0x030D13 => Ok(ToolType::RappysWing),
            0x030D14 => Ok(ToolType::CladdingofEpsilon),
            0x030D15 => Ok(ToolType::DeRolLeShell),
            0x030E00 => Ok(ToolType::BerillPhoton),
            0x030E01 => Ok(ToolType::ParasiticgeneFlow),
            0x030E02 => Ok(ToolType::MagicStoneIritista),
            0x030E03 => Ok(ToolType::Blueblackstone),
            0x030E04 => Ok(ToolType::Syncesta),
            0x030E05 => Ok(ToolType::MagicWater),
            0x030E06 => Ok(ToolType::ParasiticcellTypeD),
            0x030E07 => Ok(ToolType::MagicrockHeartKey),
            0x030E08 => Ok(ToolType::MagicrockMoola),
            0x030E09 => Ok(ToolType::StarAmplifier),
            0x030E0A => Ok(ToolType::BookofHitogata),
            0x030E0B => Ok(ToolType::HeartofChuChu),
            0x030E0C => Ok(ToolType::PartsofEggBlaster),
            0x030E0D => Ok(ToolType::HeartofAngel),
            0x030E0E => Ok(ToolType::HeartofDevil),
            0x030E0F => Ok(ToolType::KitofHamburger),
            0x030E10 => Ok(ToolType::PanthersSpirit),
            0x030E11 => Ok(ToolType::KitofMarkIII),
            0x030E12 => Ok(ToolType::KitofMasterSystem),
            0x030E13 => Ok(ToolType::KitofGenesis),
            0x030E14 => Ok(ToolType::KitofSegaSaturn),
            0x030E15 => Ok(ToolType::KitofDreamcast),
            0x030E16 => Ok(ToolType::AmplifierofResta),
            0x030E17 => Ok(ToolType::AmplifierofAnti),
            0x030E18 => Ok(ToolType::AmplifierofShifta),
            0x030E19 => Ok(ToolType::AmplifierofDeband),
            0x030E1A => Ok(ToolType::AmplifierofFoie),
            0x030E1B => Ok(ToolType::AmplifierofGifoie),
            0x030E1C => Ok(ToolType::AmplifierofRafoie),
            0x030E1D => Ok(ToolType::AmplifierofBarta),
            0x030E1E => Ok(ToolType::AmplifierofGibarta),
            0x030E1F => Ok(ToolType::AmplifierofRabarta),
            0x030E20 => Ok(ToolType::AmplifierofZonde),
            0x030E21 => Ok(ToolType::AmplifierofGizonde),
            0x030E22 => Ok(ToolType::AmplifierofRazonde),
            0x030E23 => Ok(ToolType::AmplifierofRed),
            0x030E24 => Ok(ToolType::AmplifierofBlue),
            0x030E25 => Ok(ToolType::AmplifierofYellow),
            0x030E26 => Ok(ToolType::HeartofKapuKapu),
            0x030E27 => Ok(ToolType::PhotonBooster),
            0x030F00 => Ok(ToolType::AddSlot),
            0x031000 => Ok(ToolType::PhotonDrop),
            0x031001 => Ok(ToolType::PhotonSphere),
            0x031002 => Ok(ToolType::PhotonCrystal),
            0x031500 => Ok(ToolType::ChristmasPresent),
            0x031501 => Ok(ToolType::EasterEgg),
            0x031502 => Ok(ToolType::JackOLantern),
            0x031700 => Ok(ToolType::HuntersReportA),
            0x031701 => Ok(ToolType::HuntersReportB),
            0x031702 => Ok(ToolType::HuntersReportC),
            0x031703 => Ok(ToolType::HuntersReportD),
            0x031704 => Ok(ToolType::HuntersReportF),
            _ => Err(ItemParseError::UnknownValue(value))
        }
    }
}

impl ToolType {
    fn as_value(&self) -> u32 {
        match self {
            ToolType::Monomate => 0x030000,
            ToolType::Dimate => 0x030001,
            ToolType::Trimate => 0x030002,
            ToolType::Monofluid => 0x030100,
            ToolType::Difluid => 0x030101,
            ToolType::Trifluid => 0x030102,
            ToolType::SolAtomizer => 0x030300,
            ToolType::MoonAtomizer => 0x030400,
            ToolType::StarAtomizer => 0x030500,
            ToolType::Antidote => 0x030600,
            ToolType::Antiparalysis => 0x030601,
            ToolType::Telepipe => 0x030700,
            ToolType::TrapVision => 0x030800,
            ToolType::ScapeDoll => 0x030900,
            ToolType::Monogrinder => 0x030A00,
            ToolType::Digrinder => 0x030A01,
            ToolType::Trigrinder => 0x030A02,
            ToolType::PowerMaterial => 0x030B00,
            ToolType::MindMaterial => 0x030B01,
            ToolType::EvadeMaterial => 0x030B02,
            ToolType::HPMaterial => 0x030B03,
            ToolType::TPMaterial => 0x030B04,
            ToolType::DefMaterial => 0x030B05,
            ToolType::LuckMaterial => 0x030B06,
            ToolType::CellofMag502 => 0x030C00,
            ToolType::CellofMag213 => 0x030C01,
            ToolType::PartsofRoboChao => 0x030C02,
            ToolType::HeartofOpaOpa => 0x030C03,
            ToolType::HeartofPian => 0x030C04,
            ToolType::HeartofChao => 0x030C05,
            ToolType::SorcerersRightArm => 0x030D00,
            ToolType::SbeatsArms => 0x030D01,
            ToolType::ParmsArms => 0x030D02,
            ToolType::DelsabersRightArm => 0x030D03,
            ToolType::BringersRightArm => 0x030D04,
            ToolType::DelsabersLeftArm => 0x030D05,
            ToolType::SredsArms => 0x030D06,
            ToolType::DragonsClaw => 0x030D07,
            ToolType::HildebearsHead => 0x030D08,
            ToolType::HildebluesHead => 0x030D09,
            ToolType::PartsofBaranz => 0x030D0A,
            ToolType::BelrasRightArm => 0x030D0B,
            ToolType::GiGuesbody => 0x030D0C,
            ToolType::SinowBerillsArms => 0x030D0D,
            ToolType::GrassAssassinsArms => 0x030D0E,
            ToolType::BoomasRightArm => 0x030D0F,
            ToolType::GoboomasRightArm => 0x030D10,
            ToolType::GigoboomasRightArm => 0x030D11,
            ToolType::GalGryphonsWing => 0x030D12,
            ToolType::RappysWing => 0x030D13,
            ToolType::CladdingofEpsilon => 0x030D14,
            ToolType::DeRolLeShell => 0x030D15,
            ToolType::BerillPhoton => 0x030E00,
            ToolType::ParasiticgeneFlow => 0x030E01,
            ToolType::MagicStoneIritista => 0x030E02,
            ToolType::Blueblackstone => 0x030E03,
            ToolType::Syncesta => 0x030E04,
            ToolType::MagicWater => 0x030E05,
            ToolType::ParasiticcellTypeD => 0x030E06,
            ToolType::MagicrockHeartKey => 0x030E07,
            ToolType::MagicrockMoola => 0x030E08,
            ToolType::StarAmplifier => 0x030E09,
            ToolType::BookofHitogata => 0x030E0A,
            ToolType::HeartofChuChu => 0x030E0B,
            ToolType::PartsofEggBlaster => 0x030E0C,
            ToolType::HeartofAngel => 0x030E0D,
            ToolType::HeartofDevil => 0x030E0E,
            ToolType::KitofHamburger => 0x030E0F,
            ToolType::PanthersSpirit => 0x030E10,
            ToolType::KitofMarkIII => 0x030E11,
            ToolType::KitofMasterSystem => 0x030E12,
            ToolType::KitofGenesis => 0x030E13,
            ToolType::KitofSegaSaturn => 0x030E14,
            ToolType::KitofDreamcast => 0x030E15,
            ToolType::AmplifierofResta => 0x030E16,
            ToolType::AmplifierofAnti => 0x030E17,
            ToolType::AmplifierofShifta => 0x030E18,
            ToolType::AmplifierofDeband => 0x030E19,
            ToolType::AmplifierofFoie => 0x030E1A,
            ToolType::AmplifierofGifoie => 0x030E1B,
            ToolType::AmplifierofRafoie => 0x030E1C,
            ToolType::AmplifierofBarta => 0x030E1D,
            ToolType::AmplifierofGibarta => 0x030E1E,
            ToolType::AmplifierofRabarta => 0x030E1F,
            ToolType::AmplifierofZonde => 0x030E20,
            ToolType::AmplifierofGizonde => 0x030E21,
            ToolType::AmplifierofRazonde => 0x030E22,
            ToolType::AmplifierofRed => 0x030E23,
            ToolType::AmplifierofBlue => 0x030E24,
            ToolType::AmplifierofYellow => 0x030E25,
            ToolType::HeartofKapuKapu => 0x030E26,
            ToolType::PhotonBooster => 0x030E27,
            ToolType::AddSlot => 0x030F00,
            ToolType::PhotonDrop => 0x031000,
            ToolType::PhotonSphere => 0x031001,
            ToolType::PhotonCrystal => 0x031002,
            ToolType::ChristmasPresent => 0x031500,
            ToolType::EasterEgg => 0x031501,
            ToolType::JackOLantern => 0x031502,
            ToolType::HuntersReportA => 0x031700,
            ToolType::HuntersReportB => 0x031701,
            ToolType::HuntersReportC => 0x031702,
            ToolType::HuntersReportD => 0x031703,
            ToolType::HuntersReportF => 0x031704,
        }
    }

    pub fn max_stack(&self) -> u8 {
        match self {
            ToolType::Monomate => 10,
            ToolType::Dimate => 10,
            ToolType::Trimate => 10,
            ToolType::Monofluid => 10,
            ToolType::Difluid => 10,
            ToolType::Trifluid => 10,
            ToolType::SolAtomizer => 10,
            ToolType::MoonAtomizer => 10,
            ToolType::StarAtomizer => 10,
            ToolType::Antidote => 10,
            ToolType::Antiparalysis => 10,
            ToolType::Telepipe => 10,
            ToolType::TrapVision => 10,
            _ => 1,
        }
    }
}

#[derive(Debug)]
pub struct Tool {
    pub tool: ToolType,
    pub stack: u8,
}

impl ItemData for Tool {
    fn row1(&self) -> u32 {
        self.tool.as_value() << 8
    }

    fn row2(&self) -> u32 {
        (self.stack as u32) << 16
    }

    fn row3(&self) -> u32 {
        0
    }

    fn row4(&self) -> u32 {
        0
    }
}

#[derive(Debug)]
pub enum TechType {
    Foie,
    Gifoie,
    Rafoie,
    Barta,
    Gibarta,
    Rabarta,
    Zonde,
    Gizonde,
    Razonde,
    Grants,
    Deband,
    Jellen,
    Zalure,
    Shifta,
    Ryuker,
    Resta,
    Anti,
    Reverser,
    Megid,
}

impl TryFrom<&str> for TechType {
    type Error = ItemParseError;
    fn try_from(tech: &str) -> Result<TechType, ItemParseError> {
        match tech {
            "foie" => Ok(TechType::Foie),
            "gifoie" => Ok(TechType::Gifoie),
            "rafoie" => Ok(TechType::Rafoie),
            "barta" => Ok(TechType::Barta),
            "gibarta" => Ok(TechType::Gibarta),
            "rabarta" => Ok(TechType::Rabarta),
            "zonde" => Ok(TechType::Zonde),
            "gizonde" => Ok(TechType::Gizonde),
            "razonde" => Ok(TechType::Razonde),
            "grants" => Ok(TechType::Grants),
            "deband" => Ok(TechType::Deband),
            "jellen" => Ok(TechType::Jellen),
            "zalure" => Ok(TechType::Zalure),
            "shifta" => Ok(TechType::Shifta),
            "ryuker" => Ok(TechType::Ryuker),
            "resta" => Ok(TechType::Resta),
            "anti" => Ok(TechType::Anti),
            "reverser" => Ok(TechType::Reverser),
            "megid" => Ok(TechType::Megid),
            _ => Err(ItemParseError::UnknownTech(String::from(tech)))
        }
    }
}

impl TechType {
    fn as_value(&self) -> u8 {
        match self {
            TechType::Foie => 0x00,
            TechType::Gifoie => 0x01,
            TechType::Rafoie => 0x02,
            TechType::Barta => 0x03,
            TechType::Gibarta => 0x04,
            TechType::Rabarta => 0x05,
            TechType::Zonde => 0x06,
            TechType::Gizonde => 0x07,
            TechType::Razonde => 0x08,
            TechType::Grants => 0x09,
            TechType::Deband => 0x0A,
            TechType::Jellen => 0x0B,
            TechType::Zalure => 0x0C,
            TechType::Shifta => 0x0D,
            TechType::Ryuker => 0x0E,
            TechType::Resta => 0x0F,
            TechType::Anti => 0x10,
            TechType::Reverser => 0x11,
            TechType::Megid => 0x12,
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

#[derive(Debug)]
pub enum MagType {
    Mag,
    Varuna,
    Mitra,
    Surya,
    Vayu,
    Varaha,
    Kama,
    Ushasu,
    Apsaras,
    Kumara,
    Kaitabha,
    Tapas,
    Bhirava,
    Kalki,
    Rudra,
    Marutah,
    Yaksa,
    Sita,
    Garuda,
    Nandin,
    Ashvinau,
    Ribhava,
    Soma,
    Ila,
    Durga,
    Vritra,
    Namuci,
    Sumba,
    Naga,
    Pitri,
    Kabanda,
    Ravana,
    Marica,
    Soniti,
    Preta,
    Andhaka,
    Bana,
    Naraka,
    Madhu,
    Churel,
    Robochao,
    OpaOpa,
    Pian,
    Chao,
    ChuChu,
    KapuKapu,
    AngelsWing,
    DevilsWing,
    Elenor,
    MarkIII,
    MasterSystem,
    Genesis,
    SegaSaturn,
    Dreamcast,
    Hamburger,
    PanzersTail,
    DevilsTail,
    Deva,
    Rati,
    Savitri,
    Rukmin,
    Pushan,
    Diwari,
    Sato,
    Bhima,
    Nidra,
}

impl TryFrom<&str> for MagType {
    type Error = ItemParseError;
    fn try_from(value: &str) -> Result<MagType, ItemParseError> {
        match value {
            "mag" => Ok(MagType::Mag),
            "varuna" => Ok(MagType::Varuna),
            "mitra" => Ok(MagType::Mitra),
            "surya" => Ok(MagType::Surya),
            "vayu" => Ok(MagType::Vayu),
            "varaha" => Ok(MagType::Varaha),
            "kama" => Ok(MagType::Kama),
            "ushasu" => Ok(MagType::Ushasu),
            "apsaras" => Ok(MagType::Apsaras),
            "kumara" => Ok(MagType::Kumara),
            "kaitabha" => Ok(MagType::Kaitabha),
            "tapas" => Ok(MagType::Tapas),
            "bhirava" => Ok(MagType::Bhirava),
            "kalki" => Ok(MagType::Kalki),
            "rudra" => Ok(MagType::Rudra),
            "marutah" => Ok(MagType::Marutah),
            "yaksa" => Ok(MagType::Yaksa),
            "sita" => Ok(MagType::Sita),
            "garuda" => Ok(MagType::Garuda),
            "nandin" => Ok(MagType::Nandin),
            "ashvinau" => Ok(MagType::Ashvinau),
            "ribhava" => Ok(MagType::Ribhava),
            "soma" => Ok(MagType::Soma),
            "ila" => Ok(MagType::Ila),
            "durga" => Ok(MagType::Durga),
            "vritra" => Ok(MagType::Vritra),
            "namuci" => Ok(MagType::Namuci),
            "sumba" => Ok(MagType::Sumba),
            "naga" => Ok(MagType::Naga),
            "pitri" => Ok(MagType::Pitri),
            "kabanda" => Ok(MagType::Kabanda),
            "ravana" => Ok(MagType::Ravana),
            "marica" => Ok(MagType::Marica),
            "soniti" => Ok(MagType::Soniti),
            "preta" => Ok(MagType::Preta),
            "andhaka" => Ok(MagType::Andhaka),
            "bana" => Ok(MagType::Bana),
            "naraka" => Ok(MagType::Naraka),
            "madhu" => Ok(MagType::Madhu),
            "churel" => Ok(MagType::Churel),
            "robochao" => Ok(MagType::Robochao),
            "opaopa" => Ok(MagType::OpaOpa),
            "pian" => Ok(MagType::Pian),
            "chao" => Ok(MagType::Chao),
            "chuchu" => Ok(MagType::ChuChu),
            "kapukapu" => Ok(MagType::KapuKapu),
            "angelswing" => Ok(MagType::AngelsWing),
            "devilswing" => Ok(MagType::DevilsWing),
            "elenor" => Ok(MagType::Elenor),
            "markiii" => Ok(MagType::MarkIII),
            "mastersystem" => Ok(MagType::MasterSystem),
            "genesis" => Ok(MagType::Genesis),
            "segasaturn" => Ok(MagType::SegaSaturn),
            "dreamcast" => Ok(MagType::Dreamcast),
            "hamburger" => Ok(MagType::Hamburger),
            "panzerstail" => Ok(MagType::PanzersTail),
            "devilstail" => Ok(MagType::DevilsTail),
            "deva" => Ok(MagType::Deva),
            "rati" => Ok(MagType::Rati),
            "savitri" => Ok(MagType::Savitri),
            "rukmin" => Ok(MagType::Rukmin),
            "pushan" => Ok(MagType::Pushan),
            "diwari" => Ok(MagType::Diwari),
            "sato" => Ok(MagType::Sato),
            "bhima" => Ok(MagType::Bhima),
            "nidra" => Ok(MagType::Nidra),
            _ => Err(ItemParseError::UnknownItem(String::from(value)))
        }
    }
}

impl MagType {
    fn as_value(&self) -> u32 {
        match self {
            MagType::Mag => 0x020000,
            MagType::Varuna => 0x020100,
            MagType::Mitra => 0x020200,
            MagType::Surya => 0x020300,
            MagType::Vayu => 0x020400,
            MagType::Varaha => 0x020500,
            MagType::Kama => 0x020600,
            MagType::Ushasu => 0x020700,
            MagType::Apsaras => 0x020800,
            MagType::Kumara => 0x020900,
            MagType::Kaitabha => 0x020A00,
            MagType::Tapas => 0x020B00,
            MagType::Bhirava => 0x020C00,
            MagType::Kalki => 0x020D00,
            MagType::Rudra => 0x020E00,
            MagType::Marutah => 0x020F00,
            MagType::Yaksa => 0x021000,
            MagType::Sita => 0x021100,
            MagType::Garuda => 0x021200,
            MagType::Nandin => 0x021300,
            MagType::Ashvinau => 0x021400,
            MagType::Ribhava => 0x021500,
            MagType::Soma => 0x021600,
            MagType::Ila => 0x021700,
            MagType::Durga => 0x021800,
            MagType::Vritra => 0x021900,
            MagType::Namuci => 0x021A00,
            MagType::Sumba => 0x021B00,
            MagType::Naga => 0x021C00,
            MagType::Pitri => 0x021D00,
            MagType::Kabanda => 0x021E00,
            MagType::Ravana => 0x021F00,
            MagType::Marica => 0x022000,
            MagType::Soniti => 0x022100,
            MagType::Preta => 0x022200,
            MagType::Andhaka => 0x022300,
            MagType::Bana => 0x022400,
            MagType::Naraka => 0x022500,
            MagType::Madhu => 0x022600,
            MagType::Churel => 0x022700,
            MagType::Robochao => 0x022800,
            MagType::OpaOpa => 0x022900,
            MagType::Pian => 0x022A00,
            MagType::Chao => 0x022B00,
            MagType::ChuChu => 0x022C00,
            MagType::KapuKapu => 0x022D00,
            MagType::AngelsWing => 0x022E00,
            MagType::DevilsWing => 0x022F00,
            MagType::Elenor => 0x023000,
            MagType::MarkIII => 0x023100,
            MagType::MasterSystem => 0x023200,
            MagType::Genesis => 0x023300,
            MagType::SegaSaturn => 0x023400,
            MagType::Dreamcast => 0x023500,
            MagType::Hamburger => 0x023600,
            MagType::PanzersTail => 0x023700,
            MagType::DevilsTail => 0x023800,
            MagType::Deva => 0x023900,
            MagType::Rati => 0x023A00,
            MagType::Savitri => 0x023B00,
            MagType::Rukmin => 0x023C00,
            MagType::Pushan => 0x023D00,
            MagType::Diwari => 0x023E00,
            MagType::Sato => 0x023F00,
            MagType::Bhima => 0x240000,
            MagType::Nidra => 0x241000,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum PhotonBlast {
    Farlla,
    Estlla,
    Leilla,
    Pilla,
    Golla,
    MyllaYoulla,
}

impl TryFrom<&str> for PhotonBlast {
    type Error = ItemParseError;
    fn try_from(value: &str) -> Result<PhotonBlast, ItemParseError> {
        match value {
            "farlla" | "f" => Ok(PhotonBlast::Farlla),
            "estlla" | "e" => Ok(PhotonBlast::Estlla),
            "leilla" | "l" => Ok(PhotonBlast::Leilla),
            "pilla" | "p" => Ok(PhotonBlast::Pilla),
            "golla" | "g" => Ok(PhotonBlast::Golla),
            "myllayoulla" | "mylla" | "youlla" | "twins" | "t" => Ok(PhotonBlast::MyllaYoulla),
            _ => Err(ItemParseError::UnknownPhotonBlast(String::from(value)))
        }
    }
}

#[derive(Debug)]
pub enum MagColor {
    Null
}

impl MagColor {
    fn as_value(&self) -> u8 {
        match self {
            MagColor::Null => 0,
        }
    }
}

#[derive(Debug)]
pub struct Mag {
    pub mag: MagType,
    pub iq: u8,
    pub sync: u8,
    pub def: u16,
    pub pow: u16,
    pub dex: u16,
    pub mnd: u16,
    pub pbs: [Option<PhotonBlast>; 3],
    pub color: MagColor,
}


impl Mag {
    fn photon_blast_value(&self) -> u32 {
        let mut photon_blast_list = vec![PhotonBlast::Farlla,
                                         PhotonBlast::Estlla,
                                         PhotonBlast::Golla,
                                         PhotonBlast::Pilla,
                                         PhotonBlast::Leilla,
                                         PhotonBlast::MyllaYoulla];
        let mut photon_blast: u32 = 0;

        if let Some(ref pb_mid) = self.pbs[0] {
            match *pb_mid {
                PhotonBlast::Farlla => {},
                PhotonBlast::Estlla =>      photon_blast |= 1,
                PhotonBlast::Golla =>       photon_blast |= 2,
                PhotonBlast::Pilla =>       photon_blast |= 3,
                PhotonBlast::Leilla =>      photon_blast |= 4,
                PhotonBlast::MyllaYoulla => photon_blast |= 5,
            }

            photon_blast_list.retain(|k| k != pb_mid);
        }
        if let Some(ref pb_right) = self.pbs[1] {
            match *pb_right {
                PhotonBlast::Farlla => {}
                PhotonBlast::Estlla =>      photon_blast |= 1 << 3,
                PhotonBlast::Golla =>       photon_blast |= 2 << 3,
                PhotonBlast::Pilla =>       photon_blast |= 3 << 3,
                PhotonBlast::Leilla =>      photon_blast |= 4 << 3,
                PhotonBlast::MyllaYoulla => photon_blast |= 5 << 3,
            }

            photon_blast_list.retain(|k| k != pb_right);
        }
        if let Some(ref pb_left) = self.pbs[2] {
            if let Some(pos) = photon_blast_list.iter().position(|k| k == pb_left) {
                photon_blast |= (pos as u32) << 6;
            };
        }

        photon_blast
    }

    fn photon_blast_count(&self) -> u8 {
        let mut count = 0;
        for i in 0..3 {
            if let Some(_) = self.pbs[i] {
                count |= 1 << i
            };
        }
        count
    }
}

impl ItemData for Mag {
    fn row1(&self) -> u32 {
        self.mag.as_value() << 8 | self.photon_blast_value()
    }

    fn row2(&self) -> u32 {
        (((self.def * 100).swap_bytes() as u32) << 16) | ((self.pow * 100).swap_bytes() as u32)
    }

    fn row3(&self) -> u32 {
        (((self.dex * 100).swap_bytes() as u32) << 16) | ((self.mnd * 100).swap_bytes() as u32)
    }

    fn row4(&self) -> u32 {
        (self.color.as_value() as u32) << 24 | (self.photon_blast_count() as u32) << 16 | (self.iq as u32) << 8 | (self.sync as u32)
    }
}

#[derive(Debug)]
pub struct Meseta {
    pub amount: u32,
}

impl ItemData for Meseta {
    fn row1(&self) -> u32 {
        u32::from_be_bytes([4, 0, 0, 0])
    }

    fn row2(&self) -> u32 {
        0
    }

    fn row3(&self) -> u32 {
        0
    }

    fn row4(&self) -> u32 {
        u32::from_be_bytes(self.amount.to_le_bytes())
    }
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
                Some(v) => (*v as u32) << ((3-i)*8),
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


#[derive(Debug)]
pub enum Item {
    Weapon(Weapon),
    Armor(Armor),
    Shield(Shield),
    Unit(Unit),
    Mag(Mag),
    Tool(ToolType, u8),
}

impl TryFrom<[&u8; 28]> for Item {
    type Error = ItemParseError;
    fn try_from(value: [&u8; 28]) -> Result<Item, ItemParseError> {
        let item_value = u32::from_be_bytes([0, *value[0], *value[1], *value[2]]);
        println!("item_value! {:X}", item_value);
        if let Ok(tool) = ToolType::try_from(item_value) {
            return Ok(Item::Tool(tool, *value[5]))
        }

        Err(ItemParseError::UnknownValue(item_value))
    }
}

mod test {
    #[test]
    fn test_mag_pb() {
        let mut mag = crate::items::Mag {
            mag: crate::items::MagType::Sato,
            iq: 0,
            sync: 0,
            def: 0,
            pow: 0,
            dex:0,
            mnd:0,
            pbs: [Some(crate::items::PhotonBlast::Leilla),
                  Some(crate::items::PhotonBlast::Pilla),
                  Some(crate::items::PhotonBlast::MyllaYoulla)],
            color: crate::items::MagColor::Null,
        };

        println!("{:X?}", mag.photon_blast_value());
    }
}
