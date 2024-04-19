use serde::{Deserialize, Serialize};
use std::sync::OnceLock;
use strum_macros::{Display, EnumIter, EnumString};

static DISCRETE_FILTER_TYPE: OnceLock<Vec<DiscreteFilterValue>> = OnceLock::new();
static DISCRETE_FILTER_FRACTION: OnceLock<Vec<DiscreteFilterValue>> = OnceLock::new();
static DISCRETE_FILTER_RARITY: OnceLock<Vec<DiscreteFilterValue>> = OnceLock::new();
static DISCRETE_FILTER_FOIL: OnceLock<Vec<DiscreteFilterValue>> = OnceLock::new();
static DISCRETE_FILTER_SOURCE: OnceLock<Vec<DiscreteFilterValue>> = OnceLock::new();

pub fn get_discrete_filter(filter: DiscreteFilter) -> &'static Vec<DiscreteFilterValue> {
    match filter {
        DiscreteFilter::Type => DISCRETE_FILTER_TYPE.get_or_init(|| {
            vec![
                DiscreteFilterValue::new("Leaders", "Leaders"),
                DiscreteFilterValue::new("Members", "Members"),
            ]
        }),
        DiscreteFilter::Faction => DISCRETE_FILTER_FRACTION.get_or_init(|| {
            vec![
                DiscreteFilterValue::new("Neutral", "Neutral"),
                DiscreteFilterValue::new("Animal", "Animal"),
                DiscreteFilterValue::new("Plant", "Plant"),
                DiscreteFilterValue::new("Zombie", "Zombie"),
                DiscreteFilterValue::new("Mech", "Mech"),
                DiscreteFilterValue::new("Dragon", "Dragon"),
            ]
        }),
        DiscreteFilter::Rarity => DISCRETE_FILTER_RARITY.get_or_init(|| {
            vec![
                DiscreteFilterValue::new("Common", "Common"),
                DiscreteFilterValue::new("Rare", "Rare"),
                DiscreteFilterValue::new("Epic", "Epic"),
                DiscreteFilterValue::new("Legendary", "Legendary"),
            ]
        }),
        DiscreteFilter::Foil => DISCRETE_FILTER_FOIL.get_or_init(|| {
            vec![
                DiscreteFilterValue::new("Regular", "Regular"),
                DiscreteFilterValue::new("Gold", "Gold"),
            ]
        }),
        DiscreteFilter::Source => DISCRETE_FILTER_SOURCE.get_or_init(|| {
            vec![
                DiscreteFilterValue::new("All", "All"),
                DiscreteFilterValue::new("Ahoy Box", "Ahoy Box"),
                DiscreteFilterValue::new("Ladder Chest", "Ladder Chest"),
                DiscreteFilterValue::new("Alchemy", "Alchemy"),
                DiscreteFilterValue::new("Reward", "Reward"),
                DiscreteFilterValue::new("Season Box", "Season Box"),
            ]
        }),
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Continuity {
    pub filter_name: String,
    pub filter_id: u32,
    pub start: u32,
    pub step_size: u32,
    pub end: u32,
    pub max: u32,
    pub min: u32,
}

impl Continuity {
    pub fn level() -> Continuity {
        Continuity {
            filter_name: "Level".into(),
            filter_id: 1,
            start: 1,
            step_size: 1,
            end: 10,
            max: 100,
            min: 1,
        }
    }
}

#[derive(Debug, EnumString, EnumIter, Display, PartialEq, Eq, Hash, Copy, Clone)]
pub enum DiscreteFilter {
    #[strum(to_string = "Type")]
    Type,
    #[strum(to_string = "Faction")]
    Faction,
    #[strum(to_string = "Rarity")]
    Rarity,
    #[strum(to_string = "Foil")]
    Foil,
    #[strum(to_string = "Source")]
    Source,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Discrete {
    pub filter_name: String,
    pub filter_value_list: Vec<DiscreteFilterValue>,
    pub value_id_list: Vec<String>,
    pub filter_id_list: Vec<String>,
}

impl Discrete {
    pub fn filter_type(filter_list: Vec<String>) -> Discrete {
        Discrete::with(DiscreteFilter::Type, filter_list)
    }
    pub fn faction(filter_list: Vec<String>) -> Discrete {
        Discrete::with(DiscreteFilter::Faction, filter_list)
    }

    pub fn rarity(filter_list: Vec<String>) -> Discrete {
        Discrete::with(DiscreteFilter::Rarity, filter_list)
    }
    pub fn foil(filter_list: Vec<String>) -> Discrete {
        Discrete::with(DiscreteFilter::Foil, filter_list)
    }

    pub fn source(filter_list: Vec<String>) -> Discrete {
        Discrete::with(DiscreteFilter::Source, filter_list)
    }

    pub fn with(filter: DiscreteFilter, filter_list: Vec<String>) -> Discrete {
        Discrete {
            filter_name: filter.to_string(),
            filter_value_list: get_discrete_filter(filter).clone(),
            value_id_list: filter_list.clone(),
            filter_id_list: filter_list,
        }
    }

    pub fn with_none(filter: DiscreteFilter) -> Discrete {
        let filter_value_list = get_discrete_filter(filter).clone();
        Discrete {
            filter_name: filter.to_string(),
            filter_value_list,
            value_id_list: vec![],
            filter_id_list: vec![],
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DiscreteFilterValue {
    pub value_name: String,
    pub value_id: String,
}
impl DiscreteFilterValue {
    pub fn new(value_name: &str, value_id: &str) -> DiscreteFilterValue {
        DiscreteFilterValue {
            value_name: value_name.into(),
            value_id: value_id.into(),
        }
    }
}
