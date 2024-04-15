use anyhow::Result;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BuyNftDetailResponse {
    pub user_balance: UserBalance,
    // 货币id
    pub coin_id: u32,
    // 链
    pub chain_name: String,
    // 合约信息
    pub contract_info: ContractInfo,
    // nft的信息。
    pub meta_data: Vec<MetaData>,

    pub owner_list: Option<String>,

    // 交易编码, 此编码用于买卖，相当于卡牌唯一id
    pub sale_aggregator_number: String,
    // nft::NftId
    pub chain_nft_id: u32,
    //
    pub base_info: BaseInfo,
    pub chain_id: u32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserBalance {
    pub balance: Option<u32>,
    // 货币单位
    pub price_unity: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ContractInfo {
    // 合约标准
    pub token_standard: String,
    pub copyright_fee: f64,
    // 平台手续费
    pub platform_fee: f64,
    // 合约地址
    pub contract_address: String,
    // nft的唯一id
    pub token_id: String,
    pub chain_name: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MetaData {
    pub value: String,
    pub display_color: String,
    pub if_display_bold: bool,
    pub trait_type: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BaseInfo {
    // 数量
    pub amount: u32,
    // 所有者名字
    pub owner: String,
    pub end_time: u64,
    pub owner_profile_image_url: Option<String>,
    pub nft_name: String,
    pub image: String,
    // 售价
    pub sale_price: String,
    // 货币单位
    pub price_unity: String,
    // nft唯一id
    pub token_id: String,
    pub priority_level_trait: Option<String>,
    pub desc: Option<String>,
}

impl super::CardsAhoyApi {
    /// Queries the detail of an NFT available for purchase using its sale aggregator number.
    ///
    /// This function sends a POST request to fetch the details of a specific NFT intended for purchase, identified by its sale aggregator number. It constructs a JSON payload containing the sale aggregator number, then sends this payload to a predefined URL. The function returns a `BuyNftDetailResponse`, which contains detailed information about the NFT, including its attributes, sale conditions, and more.
    ///
    /// # Arguments
    ///
    /// * `sale_aggregator_number` - A unique identifier for the NFT sale aggregator, used to fetch the NFT's purchase details.
    ///
    /// # Returns
    ///
    /// A `Result` type containing `BuyNftDetailResponse` on success, or an error if the request fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use buy_nft_detail::BuyNftDetailResponse;
    /// use serde_json::json;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::new();
    ///     let sale_aggregator_number = "unique_sale_aggregator_number".to_string();
    ///     let result = client.query_buy_nft_detail(sale_aggregator_number).await;
    ///     let nft_detail = result.unwrap();
    ///     // Process NFT purchase details...
    /// }
    /// ```
    pub async fn query_buy_nft_detail(
        &self,
        sale_aggregator_number: String,
    ) -> Result<BuyNftDetailResponse> {
        let payload = json!({
            "saleAggregatorNumber": sale_aggregator_number
        });

        let result = self
            .post::<BuyNftDetailResponse>(
                "https://game.metalist.io/api/marketQuery/queryBuyNftDetail",
                payload,
            )
            .await?;

        Ok(result)
    }
}
