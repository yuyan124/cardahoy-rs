use anyhow::Result;
use std::{
    collections::{HashMap, HashSet},
    sync::OnceLock,
};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString};

static NFT_CARD_MAP: OnceLock<HashMap<u32, NftCard>> = OnceLock::new();

#[derive(Debug)]
pub enum NftId {
    Cards = 12,
    Boxes = 13,
    Fragments = 15,
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum NftCardCategory {
    Animal,
    Plant,
    Zombie,
    Neutral,
    Dragon,
    Mech,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum NftCardColor {
    Orange,
    OrangeGold,
    Purple,
    PurpleGold,
    Blue,
    BlueGold,
    White,
    WhiteGold,
}

#[derive(Debug)]
pub enum NftSortType {
    PriceAscending = 0,
    PriceDescending,
    SalesVolumeAscending,
    SalesVolumeDescending,
    QuantityAscending,
    QuantityDescending,
}

#[derive(Debug)]
pub struct NftCard {
    pub value: u32,
    pub id: NftCardId,
    pub category: NftCardCategory,
    pub color: NftCardColor,
    pub name_en: String,
    pub name_cn: &'static str,
}
impl NftCard {
    pub fn new(
        value: u32,
        id: NftCardId,
        category: NftCardCategory,
        color: NftCardColor,
        name_en: String,
        name_cn: &'static str,
    ) -> Self {
        Self {
            value,
            id,
            category,
            color,
            name_en,
            name_cn,
        }
    }

    pub fn value(&self) -> u32 {
        self.value
    }

    pub fn id(&self) -> &NftCardId {
        &self.id
    }

    pub fn card_type(&self) -> &NftCardCategory {
        &self.category
    }

    pub fn name_cn(&self) -> &'static str {
        self.name_cn
    }

    pub fn name_en(&self) -> &str {
        self.name_en.as_str()
    }
}

// 该枚举由 script 生成。
#[derive(Debug, EnumString, EnumIter, Display, PartialEq, Eq, Hash, Copy, Clone)]
pub enum NftCardId {
    #[strum(to_string = "Alarm Bot")]
    AlarmBot = 500,
    #[strum(to_string = "Alarm Bot (Gold)")]
    AlarmBotGold = 537,
    #[strum(to_string = "Ammo Crate")]
    AmmoCrate = 320,
    #[strum(to_string = "Ammo Crate (Gold)")]
    AmmoCrateGold = 437,
    #[strum(to_string = "Angry Bull")]
    AngryBull = 415,
    #[strum(to_string = "Angry Bull (Gold)")]
    AngryBullGold = 440,
    #[strum(to_string = "Apple")]
    Apple = 393,
    #[strum(to_string = "Apple (Gold)")]
    AppleGold = 402,
    #[strum(to_string = "Arrogant Boar")]
    ArrogantBoar = 370,
    #[strum(to_string = "Arrogant Boar (Gold)")]
    ArrogantBoarGold = 372,
    #[strum(to_string = "Avenger D0")]
    AvengerD0 = 544,
    #[strum(to_string = "Avenger D0 (Gold)")]
    AvengerD0Gold = 561,
    #[strum(to_string = "Avocado")]
    Avocado = 373,
    #[strum(to_string = "Avocado (Gold)")]
    AvocadoGold = 436,
    #[strum(to_string = "Axe")]
    Axe = 369,
    #[strum(to_string = "Axe (Gold)")]
    AxeGold = 371,
    #[strum(to_string = "Barrel Zombie")]
    BarrelZombie = 455,
    #[strum(to_string = "Barrel Zombie (Gold)")]
    BarrelZombieGold = 460,
    #[strum(to_string = "Bomb Zombie")]
    BombZombie = 361,
    #[strum(to_string = "Bomb Zombie (Gold)")]
    BombZombieGold = 338,
    #[strum(to_string = "Boxing Gloves")]
    BoxingGloves = 427,
    #[strum(to_string = "Boxing Gloves (Gold)")]
    BoxingGlovesGold = 317,
    #[strum(to_string = "Brave Beaver")]
    BraveBeaver = 375,
    #[strum(to_string = "Brave Beaver (Gold)")]
    BraveBeaverGold = 383,
    #[strum(to_string = "Broom")]
    Broom = 390,
    #[strum(to_string = "Broom (Gold)")]
    BroomGold = 331,
    #[strum(to_string = "Cabbage")]
    Cabbage = 382,
    #[strum(to_string = "Cabbage (Gold)")]
    CabbageGold = 442,
    #[strum(to_string = "Cactus")]
    Cactus = 348,
    #[strum(to_string = "Cactus (Gold)")]
    CactusGold = 351,
    #[strum(to_string = "Cannon")]
    Cannon = 342,
    #[strum(to_string = "Cannon (Gold)")]
    CannonGold = 344,
    #[strum(to_string = "Chainsaw")]
    Chainsaw = 345,
    #[strum(to_string = "Chainsaw (Gold)")]
    ChainsawGold = 349,
    #[strum(to_string = "Charge Bot")]
    ChargeBot = 553,
    #[strum(to_string = "Charge Bot (Gold)")]
    ChargeBotGold = 509,
    #[strum(to_string = "Cheerleader Zombie")]
    CheerleaderZombie = 332,
    #[strum(to_string = "Cheerleader Zombie (Gold)")]
    CheerleaderZombieGold = 410,
    #[strum(to_string = "Clown Zombie")]
    ClownZombie = 334,
    #[strum(to_string = "Clown Zombie (Gold)")]
    ClownZombieGold = 439,
    #[strum(to_string = "Coconut")]
    Coconut = 409,
    #[strum(to_string = "Coconut (Gold)")]
    CoconutGold = 411,
    #[strum(to_string = "Crazy Doctor Zombie")]
    CrazyDoctorZombie = 459,
    #[strum(to_string = "Crazy Doctor Zombie (Gold)")]
    CrazyDoctorZombieGold = 452,
    #[strum(to_string = "Crystal Dragon")]
    CrystalDragon = 521,
    #[strum(to_string = "Crystal Dragon (Gold)")]
    CrystalDragonGold = 531,
    #[strum(to_string = "Cunning Fox")]
    CunningFox = 380,
    #[strum(to_string = "Cunning Fox (Gold)")]
    CunningFoxGold = 449,
    #[strum(to_string = "Dancer Zombie")]
    DancerZombie = 352,
    #[strum(to_string = "Dancer Zombie (Gold)")]
    DancerZombieGold = 354,
    #[strum(to_string = "Death Knight")]
    DeathKnight = 310,
    #[strum(to_string = "Death Knight (Gold)")]
    DeathKnightGold = 374,
    #[strum(to_string = "Deception Dragon")]
    DeceptionDragon = 498,
    #[strum(to_string = "Deception Dragon (Gold)")]
    DeceptionDragonGold = 545,
    #[strum(to_string = "Deterrent Bot")]
    DeterrentBot = 505,
    #[strum(to_string = "Deterrent Bot (Gold)")]
    DeterrentBotGold = 546,
    #[strum(to_string = "Doggy Bot")]
    DoggyBot = 519,
    #[strum(to_string = "Doggy Bot (Gold)")]
    DoggyBotGold = 556,
    #[strum(to_string = "Dragon Egg")]
    DragonEgg = 523,
    #[strum(to_string = "Dragon Egg (Gold)")]
    DragonEggGold = 535,
    #[strum(to_string = "Durian Agent")]
    DurianAgent = 443,
    #[strum(to_string = "Durian Agent (Gold)")]
    DurianAgentGold = 458,
    #[strum(to_string = "Durian Commander")]
    DurianCommander = 453,
    #[strum(to_string = "Durian Commander (Gold)")]
    DurianCommanderGold = 463,
    #[strum(to_string = "ES67")]
    ES67 = 529,
    #[strum(to_string = "ES67 (Gold)")]
    ES67Gold = 536,
    #[strum(to_string = "Epic Miracle Egg")]
    EpicMiracleEgg = 474,
    #[strum(to_string = "Epic Miracle Egg (Gold)")]
    EpicMiracleEggGold = 487,
    #[strum(to_string = "Farmer Zombie")]
    FarmerZombie = 377,
    #[strum(to_string = "Farmer Zombie (Gold)")]
    FarmerZombieGold = 389,
    #[strum(to_string = "Friendly Hen")]
    FriendlyHen = 401,
    #[strum(to_string = "Friendly Hen (Gold)")]
    FriendlyHenGold = 315,
    #[strum(to_string = "Frog")]
    Frog = 451,
    #[strum(to_string = "Frog (Gold)")]
    FrogGold = 461,
    #[strum(to_string = "Frost Dragon")]
    FrostDragon = 524,
    #[strum(to_string = "Frost Dragon (Gold)")]
    FrostDragonGold = 550,
    #[strum(to_string = "Frying Pan")]
    FryingPan = 398,
    #[strum(to_string = "Frying Pan (Gold)")]
    FryingPanGold = 399,
    #[strum(to_string = "Furious Tiger")]
    FuriousTiger = 419,
    #[strum(to_string = "Furious Tiger (Gold)")]
    FuriousTigerGold = 396,
    #[strum(to_string = "GT130")]
    GT130 = 547,
    #[strum(to_string = "GT130 (Gold)")]
    GT130Gold = 565,
    #[strum(to_string = "Garlic")]
    Garlic = 441,
    #[strum(to_string = "Garlic (Gold)")]
    GarlicGold = 435,
    #[strum(to_string = "Grandpa Broccoli")]
    GrandpaBroccoli = 404,
    #[strum(to_string = "Grandpa Broccoli (Gold)")]
    GrandpaBroccoliGold = 429,
    #[strum(to_string = "Greedy Shark")]
    GreedyShark = 379,
    #[strum(to_string = "Greedy Shark (Gold)")]
    GreedySharkGold = 387,
    #[strum(to_string = "Guardian Bot")]
    GuardianBot = 557,
    #[strum(to_string = "Guardian Bot (Gold)")]
    GuardianBotGold = 560,
    #[strum(to_string = "Hardshell Nut")]
    HardshellNut = 394,
    #[strum(to_string = "Hardshell Nut (Gold)")]
    HardshellNutGold = 425,
    #[strum(to_string = "Helper Bot")]
    HelperBot = 526,
    #[strum(to_string = "Helper Bot (Gold)")]
    HelperBotGold = 540,
    #[strum(to_string = "Inferno Dragon")]
    InfernoDragon = 510,
    #[strum(to_string = "Inferno Dragon (Gold)")]
    InfernoDragonGold = 520,
    #[strum(to_string = "Inflatable Hammer")]
    InflatableHammer = 385,
    #[strum(to_string = "Inflatable Hammer (Gold)")]
    InflatableHammerGold = 403,
    #[strum(to_string = "Iron Helmet")]
    IronHelmet = 346,
    #[strum(to_string = "Iron Helmet (Gold)")]
    IronHelmetGold = 347,
    #[strum(to_string = "Joyful Lamb")]
    JoyfulLamb = 366,
    #[strum(to_string = "Joyful Lamb (Gold)")]
    JoyfulLambGold = 367,
    #[strum(to_string = "Kitty Bot")]
    KittyBot = 502,
    #[strum(to_string = "Kitty Bot (Gold)")]
    KittyBotGold = 542,
    #[strum(to_string = "Lava Dragon")]
    LavaDragon = 507,
    #[strum(to_string = "Lava Dragon (Gold)")]
    LavaDragonGold = 527,
    #[strum(to_string = "Legendary Miracle Egg")]
    LegendaryMiracleEgg = 476,
    #[strum(to_string = "Legendary Miracle Egg (Gold)")]
    LegendaryMiracleEggGold = 486,
    #[strum(to_string = "Lion King")]
    LionKing = 324,
    #[strum(to_string = "Lion King (Gold)")]
    LionKingGold = 330,
    #[strum(to_string = "Lobster")]
    Lobster = 391,
    #[strum(to_string = "Lobster (Gold)")]
    LobsterGold = 397,
    #[strum(to_string = "Mad Rabbit")]
    MadRabbit = 318,
    #[strum(to_string = "Mad Rabbit (Gold)")]
    MadRabbitGold = 335,
    #[strum(to_string = "Master Panda")]
    MasterPanda = 424,
    #[strum(to_string = "Master Panda (Gold)")]
    MasterPandaGold = 432,
    #[strum(to_string = "Mech Hatchery")]
    MechHatchery = 518,
    #[strum(to_string = "Mech Hatchery (Gold)")]
    MechHatcheryGold = 558,
    #[strum(to_string = "Mechanic Knight")]
    MechanicKnight = 532,
    #[strum(to_string = "Mechanic Knight (Gold)")]
    MechanicKnightGold = 543,
    #[strum(to_string = "Miner Zombie")]
    MinerZombie = 384,
    #[strum(to_string = "Miner Zombie (Gold)")]
    MinerZombieGold = 388,
    #[strum(to_string = "Mini Robot")]
    MiniRobot = 503,
    #[strum(to_string = "Mini Robot (Gold)")]
    MiniRobotGold = 492,
    #[strum(to_string = "Mr. Corn")]
    MrCorn = 413,
    #[strum(to_string = "Mr. Corn (Gold)")]
    MrCornGold = 395,
    #[strum(to_string = "Mr. Tomato")]
    MrTomato = 418,
    #[strum(to_string = "Mr. Tomato (Gold)")]
    MrTomatoGold = 333,
    #[strum(to_string = "Nature Dragon")]
    NatureDragon = 517,
    #[strum(to_string = "Nature Dragon (Gold)")]
    NatureDragonGold = 554,
    #[strum(to_string = "Naughty Dragon")]
    NaughtyDragon = 495,
    #[strum(to_string = "Naughty Dragon (Gold)")]
    NaughtyDragonGold = 513,
    #[strum(to_string = "Obsidian Dragon")]
    ObsidianDragon = 512,
    #[strum(to_string = "Obsidian Dragon (Gold)")]
    ObsidianDragonGold = 534,
    #[strum(to_string = "Ocean Dragon")]
    OceanDragon = 551,
    #[strum(to_string = "Ocean Dragon (Gold)")]
    OceanDragonGold = 555,
    #[strum(to_string = "Party Zombie")]
    PartyZombie = 423,
    #[strum(to_string = "Party Zombie (Gold)")]
    PartyZombieGold = 446,
    #[strum(to_string = "Pineapple")]
    Pineapple = 355,
    #[strum(to_string = "Pineapple (Gold)")]
    PineappleGold = 357,
    #[strum(to_string = "Pirate Zombie")]
    PirateZombie = 339,
    #[strum(to_string = "Pirate Zombie (Gold)")]
    PirateZombieGold = 448,
    #[strum(to_string = "Poison")]
    Poison = 422,
    #[strum(to_string = "Poison Mushroom")]
    PoisonMushroom = 392,
    #[strum(to_string = "Poison Mushroom (Gold)")]
    PoisonMushroomGold = 406,
    #[strum(to_string = "Poison (Gold)")]
    PoisonGold = 447,
    #[strum(to_string = "Prince Zombie")]
    PrinceZombie = 323,
    #[strum(to_string = "Prince Zombie (Gold)")]
    PrinceZombieGold = 311,
    #[strum(to_string = "Pumpkin")]
    Pumpkin = 326,
    #[strum(to_string = "Pumpkin (Gold)")]
    PumpkinGold = 341,
    #[strum(to_string = "Radiant Dragon")]
    RadiantDragon = 548,
    #[strum(to_string = "Radiant Dragon (Gold)")]
    RadiantDragonGold = 549,
    #[strum(to_string = "Radish")]
    Radish = 328,
    #[strum(to_string = "Radish (Gold)")]
    RadishGold = 386,
    #[strum(to_string = "Raging Shield")]
    RagingShield = 356,
    #[strum(to_string = "Raging Shield (Gold)")]
    RagingShieldGold = 336,
    #[strum(to_string = "Red Chili")]
    RedChili = 358,
    #[strum(to_string = "Red Chili (Gold)")]
    RedChiliGold = 360,
    #[strum(to_string = "Riot Shield")]
    RiotShield = 450,
    #[strum(to_string = "Riot Shield (Gold)")]
    RiotShieldGold = 457,
    #[strum(to_string = "Roadblock Zombie")]
    RoadblockZombie = 363,
    #[strum(to_string = "Roadblock Zombie (Gold)")]
    RoadblockZombieGold = 365,
    #[strum(to_string = "Robot Duke")]
    RobotDuke = 516,
    #[strum(to_string = "Robot Duke (Gold)")]
    RobotDukeGold = 511,
    #[strum(to_string = "Rock Dragon")]
    RockDragon = 559,
    #[strum(to_string = "Rock Dragon (Gold)")]
    RockDragonGold = 562,
    #[strum(to_string = "Rocket")]
    Rocket = 414,
    #[strum(to_string = "Rocket (Gold)")]
    RocketGold = 417,
    #[strum(to_string = "Sailor Zombie")]
    SailorZombie = 405,
    #[strum(to_string = "Sailor Zombie (Gold)")]
    SailorZombieGold = 325,
    #[strum(to_string = "Scout Bot")]
    ScoutBot = 515,
    #[strum(to_string = "Scout Bot (Gold)")]
    ScoutBotGold = 508,
    #[strum(to_string = "Shadow Dragon")]
    ShadowDragon = 514,
    #[strum(to_string = "Shadow Dragon (Gold)")]
    ShadowDragonGold = 506,
    #[strum(to_string = "Shotgun")]
    Shotgun = 353,
    #[strum(to_string = "Shotgun (Gold)")]
    ShotgunGold = 319,
    #[strum(to_string = "Snail")]
    Snail = 407,
    #[strum(to_string = "Snail (Gold)")]
    SnailGold = 322,
    #[strum(to_string = "Snake")]
    Snake = 416,
    #[strum(to_string = "Snake (Gold)")]
    SnakeGold = 434,
    #[strum(to_string = "Spiked Bat")]
    SpikedBat = 301,
    #[strum(to_string = "Spiked Bat (Gold)")]
    SpikedBatGold = 400,
    #[strum(to_string = "Star Zombie")]
    StarZombie = 421,
    #[strum(to_string = "Star Zombie (Gold)")]
    StarZombieGold = 438,
    #[strum(to_string = "Starving Mimic")]
    StarvingMimic = 420,
    #[strum(to_string = "Starving Mimic (Gold)")]
    StarvingMimicGold = 428,
    #[strum(to_string = "Steel Dragon")]
    SteelDragon = 504,
    #[strum(to_string = "Steel Dragon (Gold)")]
    SteelDragonGold = 538,
    #[strum(to_string = "Stone")]
    Stone = 499,
    #[strum(to_string = "Stone (Gold)")]
    StoneGold = 525,
    #[strum(to_string = "Storm Dragon")]
    StormDragon = 552,
    #[strum(to_string = "Storm Dragon (Gold)")]
    StormDragonGold = 563,
    #[strum(to_string = "Sturdy Bot")]
    SturdyBot = 494,
    #[strum(to_string = "Sturdy Bot (Gold)")]
    SturdyBotGold = 528,
    #[strum(to_string = "Super Zombie")]
    SuperZombie = 378,
    #[strum(to_string = "Super Zombie (Gold)")]
    SuperZombieGold = 431,
    #[strum(to_string = "Support Bot")]
    SupportBot = 533,
    #[strum(to_string = "Support Bot (Gold)")]
    SupportBotGold = 539,
    #[strum(to_string = "Terra Dragon")]
    TerraDragon = 497,
    #[strum(to_string = "Terra Dragon (Gold)")]
    TerraDragonGold = 541,
    #[strum(to_string = "Thunder Dragon")]
    ThunderDragon = 530,
    #[strum(to_string = "Thunder Dragon (Gold)")]
    ThunderDragonGold = 564,
    #[strum(to_string = "Tortoise Bro")]
    TortoiseBro = 350,
    #[strum(to_string = "Tortoise Bro (Gold)")]
    TortoiseBroGold = 445,
    #[strum(to_string = "Trumpet")]
    Trumpet = 359,
    #[strum(to_string = "Trumpet (Gold)")]
    TrumpetGold = 362,
    #[strum(to_string = "Turner")]
    Turner = 364,
    #[strum(to_string = "Turner (Gold)")]
    TurnerGold = 368,
    #[strum(to_string = "UFO")]
    UFO = 337,
    #[strum(to_string = "UFO (Gold)")]
    UFOGold = 343,
    #[strum(to_string = "Uncle Elk")]
    UncleElk = 376,
    #[strum(to_string = "Uncle Elk (Gold)")]
    UncleElkGold = 408,
    #[strum(to_string = "Watermelon")]
    Watermelon = 426,
    #[strum(to_string = "Watermelon (Gold)")]
    WatermelonGold = 433,
    #[strum(to_string = "Wind Dragon")]
    WindDragon = 501,
    #[strum(to_string = "Wind Dragon (Gold)")]
    WindDragonGold = 522,
    #[strum(to_string = "Wolf King")]
    WolfKing = 454,
    #[strum(to_string = "Wolf King (Gold)")]
    WolfKingGold = 462,
    #[strum(to_string = "Zombie Baby")]
    ZombieBaby = 307,
    #[strum(to_string = "Zombie Baby (Gold)")]
    ZombieBabyGold = 412,
    #[strum(to_string = "Zombie Bride")]
    ZombieBride = 381,
    #[strum(to_string = "Zombie Bride (Gold)")]
    ZombieBrideGold = 430,
}

impl NftCardId {
    // 根据 id 获取枚举值
    pub fn from_id(id: u32) -> Option<Self> {
        Self::iter().find(|card| *card as u32 == id)
    }

