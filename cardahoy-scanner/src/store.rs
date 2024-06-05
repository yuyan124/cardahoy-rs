use anyhow::Result;
use api::market_secondary::Secondary;
use api::nft::{filter_cards, NftCardColor};
use cardahoy_api as api;
use cardahoy_api::{
    market_home::MarketHomeResponse,
    market_secondary::MarketSecondaryResponse,
    nft::{get_card_by_name, NftCardId, NftId, NftSortType},
    CardsAhoyApi,
};
use cardahoy_utils as utils;
use comfy_table::Table;
use csv::ReaderBuilder;
use futures::stream::StreamExt;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use tokio::time::Duration;

const CARD_NUMBER_GREATER: usize = 10;
const CARD_TOP_N: usize = 5;
const CARD_GOLD_TOP_N: usize = 3;

pub fn read_csv(filename: &str) -> Result<HashMap<u32, f64>> {
    let mut map: HashMap<u32, f64> = HashMap::new();
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_path(filename)?;

    for result in rdr.deserialize() {
        let record: FloorPriceRecord = result?;
        map.insert(
            get_card_by_name(&record.key)
                .ok_or(anyhow::anyhow!(format!("{} not found", record.key)))?
                .value,
            record.value,
        );
    }
    Ok(map)
}

#[derive(Debug, Deserialize)]
pub struct FloorPriceRecord {
    key: String,
    value: f64,
}

pub struct Store {
    pub api: CardsAhoyApi,
    pub prices: HashMap<u32, f64>,
    pub config: utils::Config,
    pub cards_number: u32,
    pub discrete_list: Vec<api::filter::Discrete>,
}

impl Store {
    pub fn new() -> Result<Self> {
        let api = CardsAhoyApi::new()?;
        let prices: HashMap<u32, f64> = read_csv("neutral.csv")?;
        let config = utils::Config::new();

        let discrete_list = vec![
            api::filter::Discrete::with_none(api::filter::DiscreteFilter::Type),
            api::filter::Discrete::faction(config.faction.clone()),
            api::filter::Discrete::rarity(config.rarity.clone()),
            api::filter::Discrete::foil(config.foil.clone()),
            api::filter::Discrete::with_none(api::filter::DiscreteFilter::Source),
        ];

        let categories: HashSet<api::nft::NftCardCategory> = config
            .faction
            .iter()
            .map(|faction| {
                let faction = faction.as_str();
                match faction {
                    "Neutral" => api::nft::NftCardCategory::Neutral,
                    "Zombie" => api::nft::NftCardCategory::Zombie,
                    "Dragon" => api::nft::NftCardCategory::Dragon,
                    "Mech" => api::nft::NftCardCategory::Mech,
                    "Animal" => api::nft::NftCardCategory::Animal,
                    "Plant" => api::nft::NftCardCategory::Plant,
                    _ => api::nft::NftCardCategory::Neutral,
                }
            })
            .collect();

        let colors: HashSet<api::nft::NftCardColor> = config
            .foil
            .iter()
            .flat_map(|foil| {
                config.rarity.iter().map(move |rarity| {
                    let foil = foil.as_str();
                    let rarity = rarity.as_str();
                    match (foil, rarity) {
                        ("Regular", "Common") => NftCardColor::White,
                        ("Regular", "Rare") => NftCardColor::Blue,
                        ("Regular", "Epic") => NftCardColor::Purple,
                        ("Regular", "Legendary") => NftCardColor::Orange,
                        ("Gold", "Common") => NftCardColor::WhiteGold,
                        ("Gold", "Rare") => NftCardColor::BlueGold,
                        ("Gold", "Epic") => NftCardColor::PurpleGold,
                        ("Gold", "Legendary") => NftCardColor::OrangeGold,
                        _ => NftCardColor::White,
                    }
                })
            })
            .collect();

        let cards = filter_cards(Some(categories), Some(colors)).unwrap();

        Ok(Self {
            api,
            prices,
            config,
            cards_number: cards.len() as u32,
            discrete_list,
        })
    }

