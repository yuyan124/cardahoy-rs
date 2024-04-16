use super::{nft, Continuity};
use anyhow::Result;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MarketHomeResponse {
    pub list: Vec<CardInformation>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CardInformation {
    // nft::NftId::Cards
    pub chain_nft_id: u32,
    // 数量
    pub amount: u32,
    // 货币单位
    pub price_unity: String,
    // 卡牌等级
    pub priority_trait1: String,
    // 卡牌经验信息
    pub accumulate_trait: AccumulateTrait,
    // 0？
    pub nft_type: u32,
    // 图片
    pub image: String,
    // 价格
    pub sale_price: String,
    // 卡牌唯一id
    pub token_id: String,
    // 卡牌名称 #token_id
    pub nft_name: String,
    // 卡牌经验信息，等于AccumulateTrait的 "key:value"
    pub priority_trait_2: String,
    // 交易编码, 此编码用于买卖，相当于卡牌唯一id
    pub sale_aggregator_number: String,
    // meta信息，包含了 "Price/EXP" 与 "Honor Points"
    pub metadata_list: Vec<MetadataList>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AccumulateTrait {
    pub name: String,
    pub value: u32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MetadataList {
    pub name: String,
    pub value: String,
    pub if_accumulate: Option<bool>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum MarketHomeSortType {
    PriceExpAscending = 4,
    PriceExpDescending = 5,
    PriceAscending = 0,
    PriceDescending = 1,
    Latest = 3,
    HonorPointsAscending = 6,
}

impl super::CardsAhoyApi {
    /// Queries market home information for a specific NFT and NFT card combination with various filters.
    ///
    /// This function sends a POST request to fetch market home data based on specified filters, including NFT ID, NFT card ID, page number, and sort type. It constructs a JSON payload with these parameters along with a predefined coin ID, an empty discrete list, a continuity list (in this example, based on level), a page size, and category IDs derived from the NFT ID and NFT card ID. The function returns a `MarketHomeResponse`, which contains the filtered market home data.
    ///
    /// # Arguments
    ///
    /// * `nft_id` - The ID of the NFT for which market home information is being queried.
    /// * `nft_card_id` - The card ID of the NFT for further filtering.
    /// * `page` - The page number for pagination.
    /// * `sort_type` - The criterion by which the results are sorted.
    ///
    /// # Returns
    ///
    /// A `Result` type containing `MarketHomeResponse` on success, or an error if the request fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use market_home::MarketHomeResponse;
    /// use market_home::MarketHomeSortType;
    /// use serde_json::json;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = CardsAhoyApi::new();
    ///     let nft_id = nft::NftId::Cards;
    ///     let nft_card_id = nft::NftCardId::LionKing;
    ///     let page = 1;
    ///     let sort_type = MarketHomeSortType::PriceAscending;
    ///     let result = client.query_market_home(nft_id, nft_card_id, page, sort_type).await;
    ///     let market_home_info = result.unwrap();
    ///     // Process market home information...
    /// }
    ///
    /// ```
    pub async fn query_market_home(
        &self,
        nft_id: nft::NftId,
        nft_card_id: u32,
        page: u32,
        sort_type: MarketHomeSortType,
    ) -> Result<MarketHomeResponse> {
        let continuity_list = vec![Continuity::level()];

        let payload = json!({
            "coinId": 1,
            "discreteList":[],
            "continuityList": json!(continuity_list),
            "pageNumber":page,
            "pageSize":20,
            "firstCategoryId": nft_id as u32,
            "secondCategoryId": nft_card_id,
            "sortType": sort_type as u32,
        });

        let result = self
            .post::<MarketHomeResponse>(
                "https://game.metalist.io/api/marketQuery/queryMarketHome",
                payload,
            )
            .await?;

        Ok(result)
    }
}