    // 根据 lang 参数返回不同的字符串
    pub fn to_chinese(&self) -> &'static str {
        match self {
            Self::AlarmBot => "警报机器人",
            Self::AlarmBotGold => "警报机器人 (金)",
            Self::AmmoCrate => "弹药箱",
            Self::AmmoCrateGold => "弹药箱 (金)",
            Self::AngryBull => "愤怒的公牛",
            Self::AngryBullGold => "愤怒的公牛 (金)",
            Self::Apple => "苹果",
            Self::AppleGold => "苹果 (金)",
            Self::ArrogantBoar => "傲慢野猪",
            Self::ArrogantBoarGold => "傲慢野猪 (金)",
            Self::AvengerD0 => "复仇者 D0",
            Self::AvengerD0Gold => "复仇者 D0 (金)",
            Self::Avocado => "鳄梨",
            Self::AvocadoGold => "鳄梨 (金)",
            Self::Axe => "斧头",
            Self::AxeGold => "斧头 (金)",
            Self::BarrelZombie => "桶装僵尸",
            Self::BarrelZombieGold => "桶装僵尸 (金)",
            Self::BombZombie => "炸弹僵尸",
            Self::BombZombieGold => "炸弹僵尸 (金)",
            Self::BoxingGloves => "拳击手套",
            Self::BoxingGlovesGold => "拳击手套 (金)",
            Self::BraveBeaver => "勇敢的海狸",
            Self::BraveBeaverGold => "勇敢的海狸 (金)",
            Self::Broom => "扫帚",
            Self::BroomGold => "扫帚 (金)",
            Self::Cabbage => "卷心菜",
            Self::CabbageGold => "卷心菜 (金)",
            Self::Cactus => "仙人掌",
            Self::CactusGold => "仙人掌 (金)",
            Self::Cannon => "大炮",
            Self::CannonGold => "大炮 (金)",
            Self::Chainsaw => "电锯",
            Self::ChainsawGold => "电锯 (金)",
            Self::ChargeBot => "充能机器人",
            Self::ChargeBotGold => "充能机器人 (金)",
            Self::CheerleaderZombie => "啦啦队僵尸",
            Self::CheerleaderZombieGold => "啦啦队僵尸 (金)",
            Self::ClownZombie => "小丑僵尸",
            Self::ClownZombieGold => "小丑僵尸 (金)",
            Self::Coconut => "椰子",
            Self::CoconutGold => "椰子 (金)",
            Self::CrazyDoctorZombie => "疯狂医生僵尸",
            Self::CrazyDoctorZombieGold => "疯狂医生僵尸 (金)",
            Self::CrystalDragon => "水晶龙",
            Self::CrystalDragonGold => "水晶龙 (金)",
            Self::CunningFox => "狡猾的狐狸",
            Self::CunningFoxGold => "狡猾的狐狸 (金)",
            Self::DancerZombie => "舞者僵尸",
            Self::DancerZombieGold => "舞者僵尸 (金)",
            Self::DeathKnight => "死亡骑士",
            Self::DeathKnightGold => "死亡骑士 (金)",
            Self::DeceptionDragon => "欺诈之龙",
            Self::DeceptionDragonGold => "欺诈之龙 (金)",
            Self::DeterrentBot => "威慑机器人",
            Self::DeterrentBotGold => "威慑机器人 (金)",
            Self::DoggyBot => "狗狗机器人",
            Self::DoggyBotGold => "狗狗机器人 (金)",
            Self::DragonEgg => "龙蛋",
            Self::DragonEggGold => "龙蛋 (金)",
            Self::DurianAgent => "榴莲特工",
            Self::DurianAgentGold => "榴莲特工 (金)",
            Self::DurianCommander => "榴莲指挥官",
            Self::DurianCommanderGold => "榴莲指挥官 (金)",
            Self::ES67 => "ES67",
            Self::ES67Gold => "ES67 (金)",
            Self::EpicMiracleEgg => "史诗奇迹蛋",
            Self::EpicMiracleEggGold => "史诗奇迹蛋 (金)",
            Self::FarmerZombie => "农夫僵尸",
            Self::FarmerZombieGold => "农夫僵尸 (金)",
            Self::FriendlyHen => "友好母鸡",
            Self::FriendlyHenGold => "友好母鸡 (金)",
            Self::Frog => "青蛙",
            Self::FrogGold => "青蛙 (金)",
            Self::FrostDragon => "冰霜龙",
            Self::FrostDragonGold => "冰霜龙 (金)",
            Self::FryingPan => "煎锅",
            Self::FryingPanGold => "煎锅 (金)",
            Self::FuriousTiger => "愤怒的老虎",
            Self::FuriousTigerGold => "愤怒的老虎 (金)",
            Self::GT130 => "GT130",
            Self::GT130Gold => "GT130 (金)",
            Self::Garlic => "大蒜",
            Self::GarlicGold => "大蒜 (金)",
            Self::GrandpaBroccoli => "爷爷花椰菜",
            Self::GrandpaBroccoliGold => "爷爷花椰菜 (金)",
            Self::GreedyShark => "贪婪的鲨鱼",
            Self::GreedySharkGold => "贪婪的鲨鱼 (金)",
            Self::GuardianBot => "守护机器人",
            Self::GuardianBotGold => "守护机器人 (金)",
            Self::HardshellNut => "硬壳坚果",
            Self::HardshellNutGold => "硬壳坚果 (金)",
            Self::HelperBot => "辅助机器人",
            Self::HelperBotGold => "辅助机器人 (金)",
            Self::InfernoDragon => "地狱火龙",
            Self::InfernoDragonGold => "地狱火龙 (金)",
            Self::InflatableHammer => "充气锤子",
            Self::InflatableHammerGold => "充气锤子 (金)",
            Self::IronHelmet => "铁头盔",
            Self::IronHelmetGold => "铁头盔 (金)",
            Self::JoyfulLamb => "快乐小羊",
            Self::JoyfulLambGold => "快乐小羊 (金)",
            Self::KittyBot => "小猫机器人",
            Self::KittyBotGold => "小猫机器人 (金)",
            Self::LavaDragon => "熔岩龙",
            Self::LavaDragonGold => "熔岩龙 (金)",
            Self::LegendaryMiracleEgg => "传奇奇迹蛋",
            Self::LegendaryMiracleEggGold => "传奇奇迹蛋 (金)",
            Self::LionKing => "狮子王",
            Self::LionKingGold => "狮子王 (金)",
            Self::Lobster => "龙虾",
            Self::LobsterGold => "龙虾 (金)",
            Self::MadRabbit => "疯狂兔子",
            Self::MadRabbitGold => "疯狂兔子 (金)",
            Self::MasterPanda => "熊猫大师",
            Self::MasterPandaGold => "熊猫大师 (金)",
            Self::MechHatchery => "机械孵化场",
            Self::MechHatcheryGold => "机械孵化场 (金)",
            Self::MechanicKnight => "机械骑士",
            Self::MechanicKnightGold => "机械骑士 (金)",
            Self::MinerZombie => "矿工僵尸",
            Self::MinerZombieGold => "矿工僵尸 (金)",
            Self::MiniRobot => "迷你机器人",
            Self::MiniRobotGold => "迷你机器人 (金)",
            Self::MrCorn => "玉米先生",
            Self::MrCornGold => "玉米先生 (金)",
            Self::MrTomato => "番茄先生",
            Self::MrTomatoGold => "番茄先生 (金)",
            Self::NatureDragon => "自然龙",
            Self::NatureDragonGold => "自然龙 (金)",
            Self::NaughtyDragon => "顽皮龙",
            Self::NaughtyDragonGold => "顽皮龙 (金)",
            Self::ObsidianDragon => "黑曜石龙",
            Self::ObsidianDragonGold => "黑曜石龙 (金)",
            Self::OceanDragon => "海洋龙",
            Self::OceanDragonGold => "海洋龙 (金)",
            Self::PartyZombie => "派对僵尸",
            Self::PartyZombieGold => "派对僵尸 (金)",
            Self::Pineapple => "菠萝",
            Self::PineappleGold => "菠萝 (金)",
            Self::PirateZombie => "海盗僵尸",
            Self::PirateZombieGold => "海盗僵尸 (金)",
            Self::Poison => "毒药",
            Self::PoisonMushroom => "毒蘑菇",
            Self::PoisonMushroomGold => "毒蘑菇 (金)",
            Self::PoisonGold => "毒药 (金)",
            Self::PrinceZombie => "僵尸王子",
            Self::PrinceZombieGold => "僵尸王子 (金)",
            Self::Pumpkin => "南瓜",
            Self::PumpkinGold => "南瓜 (金)",
            Self::RadiantDragon => "光辉龙",
            Self::RadiantDragonGold => "光辉龙 (金)",
            Self::Radish => "萝卜",
            Self::RadishGold => "萝卜 (金)",
            Self::RagingShield => "暴怒之盾",
            Self::RagingShieldGold => "暴怒之盾 (金)",
            Self::RedChili => "红辣椒",
            Self::RedChiliGold => "红辣椒 (金)",
            Self::RiotShield => "防暴盾牌",
            Self::RiotShieldGold => "防暴盾牌 (金)",
            Self::RoadblockZombie => "路障僵尸",
            Self::RoadblockZombieGold => "路障僵尸 (金)",
            Self::RobotDuke => "机器人公爵",
            Self::RobotDukeGold => "机器人公爵 (金)",
            Self::RockDragon => "岩石龙",
            Self::RockDragonGold => "岩石龙 (金)",
            Self::Rocket => "火箭",
            Self::RocketGold => "火箭 (金)",
            Self::SailorZombie => "水手僵尸",
            Self::SailorZombieGold => "水手僵尸 (金)",
            Self::ScoutBot => "侦察机器人",
            Self::ScoutBotGold => "侦察机器人 (金)",
            Self::ShadowDragon => "暗影龙",
            Self::ShadowDragonGold => "暗影龙 (金)",
            Self::Shotgun => "霰弹枪",
            Self::ShotgunGold => "霰弹枪 (金)",
            Self::Snail => "蜗牛",
            Self::SnailGold => "蜗牛 (金)",
            Self::Snake => "蛇",
            Self::SnakeGold => "蛇 (金)",
            Self::SpikedBat => "尖刺球棒",
            Self::SpikedBatGold => "尖刺球棒 (金)",
            Self::StarZombie => "星星僵尸",
            Self::StarZombieGold => "星星僵尸 (金)",
            Self::StarvingMimic => "饥饿的拟态怪",
            Self::StarvingMimicGold => "饥饿的拟态怪 (金)",
            Self::SteelDragon => "钢铁龙",
            Self::SteelDragonGold => "钢铁龙 (金)",
            Self::Stone => "石头",
            Self::StoneGold => "石头 (金)",
            Self::StormDragon => "风暴龙",
            Self::StormDragonGold => "风暴龙 (金)",
            Self::SturdyBot => "坚固机器人",
            Self::SturdyBotGold => "坚固机器人 (金)",
            Self::SuperZombie => "超能僵尸",
            Self::SuperZombieGold => "超能僵尸 (金)",
            Self::SupportBot => "支援机器人",
            Self::SupportBotGold => "支援机器人 (金)",
            Self::TerraDragon => "泰拉龙",
            Self::TerraDragonGold => "泰拉龙 (金)",
            Self::ThunderDragon => "雷电龙",
            Self::ThunderDragonGold => "雷电龙 (金)",
            Self::TortoiseBro => "乌龟",
            Self::TortoiseBroGold => "乌龟 (金)",
            Self::Trumpet => "小号",
            Self::TrumpetGold => "小号 (金)",
            Self::Turner => "转盘",
            Self::TurnerGold => "转盘 (金)",
            Self::UFO => "UFO",
            Self::UFOGold => "UFO (金)",
            Self::UncleElk => "麋鹿叔叔",
            Self::UncleElkGold => "麋鹿叔叔 (金)",
            Self::Watermelon => "西瓜",
            Self::WatermelonGold => "西瓜 (金)",
            Self::WindDragon => "风之龙",
            Self::WindDragonGold => "风之龙 (金)",
            Self::WolfKing => "狼王",
            Self::WolfKingGold => "狼王 (金)",
            Self::ZombieBaby => "僵尸宝宝",
            Self::ZombieBabyGold => "僵尸宝宝 (金)",
            Self::ZombieBride => "僵尸新娘",
            Self::ZombieBrideGold => "僵尸新娘 (金)",
        }
    }