    pub async fn scan_cards_full_secondary(&self) -> Result<()> {
        let discrete_list = vec![
            api::filter::Discrete::filter_type(vec![]),
            api::filter::Discrete::faction(vec![]),
            api::filter::Discrete::rarity(vec![]),
            api::filter::Discrete::foil(vec![]),
            api::filter::Discrete::source(vec![]),
        ];

        let resp = self
            .api
            .query_market_secondary(
                NftId::Cards,
                1,
                222,
                api::nft::NftSortType::PriceDescending,
                &discrete_list,
            )
            .await?;

        let mut table = Table::new();
        table.set_header(vec!["卡牌名称", "ID", "最低价格", "在售数量", "成交数量"]);
        for card in resp.list {
            table.add_row(vec![
                NftCardId::get_name_by_value(card.secondary_id, "zh").unwrap(),
                card.secondary_id.to_string(),
                card.floor_price.to_string(),
                card.quantity.to_string(),
                card.volume.to_string(),
            ]);
        }
        println!("{table}");

        Ok(())
    }

    pub async fn scan_cards_full_home(&self) -> Result<()> {
        // let mut colors = HashSet::new();
        // colors.insert(api::nft::NftCardColor::Orange);
        // colors.insert(api::nft::NftCardColor::Purple);
        // colors.insert(api::nft::NftCardColor::Blue);

        let mut categories = HashSet::new();
        categories.insert(api::nft::NftCardCategory::Animal);
        let animals = api::nft::filter_cards_id_only(Some(categories), None)?;

        let mut categories = HashSet::new();
        categories.insert(api::nft::NftCardCategory::Plant);
        let plants = api::nft::filter_cards_id_only(Some(categories), None)?;

        let mut categories = HashSet::new();
        categories.insert(api::nft::NftCardCategory::Zombie);
        let zombies = api::nft::filter_cards_id_only(Some(categories), None)?;

        let mut categories = HashSet::new();
        categories.insert(api::nft::NftCardCategory::Neutral);
        let neutrals = api::nft::filter_cards_id_only(Some(categories), None)?;

        let mut categories = HashSet::new();
        categories.insert(api::nft::NftCardCategory::Dragon);
        let dragons = api::nft::filter_cards_id_only(Some(categories), None)?;

        let mut categories = HashSet::new();
        categories.insert(api::nft::NftCardCategory::Mech);
        let mechs = api::nft::filter_cards_id_only(Some(categories), None)?;

        let interests: Vec<NftCardId> = vec![animals, dragons, mechs, plants, zombies, neutrals]
            .into_iter()
            .flatten()
            .collect();

        let mut stream = futures::stream::iter(interests.into_iter())
            .map(|interest| self.scan_card_home(interest as u32))
            .buffered(10);

        while let Some(result) = stream.next().await {
            if let Err(e) = result {
                println!("Error: {}", e);
            }
        }

        Ok(())
    }

