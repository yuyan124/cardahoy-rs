use super::{nft, Discrete};
use anyhow::Result;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MarketSecondaryResponse {
    pub total: u32,
    pub list: Vec<Secondary>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Secondary {
    // 卖出数量
    pub volume: u32,
    // 当前卡牌数量
    pub quantity: u32,
    // nft::NftCardId 具体的数字值
    pub secondary_id: u32,
    // nft::NftCardId 对应的enum
    pub secondary_name: String,
    // Nft名字：包含Cards, Boxes, Fragments
    pub nft_name: String,
    pub image: String,
    // Nft id： ntf::NtfId
    pub chain_nft_id: u32,
    // 底价
    pub floor_price: String,
    // 货币单位： USDT
    pub price_unity: String,
}

impl super::CardsAhoyApi {
    /// Makes a query to retrieve the secondary market information for a specific NFT.
    ///
    /// # Arguments
    ///
    /// * `nft_id` - The ID of the NFT to query.
    /// * `page` - The page number of the results.
    /// * `sort_type` - The sort type for the results.
    /// * `discrete_list` - A vector of `Discrete` objects representing the discrete list.
    ///
    /// # Examples
    ///
    /// ```
    /// use api::nft::{Discrete, NftId, NftSortType};
    /// use market_secondary::MarketSecondaryResponse;
    /// use serde_json::{json, Value};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = CardsAhoyApi::new();
    ///     let nft_id = NftId::Cards;
    ///     let page = 1;
    ///     let sort_type = NftSortType::PriceDescending;
    ///     let discrete_list = vec![
    ///         api::Discrete::filter_type(vec![], vec![]),
    ///         api::Discrete::faction(vec![], vec![]),
    ///         api::Discrete::rarity(vec![], vec![]),
    ///         api::Discrete::foil(vec![], vec![]),
    ///         api::Discrete::source(vec![], vec![]),
    ///     ];
    ///     let result = client.query_market_secondary(nft_id, page, sort_type, &discrete_list).await;
    ///     let market_secondary = result.unwrap();
    ///     // Process market secondary information...
    /// }
    /// ```
    pub async fn query_market_secondary(
        &self,
        nft_id: nft::NftId,
        page: u32,
        sort_type: nft::NftSortType,
        discrete_list: &Vec<Discrete>,
    ) -> Result<MarketSecondaryResponse> {
        let payload = json!({
            "chainNftId": nft_id as u32,
            "discreteList": json!(discrete_list),
            "continuityList":[],
            "pageNumber":page,
            "pageSize":20,
            "sortType": sort_type as u32,
        });

        let result = self
            .post::<MarketSecondaryResponse>(
                "https://game.metalist.io/api/marketQuery/queryMarketSecondary",
                payload,
            )
            .await?;

        Ok(result)
    }
}
