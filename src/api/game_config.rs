use anyhow::Result;
use serde::Deserialize;
use serde_json::json;
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GameConfigResponse {
    pub exp_attribute: bool,
    pub ground_expire_minute: u32,
    pub analyze_tab_open: bool,
    pub trade_coin_list: Vec<TradeCoin>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TradeCoin {
    pub coin_id: u32,
    pub decimals: u32,
    pub coin_url: String,
    pub if_default: bool,
    pub price_unity: String,
    pub coin_type: Option<String>,
    pub platform_fee_percent: f64,
    pub copyright_fee_percent: f64,
}

impl super::CardsAhoyApi {
    /// Makes a query to retrieve the game configuration.
    ///
    /// # Examples
    ///
    /// ```
    /// use game_config::GameConfigResponse;
    /// use serde_json::json;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = CardsAhoyApi::new();
    ///     let result = api.query_game_config().await;
    ///     let game_config = result.unwrap();
    ///     // Process game configuration...
    /// }
    ///
    /// ```
    pub async fn query_game_config(&self) -> Result<GameConfigResponse> {
        let result = self
            .post::<GameConfigResponse>(
                "https://game.metalist.io/api/marketQuery/queryGameConfig",
                json!({}),
            )
            .await?;

        Ok(result)
    }
}