    pub async fn buy_nft_strategy_1(&self) -> Result<()> {
        // 扫描某种卡牌的金卡和普通卡
        // 查看普通卡价格，最便宜5张的均价，扫描金卡价格，金卡的价格小于普卡均价的1.1倍，直接购买金卡

        let animals = vec![
            (NftCardId::WolfKing, NftCardId::WolfKingGold),
            (NftCardId::MasterPanda, NftCardId::MasterPandaGold),
            (NftCardId::LionKing, NftCardId::LionKingGold),
            (NftCardId::FuriousTiger, NftCardId::FuriousTigerGold),
            (NftCardId::CunningFox, NftCardId::CunningFoxGold),
            (NftCardId::Frog, NftCardId::FrogGold),
            (NftCardId::Snake, NftCardId::SnakeGold),
            (NftCardId::AngryBull, NftCardId::AngryBullGold),
            (NftCardId::TortoiseBro, NftCardId::TortoiseBroGold),
            (NftCardId::CunningFox, NftCardId::CunningFoxGold),
        ];
        let zombies = vec![
            (
                NftCardId::CrazyDoctorZombie,
                NftCardId::CrazyDoctorZombieGold,
            ),
            (NftCardId::SuperZombie, NftCardId::SuperZombieGold),
            (NftCardId::PirateZombie, NftCardId::PirateZombieGold),
            (NftCardId::PartyZombie, NftCardId::PartyZombieGold),
            (NftCardId::BarrelZombie, NftCardId::BarrelZombieGold),
            (NftCardId::PrinceZombie, NftCardId::PrinceZombieGold),
            (NftCardId::StarZombie, NftCardId::StarZombieGold),
            (NftCardId::ClownZombie, NftCardId::ClownZombieGold),
            (NftCardId::BombZombie, NftCardId::BombZombieGold),
            (
                NftCardId::CheerleaderZombie,
                NftCardId::CheerleaderZombieGold,
            ),
            (NftCardId::RoadblockZombie, NftCardId::RoadblockZombieGold),
            (NftCardId::DancerZombie, NftCardId::DancerZombieGold),
            (NftCardId::ZombieBride, NftCardId::ZombieBrideGold),
        ];
        let plants = vec![
            (NftCardId::DurianCommander, NftCardId::DurianCommanderGold),
            (NftCardId::Garlic, NftCardId::GarlicGold),
            (NftCardId::Pumpkin, NftCardId::PumpkinGold),
            (NftCardId::Cabbage, NftCardId::CabbageGold),
            (NftCardId::Watermelon, NftCardId::WatermelonGold),
            (NftCardId::GrandpaBroccoli, NftCardId::GrandpaBroccoliGold),
            (NftCardId::MrTomato, NftCardId::MrTomatoGold),
            (NftCardId::Avocado, NftCardId::AvocadoGold),
            (NftCardId::Pumpkin, NftCardId::PumpkinGold),
            (NftCardId::DurianAgent, NftCardId::DurianAgentGold),
            (NftCardId::Pineapple, NftCardId::PineappleGold),
            (NftCardId::RedChili, NftCardId::RedChiliGold),
            (NftCardId::Cactus, NftCardId::CactusGold),
            (NftCardId::Radish, NftCardId::RadishGold),
        ];
        let _neutrals = vec![
            (
                NftCardId::LegendaryMiracleEgg,
                NftCardId::LegendaryMiracleEggGold,
            ),
            (NftCardId::EpicMiracleEgg, NftCardId::EpicMiracleEggGold),
            (NftCardId::RiotShield, NftCardId::RiotShieldGold),
            (NftCardId::Poison, NftCardId::PoisonGold),
            (NftCardId::AmmoCrate, NftCardId::AmmoCrateGold),
            (NftCardId::StarvingMimic, NftCardId::StarvingMimicGold),
            (NftCardId::Rocket, NftCardId::RocketGold),
            (NftCardId::Trumpet, NftCardId::TrumpetGold),
            (NftCardId::IronHelmet, NftCardId::IronHelmetGold),
        ];

        let _dragons = vec![
            (NftCardId::RadiantDragon, NftCardId::RadiantDragonGold),
            (NftCardId::ThunderDragon, NftCardId::ThunderDragonGold),
            (NftCardId::RadiantDragon, NftCardId::RadiantDragonGold),
            (NftCardId::OceanDragon, NftCardId::OceanDragonGold),
            (NftCardId::StormDragon, NftCardId::StormDragonGold),
            (NftCardId::RockDragon, NftCardId::RockDragonGold),
            (NftCardId::TerraDragon, NftCardId::TerraDragonGold),
            (NftCardId::NatureDragon, NftCardId::NatureDragonGold),
            (NftCardId::ShadowDragon, NftCardId::ShadowDragonGold),
            (NftCardId::RockDragon, NftCardId::RockDragonGold),
            (NftCardId::DeceptionDragon, NftCardId::DeceptionDragonGold),
            (NftCardId::TerraDragon, NftCardId::TerraDragonGold),
            (NftCardId::DeceptionDragon, NftCardId::DeceptionDragonGold),
            (NftCardId::CrystalDragon, NftCardId::CrystalDragonGold),
            (NftCardId::NatureDragon, NftCardId::NatureDragonGold),
            (NftCardId::DragonEgg, NftCardId::DragonEggGold),
            (NftCardId::FrostDragon, NftCardId::FrostDragonGold),
            (NftCardId::SteelDragon, NftCardId::SteelDragonGold),
            (NftCardId::InfernoDragon, NftCardId::InfernoDragonGold),
            (NftCardId::ObsidianDragon, NftCardId::ObsidianDragonGold),
        ];

        let _mechs = vec![
            (NftCardId::ChargeBot, NftCardId::ChargeBotGold),
            (NftCardId::GuardianBot, NftCardId::GuardianBotGold),
            // NftCardId::GT130,
            // NftCardId::GT130Gold,
            (NftCardId::DoggyBot, NftCardId::DoggyBotGold),
            (NftCardId::AvengerD0, NftCardId::AvengerD0Gold),
            (NftCardId::DeterrentBot, NftCardId::DeterrentBotGold),
            (NftCardId::MechHatchery, NftCardId::MechHatcheryGold),
            (NftCardId::DoggyBot, NftCardId::DoggyBotGold),
            (NftCardId::MechHatchery, NftCardId::MechHatcheryGold),
            (NftCardId::MechanicKnight, NftCardId::MechanicKnightGold),
            (NftCardId::KittyBot, NftCardId::KittyBotGold),
            (NftCardId::ScoutBot, NftCardId::ScoutBotGold),
            (NftCardId::SupportBot, NftCardId::SupportBotGold),
            (NftCardId::AlarmBot, NftCardId::AlarmBotGold),
        ];

        let card_pairs: HashMap<NftCardId, NftCardId> = vec![animals, zombies, plants]
            .into_iter()
            .flatten()
            .collect();

        let mut stream = futures::stream::iter(card_pairs.into_iter())
            .map(|(key, value)| async move {
                println!("[扫描]: {}", key.to_chinese());
                let card = self.scan_card_home(key as u32).await?;

                tokio::time::sleep(Duration::from_secs(1)).await;
                println!("[扫描]: {}", value.to_chinese());
                let card_gold = self.scan_card_home(value as u32).await?;
                Ok::<_, anyhow::Error>((key, value, card, card_gold))
            })
            .buffered(10);

        while let Some(result) = stream.next().await {
            match result {
                Ok((key, value, card, card_gold)) => {
                    // 普卡卡片数量不得低于5张
                    if card.list.len() >= CARD_NUMBER_GREATER {
                        self.process_card_gold(key, value, card, card_gold).await?;
                    }
                }
                Err(e) => {
                    println!("{:?}", e);
                }
            }
        }

        Ok(())
    }