    pub fn get_name_by_id(id: NftCardId, lang: &str) -> String {
        let name = match lang {
            "cn" => id.to_chinese().to_string(),
            _ => id.to_string(),
        };
        name
    }

    pub fn get_name_by_value(value: u32, lang: &str) -> Option<String> {
        let card_id = NftCardId::from_id(value)?;
        let name = match lang {
            "cn" => card_id.to_chinese().to_string(),
            _ => card_id.to_string(),
        };
        Some(name)
    }
}

pub fn init_nft_card_map() {
    let map = generate_nft_card_map();
    let _ = NFT_CARD_MAP.set(map);
}

fn generate_nft_card_map() -> HashMap<u32, NftCard> {
    let mut card_map = HashMap::new();

    for card in NftCardId::iter() {
        let value = card as u32;
        let name_en = card.to_string();
        let name_cn = card.to_chinese();
        let color = get_nft_card_color(card);
        let category = get_nft_card_category(card);
        let c = NftCard::new(value, card, category, color, name_en, name_cn);
        card_map.insert(value, c);
    }

    card_map
}

fn get_nft_card_category(id: NftCardId) -> NftCardCategory {
    match id {
        // Animal
        NftCardId::WolfKing => NftCardCategory::Animal,
        NftCardId::WolfKingGold => NftCardCategory::Animal,
        NftCardId::MasterPanda => NftCardCategory::Animal,
        NftCardId::MasterPandaGold => NftCardCategory::Animal,
        NftCardId::LionKing => NftCardCategory::Animal,
        NftCardId::LionKingGold => NftCardCategory::Animal,
        NftCardId::Frog => NftCardCategory::Animal,
        NftCardId::FrogGold => NftCardCategory::Animal,
        NftCardId::FuriousTiger => NftCardCategory::Animal,
        NftCardId::FuriousTigerGold => NftCardCategory::Animal,
        NftCardId::CunningFox => NftCardCategory::Animal,
        NftCardId::CunningFoxGold => NftCardCategory::Animal,
        NftCardId::Snake => NftCardCategory::Animal,
        NftCardId::SnakeGold => NftCardCategory::Animal,
        NftCardId::AngryBull => NftCardCategory::Animal,
        NftCardId::AngryBullGold => NftCardCategory::Animal,
        NftCardId::TortoiseBro => NftCardCategory::Animal,
        NftCardId::TortoiseBroGold => NftCardCategory::Animal,
        NftCardId::UncleElk => NftCardCategory::Animal,
        NftCardId::UncleElkGold => NftCardCategory::Animal,
        NftCardId::GreedyShark => NftCardCategory::Animal,
        NftCardId::GreedySharkGold => NftCardCategory::Animal,
        NftCardId::Lobster => NftCardCategory::Animal,
        NftCardId::LobsterGold => NftCardCategory::Animal,
        NftCardId::BraveBeaver => NftCardCategory::Animal,
        NftCardId::BraveBeaverGold => NftCardCategory::Animal,
        NftCardId::JoyfulLamb => NftCardCategory::Animal,
        NftCardId::JoyfulLambGold => NftCardCategory::Animal,
        NftCardId::Snail => NftCardCategory::Animal,
        NftCardId::SnailGold => NftCardCategory::Animal,
        NftCardId::ArrogantBoar => NftCardCategory::Animal,
        NftCardId::ArrogantBoarGold => NftCardCategory::Animal,
        NftCardId::FriendlyHen => NftCardCategory::Animal,
        NftCardId::FriendlyHenGold => NftCardCategory::Animal,
        NftCardId::MadRabbit => NftCardCategory::Animal,
        NftCardId::MadRabbitGold => NftCardCategory::Animal,
        // Plant
        NftCardId::DurianCommander => NftCardCategory::Plant,
        NftCardId::DurianCommanderGold => NftCardCategory::Plant,
        NftCardId::Garlic => NftCardCategory::Plant,
        NftCardId::GarlicGold => NftCardCategory::Plant,
        NftCardId::Pumpkin => NftCardCategory::Plant,
        NftCardId::PumpkinGold => NftCardCategory::Plant,
        NftCardId::Watermelon => NftCardCategory::Plant,
        NftCardId::WatermelonGold => NftCardCategory::Plant,
        NftCardId::Cabbage => NftCardCategory::Plant,
        NftCardId::CabbageGold => NftCardCategory::Plant,
        NftCardId::GrandpaBroccoli => NftCardCategory::Plant,
        NftCardId::GrandpaBroccoliGold => NftCardCategory::Plant,
        NftCardId::MrTomato => NftCardCategory::Plant,
        NftCardId::MrTomatoGold => NftCardCategory::Plant,
        NftCardId::RedChili => NftCardCategory::Plant,
        NftCardId::RedChiliGold => NftCardCategory::Plant,
        NftCardId::DurianAgent => NftCardCategory::Plant,
        NftCardId::DurianAgentGold => NftCardCategory::Plant,
        NftCardId::Avocado => NftCardCategory::Plant,
        NftCardId::AvocadoGold => NftCardCategory::Plant,
        NftCardId::Cactus => NftCardCategory::Plant,
        NftCardId::CactusGold => NftCardCategory::Plant,
        NftCardId::Pineapple => NftCardCategory::Plant,
        NftCardId::PineappleGold => NftCardCategory::Plant,
        NftCardId::Radish => NftCardCategory::Plant,
        NftCardId::RadishGold => NftCardCategory::Plant,
        NftCardId::Coconut => NftCardCategory::Plant,
        NftCardId::CoconutGold => NftCardCategory::Plant,
        NftCardId::MrCorn => NftCardCategory::Plant,
        NftCardId::MrCornGold => NftCardCategory::Plant,
        NftCardId::PoisonMushroom => NftCardCategory::Plant,
        NftCardId::PoisonMushroomGold => NftCardCategory::Plant,
        NftCardId::Apple => NftCardCategory::Plant,
        NftCardId::AppleGold => NftCardCategory::Plant,
        NftCardId::HardshellNut => NftCardCategory::Plant,
        NftCardId::HardshellNutGold => NftCardCategory::Plant,
        // Zombie
        NftCardId::CrazyDoctorZombie => NftCardCategory::Zombie,
        NftCardId::CrazyDoctorZombieGold => NftCardCategory::Zombie,
        NftCardId::SuperZombie => NftCardCategory::Zombie,
        NftCardId::SuperZombieGold => NftCardCategory::Zombie,
        NftCardId::PrinceZombie => NftCardCategory::Zombie,
        NftCardId::PrinceZombieGold => NftCardCategory::Zombie,
        NftCardId::PirateZombie => NftCardCategory::Zombie,
        NftCardId::PirateZombieGold => NftCardCategory::Zombie,
        NftCardId::PartyZombie => NftCardCategory::Zombie,
        NftCardId::PartyZombieGold => NftCardCategory::Zombie,
        NftCardId::BarrelZombie => NftCardCategory::Zombie,
        NftCardId::BarrelZombieGold => NftCardCategory::Zombie,
        NftCardId::StarZombie => NftCardCategory::Zombie,
        NftCardId::StarZombieGold => NftCardCategory::Zombie,
        NftCardId::ClownZombie => NftCardCategory::Zombie,
        NftCardId::ClownZombieGold => NftCardCategory::Zombie,
        NftCardId::ZombieBride => NftCardCategory::Zombie,
        NftCardId::ZombieBrideGold => NftCardCategory::Zombie,
        NftCardId::BombZombie => NftCardCategory::Zombie,
        NftCardId::BombZombieGold => NftCardCategory::Zombie,
        NftCardId::CheerleaderZombie => NftCardCategory::Zombie,
        NftCardId::CheerleaderZombieGold => NftCardCategory::Zombie,
        NftCardId::RoadblockZombie => NftCardCategory::Zombie,
        NftCardId::RoadblockZombieGold => NftCardCategory::Zombie,
        NftCardId::DancerZombie => NftCardCategory::Zombie,
        NftCardId::DancerZombieGold => NftCardCategory::Zombie,
        NftCardId::MinerZombie => NftCardCategory::Zombie,
        NftCardId::MinerZombieGold => NftCardCategory::Zombie,
        NftCardId::SailorZombie => NftCardCategory::Zombie,
        NftCardId::SailorZombieGold => NftCardCategory::Zombie,
        NftCardId::DeathKnight => NftCardCategory::Zombie,
        NftCardId::DeathKnightGold => NftCardCategory::Zombie,
        NftCardId::ZombieBaby => NftCardCategory::Zombie,
        NftCardId::ZombieBabyGold => NftCardCategory::Zombie,
        NftCardId::FarmerZombie => NftCardCategory::Zombie,
        NftCardId::FarmerZombieGold => NftCardCategory::Zombie,
        // Neutral
        NftCardId::LegendaryMiracleEgg => NftCardCategory::Neutral,
        NftCardId::LegendaryMiracleEggGold => NftCardCategory::Neutral,
        NftCardId::EpicMiracleEgg => NftCardCategory::Neutral,
        NftCardId::EpicMiracleEggGold => NftCardCategory::Neutral,
        NftCardId::RiotShield => NftCardCategory::Neutral,
        NftCardId::RiotShieldGold => NftCardCategory::Neutral,
        NftCardId::Poison => NftCardCategory::Neutral,
        NftCardId::PoisonGold => NftCardCategory::Neutral,
        NftCardId::AmmoCrate => NftCardCategory::Neutral,
        NftCardId::AmmoCrateGold => NftCardCategory::Neutral,
        NftCardId::Trumpet => NftCardCategory::Neutral,
        NftCardId::TrumpetGold => NftCardCategory::Neutral,
        NftCardId::StarvingMimic => NftCardCategory::Neutral,
        NftCardId::StarvingMimicGold => NftCardCategory::Neutral,
        NftCardId::Rocket => NftCardCategory::Neutral,
        NftCardId::RocketGold => NftCardCategory::Neutral,
        NftCardId::IronHelmet => NftCardCategory::Neutral,
        NftCardId::IronHelmetGold => NftCardCategory::Neutral,
        NftCardId::InflatableHammer => NftCardCategory::Neutral,
        NftCardId::InflatableHammerGold => NftCardCategory::Neutral,
        NftCardId::FryingPan => NftCardCategory::Neutral,
        NftCardId::FryingPanGold => NftCardCategory::Neutral,
        NftCardId::RagingShield => NftCardCategory::Neutral,
        NftCardId::RagingShieldGold => NftCardCategory::Neutral,
        NftCardId::UFO => NftCardCategory::Neutral,
        NftCardId::UFOGold => NftCardCategory::Neutral,
        NftCardId::Cannon => NftCardCategory::Neutral,
        NftCardId::CannonGold => NftCardCategory::Neutral,
        NftCardId::Turner => NftCardCategory::Neutral,
        NftCardId::TurnerGold => NftCardCategory::Neutral,
        NftCardId::Chainsaw => NftCardCategory::Neutral,
        NftCardId::ChainsawGold => NftCardCategory::Neutral,
        NftCardId::Shotgun => NftCardCategory::Neutral,
        NftCardId::ShotgunGold => NftCardCategory::Neutral,
        NftCardId::SpikedBat => NftCardCategory::Neutral,
        NftCardId::SpikedBatGold => NftCardCategory::Neutral,
        NftCardId::Broom => NftCardCategory::Neutral,
        NftCardId::BroomGold => NftCardCategory::Neutral,
        NftCardId::BoxingGloves => NftCardCategory::Neutral,
        NftCardId::BoxingGlovesGold => NftCardCategory::Neutral,
        NftCardId::Axe => NftCardCategory::Neutral,
        NftCardId::AxeGold => NftCardCategory::Neutral,

        // Dragon
        NftCardId::RadiantDragon => NftCardCategory::Dragon,
        NftCardId::RadiantDragonGold => NftCardCategory::Dragon,
        NftCardId::ThunderDragon => NftCardCategory::Dragon,
        NftCardId::ThunderDragonGold => NftCardCategory::Dragon,
        NftCardId::OceanDragon => NftCardCategory::Dragon,
        NftCardId::OceanDragonGold => NftCardCategory::Dragon,
        NftCardId::StormDragon => NftCardCategory::Dragon,
        NftCardId::StormDragonGold => NftCardCategory::Dragon,
        NftCardId::TerraDragon => NftCardCategory::Dragon,
        NftCardId::TerraDragonGold => NftCardCategory::Dragon,
        NftCardId::RockDragon => NftCardCategory::Dragon,
        NftCardId::RockDragonGold => NftCardCategory::Dragon,
        NftCardId::ShadowDragon => NftCardCategory::Dragon,
        NftCardId::ShadowDragonGold => NftCardCategory::Dragon,
        NftCardId::NatureDragon => NftCardCategory::Dragon,
        NftCardId::NatureDragonGold => NftCardCategory::Dragon,
        NftCardId::DeceptionDragon => NftCardCategory::Dragon,
        NftCardId::DeceptionDragonGold => NftCardCategory::Dragon,
        NftCardId::CrystalDragon => NftCardCategory::Dragon,
        NftCardId::CrystalDragonGold => NftCardCategory::Dragon,
        NftCardId::DragonEgg => NftCardCategory::Dragon,
        NftCardId::DragonEggGold => NftCardCategory::Dragon,
        NftCardId::FrostDragon => NftCardCategory::Dragon,
        NftCardId::FrostDragonGold => NftCardCategory::Dragon,
        NftCardId::SteelDragon => NftCardCategory::Dragon,
        NftCardId::SteelDragonGold => NftCardCategory::Dragon,
        NftCardId::InfernoDragon => NftCardCategory::Dragon,
        NftCardId::InfernoDragonGold => NftCardCategory::Dragon,
        NftCardId::ObsidianDragon => NftCardCategory::Dragon,
        NftCardId::ObsidianDragonGold => NftCardCategory::Dragon,
        NftCardId::WindDragon => NftCardCategory::Dragon,
        NftCardId::WindDragonGold => NftCardCategory::Dragon,
        NftCardId::NaughtyDragon => NftCardCategory::Dragon,
        NftCardId::NaughtyDragonGold => NftCardCategory::Dragon,
        NftCardId::LavaDragon => NftCardCategory::Dragon,
        NftCardId::LavaDragonGold => NftCardCategory::Dragon,

        // Mech
        NftCardId::GT130 => NftCardCategory::Mech,
        NftCardId::GT130Gold => NftCardCategory::Mech,
        NftCardId::ChargeBot => NftCardCategory::Mech,
        NftCardId::ChargeBotGold => NftCardCategory::Mech,
        NftCardId::GuardianBot => NftCardCategory::Mech,
        NftCardId::GuardianBotGold => NftCardCategory::Mech,
        NftCardId::DeterrentBot => NftCardCategory::Mech,
        NftCardId::DeterrentBotGold => NftCardCategory::Mech,
        NftCardId::DoggyBot => NftCardCategory::Mech,
        NftCardId::DoggyBotGold => NftCardCategory::Mech,
        NftCardId::MechHatchery => NftCardCategory::Mech,
        NftCardId::MechHatcheryGold => NftCardCategory::Mech,
        NftCardId::AvengerD0 => NftCardCategory::Mech,
        NftCardId::AvengerD0Gold => NftCardCategory::Mech,
        NftCardId::ScoutBot => NftCardCategory::Mech,
        NftCardId::ScoutBotGold => NftCardCategory::Mech,
        NftCardId::MechanicKnight => NftCardCategory::Mech,
        NftCardId::MechanicKnightGold => NftCardCategory::Mech,
        NftCardId::SupportBot => NftCardCategory::Mech,
        NftCardId::SupportBotGold => NftCardCategory::Mech,
        NftCardId::KittyBot => NftCardCategory::Mech,
        NftCardId::KittyBotGold => NftCardCategory::Mech,
        NftCardId::AlarmBot => NftCardCategory::Mech,
        NftCardId::AlarmBotGold => NftCardCategory::Mech,
        NftCardId::ES67 => NftCardCategory::Mech,
        NftCardId::ES67Gold => NftCardCategory::Mech,
        NftCardId::RobotDuke => NftCardCategory::Mech,
        NftCardId::RobotDukeGold => NftCardCategory::Mech,
        NftCardId::HelperBot => NftCardCategory::Mech,
        NftCardId::HelperBotGold => NftCardCategory::Mech,
        NftCardId::Stone => NftCardCategory::Mech,
        NftCardId::StoneGold => NftCardCategory::Mech,
        NftCardId::MiniRobot => NftCardCategory::Mech,
        NftCardId::MiniRobotGold => NftCardCategory::Mech,
        NftCardId::SturdyBot => NftCardCategory::Mech,
        NftCardId::SturdyBotGold => NftCardCategory::Mech,
    }
}