    async fn process_card_gold(
        &self,
        key: NftCardId,
        value: NftCardId,
        card: MarketHomeResponse,
        card_gold: MarketHomeResponse,
    ) -> Result<()> {
        tracing::info!("[分析]: {} -> {}", value.to_chinese(), key.to_chinese());
        let sum: f64 = card
            .list
            .iter()
            .take(CARD_TOP_N)
            .filter_map(|ci| {
                let price = ci
                    .metadata_list
                    .iter()
                    .find(|meta| meta.name == "Price/EXP")
                    .map(|meta| meta.value.parse::<f64>().unwrap())
                    .unwrap_or_else(|| {
                        ci.sale_price.parse::<f64>().unwrap() / ci.accumulate_trait.value as f64
                    });
                Some(price)
            })
            .sum();
        let avg = sum / CARD_TOP_N as f64;

        // 前3张金卡
        let len = card_gold.list.len();
        let take_count = std::cmp::min(len, CARD_GOLD_TOP_N);

        let _buys = futures::stream::iter(card_gold.list.iter().take(take_count))
            .then(|cg| async move {
                if let Some(price) = cg
                    .metadata_list
                    .iter()
                    .find(|meta| meta.name == "Price/EXP")
                    .and_then(|meta| meta.value.parse::<f64>().ok())
                {
                    if price < avg * 1.1 {
                        let name_gold = NftCardId::get_name_by_id(value, "cn");
                        println!(
                            "[{}]: Found cheap card. Price:{}, avg:{}",
                            name_gold, &cg.sale_price, &avg
                        );
                        if let Ok(_) = self.api.buy_ntf_asset(&cg.sale_aggregator_number).await {
                            println!("[{}]: buy success.", value.to_chinese());
                            // tracing::info!(
                            //     "[{}]: Found cheap card. Price:{}",
                            //     name_gold,
                            //     cg.sale_price
                            // );
                        }
                    }
                }
            })
            .collect::<Vec<_>>()
            .await;

        Ok(())
    }

    pub async fn buy_nft_strategy_with_five_avg(&self) -> Result<()> {
        // 扫描卡牌前5的平均价格，如果第一张卡牌是平均价格50%以下，直接购买此卡牌

        let mut colors = HashSet::new();
        colors.insert(api::nft::NftCardColor::Purple);
        colors.insert(api::nft::NftCardColor::Blue);
        colors.insert(api::nft::NftCardColor::White);

        let mut categories = HashSet::new();
        categories.insert(api::nft::NftCardCategory::Neutral);

        let neutrals = api::nft::filter_cards_id_only(Some(categories), None)?;

        let mut stream = futures::stream::iter(neutrals.into_iter())
            .map(|neutral| {
                println!("[扫描]: {}", neutral.to_chinese());
                self.scan_card_home(neutral as u32)
            })
            .buffered(10);

        while let Some(result) = stream.next().await {
            match result {
                Ok(cards) => {
                    let sum: f64 = cards
                        .list
                        .iter()
                        .take(5)
                        .filter_map(|ci| {
                            let price = ci
                                .metadata_list
                                .iter()
                                .find(|meta| meta.name == "Price/EXP")
                                .map(|meta| meta.value.parse::<f64>().unwrap())
                                .unwrap_or_else(|| {
                                    ci.sale_price.parse::<f64>().unwrap()
                                        / ci.accumulate_trait.value as f64
                                });
                            Some(price)
                        })
                        .sum();

                    let avg = sum / CARD_TOP_N as f64;
                    if let Some(first_card) = cards.list.first() {
                        let price = first_card
                            .metadata_list
                            .iter()
                            .find(|meta| meta.name == "Price/EXP")
                            .map(|meta| meta.value.parse::<f64>().unwrap())
                            .unwrap_or_else(|| {
                                first_card.sale_price.parse::<f64>().unwrap()
                                    / first_card.accumulate_trait.value as f64
                            });
                        if price <= avg * 0.5 {
                            if let Ok(_) = self
                                .api
                                .buy_ntf_asset(&first_card.sale_aggregator_number)
                                .await
                            {
                                println!(
                                    "[{}]: buy success. 购买价格:{}, 均价是:{}",
                                    &first_card.nft_name, &first_card.sale_price, &avg
                                );
                            }
                        }
                    }
                }
                Err(e) => {
                    println!("{:?}", e);
                }
            }
        }
        Ok(())
    }