fn get_nft_card_color(id: NftCardId) -> NftCardColor {
    match id {
        // Animal
        NftCardId::WolfKing => NftCardColor::Orange,
        NftCardId::WolfKingGold => NftCardColor::OrangeGold,
        NftCardId::MasterPanda => NftCardColor::Orange,
        NftCardId::MasterPandaGold => NftCardColor::OrangeGold,
        NftCardId::LionKing => NftCardColor::Orange,
        NftCardId::LionKingGold => NftCardColor::OrangeGold,
        NftCardId::Frog => NftCardColor::Purple,
        NftCardId::FrogGold => NftCardColor::PurpleGold,
        NftCardId::FuriousTiger => NftCardColor::Purple,
        NftCardId::FuriousTigerGold => NftCardColor::PurpleGold,
        NftCardId::CunningFox => NftCardColor::Purple,
        NftCardId::CunningFoxGold => NftCardColor::PurpleGold,
        NftCardId::Snake => NftCardColor::Purple,
        NftCardId::SnakeGold => NftCardColor::PurpleGold,
        NftCardId::AngryBull => NftCardColor::Purple,
        NftCardId::AngryBullGold => NftCardColor::PurpleGold,
        NftCardId::TortoiseBro => NftCardColor::Blue,
        NftCardId::TortoiseBroGold => NftCardColor::BlueGold,
        NftCardId::UncleElk => NftCardColor::Blue,
        NftCardId::UncleElkGold => NftCardColor::BlueGold,
        NftCardId::GreedyShark => NftCardColor::Blue,
        NftCardId::GreedySharkGold => NftCardColor::BlueGold,
        NftCardId::Lobster => NftCardColor::Blue,
        NftCardId::LobsterGold => NftCardColor::BlueGold,
        NftCardId::BraveBeaver => NftCardColor::Blue,
        NftCardId::BraveBeaverGold => NftCardColor::BlueGold,
        NftCardId::JoyfulLamb => NftCardColor::Blue,
        NftCardId::JoyfulLambGold => NftCardColor::BlueGold,
        NftCardId::Snail => NftCardColor::White,
        NftCardId::SnailGold => NftCardColor::WhiteGold,
        NftCardId::ArrogantBoar => NftCardColor::White,
        NftCardId::ArrogantBoarGold => NftCardColor::WhiteGold,
        NftCardId::FriendlyHen => NftCardColor::White,
        NftCardId::FriendlyHenGold => NftCardColor::WhiteGold,
        NftCardId::MadRabbit => NftCardColor::White,
        NftCardId::MadRabbitGold => NftCardColor::WhiteGold,
        // Plant
        NftCardId::DurianCommander => NftCardColor::Orange,
        NftCardId::DurianCommanderGold => NftCardColor::OrangeGold,
        NftCardId::Garlic => NftCardColor::Orange,
        NftCardId::GarlicGold => NftCardColor::OrangeGold,
        NftCardId::Pumpkin => NftCardColor::Orange,
        NftCardId::PumpkinGold => NftCardColor::OrangeGold,
        NftCardId::Watermelon => NftCardColor::Purple,
        NftCardId::WatermelonGold => NftCardColor::PurpleGold,
        NftCardId::Cabbage => NftCardColor::Purple,
        NftCardId::CabbageGold => NftCardColor::PurpleGold,
        NftCardId::GrandpaBroccoli => NftCardColor::Purple,
        NftCardId::GrandpaBroccoliGold => NftCardColor::PurpleGold,
        NftCardId::MrTomato => NftCardColor::Purple,
        NftCardId::MrTomatoGold => NftCardColor::PurpleGold,
        NftCardId::DurianAgent => NftCardColor::Purple,
        NftCardId::DurianAgentGold => NftCardColor::PurpleGold,
        NftCardId::Avocado => NftCardColor::Purple,
        NftCardId::AvocadoGold => NftCardColor::PurpleGold,
        NftCardId::RedChili => NftCardColor::Blue,
        NftCardId::RedChiliGold => NftCardColor::BlueGold,
        NftCardId::Cactus => NftCardColor::Blue,
        NftCardId::CactusGold => NftCardColor::BlueGold,
        NftCardId::Pineapple => NftCardColor::Blue,
        NftCardId::PineappleGold => NftCardColor::BlueGold,
        NftCardId::Radish => NftCardColor::Blue,
        NftCardId::RadishGold => NftCardColor::BlueGold,
        NftCardId::Coconut => NftCardColor::Blue,
        NftCardId::CoconutGold => NftCardColor::BlueGold,
        NftCardId::MrCorn => NftCardColor::White,
        NftCardId::MrCornGold => NftCardColor::WhiteGold,
        NftCardId::PoisonMushroom => NftCardColor::White,
        NftCardId::PoisonMushroomGold => NftCardColor::WhiteGold,
        NftCardId::Apple => NftCardColor::White,
        NftCardId::AppleGold => NftCardColor::WhiteGold,
        NftCardId::HardshellNut => NftCardColor::White,
        NftCardId::HardshellNutGold => NftCardColor::WhiteGold,
        // Zombie
        NftCardId::CrazyDoctorZombie => NftCardColor::Orange,
        NftCardId::CrazyDoctorZombieGold => NftCardColor::OrangeGold,
        NftCardId::SuperZombie => NftCardColor::Orange,
        NftCardId::SuperZombieGold => NftCardColor::OrangeGold,
        NftCardId::PrinceZombie => NftCardColor::Orange,
        NftCardId::PrinceZombieGold => NftCardColor::OrangeGold,
        NftCardId::PirateZombie => NftCardColor::Purple,
        NftCardId::PirateZombieGold => NftCardColor::PurpleGold,
        NftCardId::PartyZombie => NftCardColor::Purple,
        NftCardId::PartyZombieGold => NftCardColor::PurpleGold,
        NftCardId::BarrelZombie => NftCardColor::Purple,
        NftCardId::BarrelZombieGold => NftCardColor::PurpleGold,
        NftCardId::StarZombie => NftCardColor::Purple,
        NftCardId::StarZombieGold => NftCardColor::PurpleGold,
        NftCardId::ClownZombie => NftCardColor::Purple,
        NftCardId::ClownZombieGold => NftCardColor::PurpleGold,
        NftCardId::ZombieBride => NftCardColor::Purple,
        NftCardId::ZombieBrideGold => NftCardColor::PurpleGold,
        NftCardId::BombZombie => NftCardColor::Blue,
        NftCardId::BombZombieGold => NftCardColor::BlueGold,
        NftCardId::CheerleaderZombie => NftCardColor::Blue,
        NftCardId::CheerleaderZombieGold => NftCardColor::BlueGold,
        NftCardId::RoadblockZombie => NftCardColor::Blue,
        NftCardId::RoadblockZombieGold => NftCardColor::BlueGold,
        NftCardId::DancerZombie => NftCardColor::Blue,
        NftCardId::DancerZombieGold => NftCardColor::BlueGold,
        NftCardId::MinerZombie => NftCardColor::Blue,
        NftCardId::MinerZombieGold => NftCardColor::BlueGold,
        NftCardId::SailorZombie => NftCardColor::White,
        NftCardId::SailorZombieGold => NftCardColor::WhiteGold,
        NftCardId::DeathKnight => NftCardColor::White,
        NftCardId::DeathKnightGold => NftCardColor::WhiteGold,
        NftCardId::ZombieBaby => NftCardColor::White,
        NftCardId::ZombieBabyGold => NftCardColor::WhiteGold,
        NftCardId::FarmerZombie => NftCardColor::White,
        NftCardId::FarmerZombieGold => NftCardColor::WhiteGold,
        // Neutral
        NftCardId::LegendaryMiracleEgg => NftCardColor::Orange,
        NftCardId::LegendaryMiracleEggGold => NftCardColor::OrangeGold,
        NftCardId::EpicMiracleEgg => NftCardColor::Purple,
        NftCardId::EpicMiracleEggGold => NftCardColor::PurpleGold,
        NftCardId::RiotShield => NftCardColor::Purple,
        NftCardId::RiotShieldGold => NftCardColor::PurpleGold,
        NftCardId::Poison => NftCardColor::Purple,
        NftCardId::PoisonGold => NftCardColor::PurpleGold,
        NftCardId::AmmoCrate => NftCardColor::Purple,
        NftCardId::AmmoCrateGold => NftCardColor::PurpleGold,
        NftCardId::StarvingMimic => NftCardColor::Purple,
        NftCardId::StarvingMimicGold => NftCardColor::PurpleGold,
        NftCardId::Rocket => NftCardColor::Purple,
        NftCardId::RocketGold => NftCardColor::PurpleGold,
        NftCardId::Trumpet => NftCardColor::Blue,
        NftCardId::TrumpetGold => NftCardColor::BlueGold,
        NftCardId::IronHelmet => NftCardColor::Blue,
        NftCardId::IronHelmetGold => NftCardColor::BlueGold,
        NftCardId::InflatableHammer => NftCardColor::Blue,
        NftCardId::InflatableHammerGold => NftCardColor::BlueGold,
        NftCardId::FryingPan => NftCardColor::Blue,
        NftCardId::FryingPanGold => NftCardColor::BlueGold,
        NftCardId::RagingShield => NftCardColor::Blue,
        NftCardId::RagingShieldGold => NftCardColor::BlueGold,
        NftCardId::UFO => NftCardColor::Blue,
        NftCardId::UFOGold => NftCardColor::BlueGold,
        NftCardId::Turner => NftCardColor::Blue,
        NftCardId::TurnerGold => NftCardColor::BlueGold,
        NftCardId::Cannon => NftCardColor::White,
        NftCardId::CannonGold => NftCardColor::WhiteGold,
        NftCardId::Chainsaw => NftCardColor::White,
        NftCardId::ChainsawGold => NftCardColor::WhiteGold,
        NftCardId::Shotgun => NftCardColor::White,
        NftCardId::ShotgunGold => NftCardColor::WhiteGold,
        NftCardId::SpikedBat => NftCardColor::White,
        NftCardId::SpikedBatGold => NftCardColor::WhiteGold,
        NftCardId::Broom => NftCardColor::White,
        NftCardId::BroomGold => NftCardColor::WhiteGold,
        NftCardId::BoxingGloves => NftCardColor::White,
        NftCardId::BoxingGlovesGold => NftCardColor::WhiteGold,
        NftCardId::Axe => NftCardColor::White,
        NftCardId::AxeGold => NftCardColor::WhiteGold,

        // Dragon
        NftCardId::RadiantDragon => NftCardColor::Orange,
        NftCardId::RadiantDragonGold => NftCardColor::OrangeGold,
        NftCardId::ThunderDragon => NftCardColor::Orange,
        NftCardId::ThunderDragonGold => NftCardColor::OrangeGold,
        NftCardId::OceanDragon => NftCardColor::Orange,
        NftCardId::OceanDragonGold => NftCardColor::OrangeGold,
        NftCardId::TerraDragon => NftCardColor::Orange,
        NftCardId::TerraDragonGold => NftCardColor::OrangeGold,
        NftCardId::StormDragon => NftCardColor::Purple,
        NftCardId::StormDragonGold => NftCardColor::PurpleGold,
        NftCardId::RockDragon => NftCardColor::Purple,
        NftCardId::RockDragonGold => NftCardColor::PurpleGold,
        NftCardId::ShadowDragon => NftCardColor::Purple,
        NftCardId::ShadowDragonGold => NftCardColor::PurpleGold,
        NftCardId::NatureDragon => NftCardColor::Purple,
        NftCardId::NatureDragonGold => NftCardColor::PurpleGold,
        NftCardId::DeceptionDragon => NftCardColor::Purple,
        NftCardId::DeceptionDragonGold => NftCardColor::PurpleGold,
        NftCardId::CrystalDragon => NftCardColor::Purple,
        NftCardId::CrystalDragonGold => NftCardColor::PurpleGold,
        NftCardId::DragonEgg => NftCardColor::Purple,
        NftCardId::DragonEggGold => NftCardColor::PurpleGold,
        NftCardId::FrostDragon => NftCardColor::Purple,
        NftCardId::FrostDragonGold => NftCardColor::PurpleGold,
        NftCardId::SteelDragon => NftCardColor::Blue,
        NftCardId::SteelDragonGold => NftCardColor::BlueGold,
        NftCardId::InfernoDragon => NftCardColor::Blue,
        NftCardId::InfernoDragonGold => NftCardColor::BlueGold,
        NftCardId::ObsidianDragon => NftCardColor::Blue,
        NftCardId::ObsidianDragonGold => NftCardColor::BlueGold,
        NftCardId::WindDragon => NftCardColor::Blue,
        NftCardId::WindDragonGold => NftCardColor::BlueGold,
        NftCardId::NaughtyDragon => NftCardColor::White,
        NftCardId::NaughtyDragonGold => NftCardColor::WhiteGold,
        NftCardId::LavaDragon => NftCardColor::White,
        NftCardId::LavaDragonGold => NftCardColor::WhiteGold,

        // Mech
        NftCardId::GT130 => NftCardColor::Orange,
        NftCardId::GT130Gold => NftCardColor::OrangeGold,
        NftCardId::ChargeBot => NftCardColor::Orange,
        NftCardId::ChargeBotGold => NftCardColor::OrangeGold,
        NftCardId::GuardianBot => NftCardColor::Orange,
        NftCardId::GuardianBotGold => NftCardColor::OrangeGold,
        NftCardId::DeterrentBot => NftCardColor::Orange,
        NftCardId::DeterrentBotGold => NftCardColor::OrangeGold,
        NftCardId::DoggyBot => NftCardColor::Purple,
        NftCardId::DoggyBotGold => NftCardColor::PurpleGold,
        NftCardId::MechHatchery => NftCardColor::Purple,
        NftCardId::MechHatcheryGold => NftCardColor::PurpleGold,
        NftCardId::AvengerD0 => NftCardColor::Purple,
        NftCardId::AvengerD0Gold => NftCardColor::PurpleGold,
        NftCardId::ScoutBot => NftCardColor::Purple,
        NftCardId::ScoutBotGold => NftCardColor::PurpleGold,
        NftCardId::MechanicKnight => NftCardColor::Purple,
        NftCardId::MechanicKnightGold => NftCardColor::PurpleGold,
        NftCardId::SupportBot => NftCardColor::Purple,
        NftCardId::SupportBotGold => NftCardColor::PurpleGold,
        NftCardId::KittyBot => NftCardColor::Purple,
        NftCardId::KittyBotGold => NftCardColor::PurpleGold,
        NftCardId::AlarmBot => NftCardColor::Purple,
        NftCardId::AlarmBotGold => NftCardColor::PurpleGold,
        NftCardId::ES67 => NftCardColor::Blue,
        NftCardId::ES67Gold => NftCardColor::BlueGold,
        NftCardId::RobotDuke => NftCardColor::Blue,
        NftCardId::RobotDukeGold => NftCardColor::BlueGold,
        NftCardId::HelperBot => NftCardColor::Blue,
        NftCardId::HelperBotGold => NftCardColor::BlueGold,
        NftCardId::Stone => NftCardColor::Blue,
        NftCardId::StoneGold => NftCardColor::BlueGold,
        NftCardId::MiniRobot => NftCardColor::White,
        NftCardId::MiniRobotGold => NftCardColor::WhiteGold,
        NftCardId::SturdyBot => NftCardColor::White,
        NftCardId::SturdyBotGold => NftCardColor::WhiteGold,
    }
}