    pub async fn buy_nft_strategy_full(&self) -> Result<()> {
        tracing::info!("[扫描]: start...");

        let neutrals = self.scan_neutrals().await?;
        let prices: &HashMap<u32, f64> = &self.prices;
        let found_neutrals: Vec<&Secondary> = neutrals
            .list
            .iter()
            .filter(|&neutral| {
                if let Some(price) = prices.get(&neutral.secondary_id) {
                    neutral.floor_price.parse::<f64>().unwrap() <= *price
                } else {
                    false
                }
            })
            .collect();

        let mut stream = futures::stream::iter(found_neutrals.into_iter())
            .map(|neutral| async move {
                let card = NftCardId::from_id(neutral.secondary_id)
                    .map_or(None, |x| Some(x))
                    .unwrap();
                tracing::info!(
                    "[扫描]: 发现便宜卡片，{}, 价格:{}",
                    card.to_chinese(),
                    neutral.floor_price
                );
                let cards = self.scan_card_home(neutral.secondary_id).await?;
                Ok::<_, anyhow::Error>((cards, neutral))
            })
            .buffered(3);
        while let Some(result) = stream.next().await {
            match result {
                Ok((cards, neutral)) => {
                    if let Some(first_card) = cards.list.first() {
                        let unit_price = first_card
                            .metadata_list
                            .iter()
                            .find(|meta| meta.name == "Price/EXP")
                            .map(|meta| meta.value.parse::<f64>().unwrap())
                            .unwrap_or_else(|| {
                                first_card.sale_price.parse::<f64>().unwrap()
                                    / first_card.accumulate_trait.value as f64
                            });

                        if let Ok(floor_price) = neutral.floor_price.parse::<f64>() {
                            if unit_price <= floor_price {
                                tracing::info!("[交易]: 准备购买{}", first_card.nft_name);
                                if let Ok(_) = self
                                    .api
                                    .buy_ntf_asset(&first_card.sale_aggregator_number)
                                    .await
                                {
                                    tracing::info!(
                                        "[交易]: 购买{}成功, 价格为{}",
                                        first_card.nft_name,
                                        first_card.sale_price
                                    );
                                } else {
                                    tracing::error!("[交易]: 购买{}失败", first_card.nft_name);
                                }
                            }
                        }
                    }
                }
                Err(ref e) => {
                    println!("{:?}", &result);
                    println!("{:?}", e);
                }
            }
        }
        Ok(())
    }

    async fn scan_category_cards(
        &self,
        discrete_list: Option<Vec<api::filter::Discrete>>,
    ) -> Result<MarketSecondaryResponse> {
        let page = 1;
        let sort_type = NftSortType::PriceAscending;
        let resp = if let Some(list) = discrete_list {
            self.api
                .query_market_secondary(NftId::Cards, page, self.cards_number, sort_type, &list)
                .await?
        } else {
            self.api
                .query_market_secondary(
                    NftId::Cards,
                    page,
                    self.cards_number,
                    sort_type,
                    &self.discrete_list,
                )
                .await?
        };
        Ok(resp)
    }

    pub async fn scan_card_home(&self, card_id: u32) -> Result<MarketHomeResponse> {
        let resp = self
            .api
            .query_market_home(
                NftId::Cards,
                card_id,
                1,
                api::market_home::MarketHomeSortType::PriceExpAscending,
            )
            .await?;

        Ok(resp)
    }

    pub async fn scan_neutrals(&self) -> Result<MarketSecondaryResponse> {
        let discrete_list = vec![
            api::filter::Discrete::filter_type(vec![]),
            api::filter::Discrete::faction(vec!["Neutral".into()]),
            api::filter::Discrete::rarity(vec!["Common".into()]),
            api::filter::Discrete::foil(vec!["Regular".into(), "Gold".into()]),
            api::filter::Discrete::source(vec![]),
        ];
        let resp = self.scan_category_cards(Some(discrete_list)).await?;
        Ok(resp)
    }

    async fn find_cheap_cards(&self, cards: MarketSecondaryResponse) -> Vec<Secondary> {
        let prices = &self.prices;
        cards
            .list
            .into_iter()
            .filter(|card| {
                if let Some(price) = prices.get(&card.secondary_id) {
                    card.floor_price.parse::<f64>().unwrap() <= *price
                } else {
                    false
                }
            })
            .collect()
    }

    async fn buy_cheap_cards(&self, cheap_cards: Vec<Secondary>) -> Result<()> {
        let mut stream = futures::stream::iter(cheap_cards.into_iter())
            .map(|cheap_card| async move {
                let card = NftCardId::from_id(cheap_card.secondary_id)
                    .map_or(None, |x| Some(x))
                    .unwrap();
                tracing::info!(
                    "[扫描]: 发现便宜卡片，{}, 价格:{}",
                    card.to_chinese(),
                    cheap_card.floor_price
                );
                let cards = self.scan_card_home(cheap_card.secondary_id).await?;
                Ok::<_, anyhow::Error>((cards, cheap_card))
            })
            .buffered(3);

        while let Some(result) = stream.next().await {
            match result {
                Ok((cards, cheap_card)) => {
                    if let Some(first_card) = cards.list.first() {
                        if first_card.accumulate_trait.value > 3 {
                            tracing::info!("卡牌等级大于{}", first_card.accumulate_trait.value);
                            continue;
                        }
                        // 如果卡片，有含有等级的卡片，则计算卡片实际的单价，即总价/卡片经验
                        let unit_price = first_card
                            .metadata_list
                            .iter()
                            .find(|meta| meta.name == "Price/EXP")
                            .map(|meta| meta.value.parse::<f64>().unwrap())
                            .unwrap_or_else(|| {
                                first_card.sale_price.parse::<f64>().unwrap()
                                    / first_card.accumulate_trait.value as f64
                            });

                        let exp = first_card.accumulate_trait.value as f64;

                        // 使用了Buffer容器，可能会导致，同一张卡片，被扫描几次后统一处理。
                        // 所以在这里，对卡片价格做一个二次检测。
                        if let Ok(floor_price) = cheap_card.floor_price.parse::<f64>() {
                            if unit_price <= floor_price {
                                tracing::info!("[交易]: 准备购买{}", first_card.nft_name);
                                // if let Ok(_) = self
                                //     .api
                                //     .buy_ntf_asset(&first_card.sale_aggregator_number)
                                //     .await
                                // {
                                //     tracing::info!(
                                //         "[交易]: 购买{}成功, 价格为{}",
                                //         first_card.nft_name,
                                //         first_card.sale_price
                                //     );
                                // } else {
                                //     tracing::error!("[交易]: 购买{}失败", first_card.nft_name);
                                // }
                                let resp = self
                                    .api
                                    .buy_ntf_asset(&first_card.sale_aggregator_number)
                                    .await;
                                match resp {
                                    Ok(_) => {
                                        tracing::info!(
                                            "[交易]: 购买{}成功, 价格为{}",
                                            first_card.nft_name,
                                            first_card.sale_price
                                        );
                                    }
                                    Err(e) => {
                                        tracing::error!("[交易]: 购买{}失败", first_card.nft_name);
                                        tracing::error!("{:?}", e);
                                    }
                                }
                            }
                        }
                    }
                }
                Err(ref e) => {
                    println!("{:?}", &result);
                    println!("{:?}", e);
                }
            }
        }
        Ok(())
    }

    pub async fn filter_scan(&self) -> Result<()> {
        tracing::info!("[扫描]: start...");
        let discrete_list = self.discrete_list.clone();
        let cards = self.scan_category_cards(Some(discrete_list)).await?;
        let cheap_cards = self.find_cheap_cards(cards).await;
        self.buy_cheap_cards(cheap_cards).await?;
        Ok(())
    }
    pub async fn custom_scan_full(&self) -> Result<()> {
        tracing::info!("[扫描]: start...");
        let cards = self.scan_category_cards(None).await?;
        let cheap_cards = self.find_cheap_cards(cards).await;
        self.buy_cheap_cards(cheap_cards).await?;
        Ok(())
    }
}