pub fn get_card_by_value(value: u32) -> Option<&'static NftCard> {
    match NFT_CARD_MAP.get() {
        Some(map) => map.get(&value),
        None => None,
    }
}

pub fn get_card_by_id(id: NftCardId) -> Option<&'static NftCard> {
    match NFT_CARD_MAP.get() {
        Some(cards) => cards.get(&(id as u32)),
        None => None,
    }
}

pub fn get_card_by_name(name: &str) -> Option<&'static NftCard> {
    match NFT_CARD_MAP.get() {
        Some(map) => {
            let find_card = map.iter().filter(|(_, card)| card.name_en == name);
            match find_card.last() {
                Some(card) => Some(card.1),
                None => None,
            }
        }
        None => None,
    }
}

pub fn filter_cards(
    category: Option<HashSet<NftCardCategory>>,
    colors: Option<HashSet<NftCardColor>>,
) -> Option<Vec<&'static NftCard>> {
    match NFT_CARD_MAP.get() {
        Some(cards) => Some(
            cards
                .values()
                .filter(|card| {
                    let category_match = match &category {
                        Some(cat_set) => cat_set.is_empty() || cat_set.contains(&card.category),
                        None => true,
                    };
                    let color_match = colors.as_ref().map_or(true, |color_set| {
                        color_set.is_empty() || color_set.contains(&card.color)
                    });
                    category_match && color_match
                })
                .collect(),
        ),
        None => None,
    }
}
pub fn filter_cards_id_only(
    category: Option<HashSet<NftCardCategory>>,
    colors: Option<HashSet<NftCardColor>>,
) -> Result<Vec<NftCardId>> {
    match NFT_CARD_MAP.get() {
        Some(cards) => Ok(cards
            .values()
            .filter(|card| {
                let category_match = match &category {
                    Some(cat_set) => cat_set.is_empty() || cat_set.contains(&card.category),
                    None => true,
                };
                let color_match = colors.as_ref().map_or(true, |color_set| {
                    color_set.is_empty() || color_set.contains(&card.color)
                });
                category_match && color_match
            })
            .map(|card| card.id)
            .collect()),
        None => Err(anyhow::anyhow!("Cards cannot found")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nft_card_id() {
        let id = 320;
        let lang = "en";

        if let Some(card) = NftCardId::get_name_by_value(id, lang) {
            assert_eq!(card, "Ammo Crate")
        }
    }
    #[test]
    fn test_nft_card_id_none() {
        let id = 1;
        let lang = "en";

        assert_eq!(NftCardId::get_name_by_value(id, lang), None)
    }
    #[test]
    fn test_nft_card_id_en() {
        let id = 500;
        let lang = "en";

        if let Some(card) = NftCardId::get_name_by_value(id, lang) {
            assert_eq!(card, "Alarm Bot")
        }
    }

    #[test]
    fn test_nft_card_id_zh() {
        let id = 320;
        let lang = "zh";

        if let Some(card) = NftCardId::get_name_by_value(id, lang) {
            assert_eq!(card, "弹药箱".to_string())
        }
    }
}
