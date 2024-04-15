use super::{nft, payment};
use anyhow::Result;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AnalyzeSellHistoryResponse {
    pub amount: u32,
    pub sale_status: u32,
    pub sale_time: u64,
    pub nft_name: String,
    pub sale_price: String,
    pub total_price: String,
    pub price_unity: String,
    pub fee: f64,
    pub token_id: String,
    pub image: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AnalyzeNftResponse {
    pub analyze_nft_list: Vec<AnalyzeNft>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AnalyzeNft {
    pub nft_name: String,
    pub nft_type: u32,
    pub chain_nft_id: u32,
    pub category_type: u32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AnalyzeFloorPriceTrendResponse {
    pub nodes: Vec<FloorPrice>,
    pub payment_info: payment::PaymentInfo,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FloorPrice {
    pub value: String,
    pub timestamp: u64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AnalyzeDealTrendResponse {
    pub nodes: Vec<AnalyzeDealTrendNode>,
    pub payment_info: payment::PaymentInfo,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AnalyzeDealTrendNode {
    pub max_value: String,
    pub total_value: String,
    pub timestamp: u64,
    pub avg_value: String,
    pub count: u32,
    pub min_value: String,
}

impl super::CardsAhoyApi {
    /// Queries the analysis list for NFTs.
    ///
    /// This function sends a POST request to fetch an analysis list related to NFTs from a predefined URL. It returns a result wrapped in a `Result` type, which, on success, contains an `AnalyzeNftResponse` indicating the analysis details.
    ///
    /// # Returns
    ///
    /// A `Result` type containing `AnalyzeNftResponse` on success, or an error if the request fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use analyze::AnalyzeNftResponse;
    /// use serde_json::json;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = CardsAhoyApi::new();
    ///     let result = api.query_analyze_list().await;
    ///     let analyze_list = result.unwrap();
    ///     // Process analyze list...
    /// }
    ///
    /// ```
    pub async fn query_analyze_list(&self) -> Result<AnalyzeNftResponse> {
        let result = self
            .post::<AnalyzeNftResponse>(
                "https://game.metalist.io/api/marketQuery/queryAnalyzeList",
                json!({}),
            )
            .await?;

        Ok(result)
    }

    /// Queries the sell history analysis for a specific NFT.
    ///
    /// This function sends a POST request to retrieve the sell history analysis of a given NFT. It constructs a JSON payload with the NFT's ID and an empty category ID, then sends this payload to a predefined URL. The function returns a `Vec<AnalyzeSellHistoryResponse>`, which is a list of sell history analysis responses for the specified NFT.
    ///
    /// # Arguments
    ///
    /// * `nft_id` - The ID of the NFT for which the sell history analysis is being queried.
    ///
    /// # Returns
    ///
    /// A `Result` type containing a vector of `AnalyzeSellHistoryResponse` on success, or an error if the request fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use analyze::AnalyzeSellHistoryResponse;
    /// use serde_json::json;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = CardsAhoyApi::new();
    ///     let nft_id = nft::NftId::Cards;
    ///     let result = client.query_analyze_sell_history(nft_id).await;
    ///     let sell_history = result.unwrap();
    ///     // Process sell history...
    /// }
    ///    
    /// ```
    pub async fn query_analyze_sell_history(
        &self,
        nft_id: nft::NftId,
    ) -> Result<Vec<AnalyzeSellHistoryResponse>> {
        let payload = json!({
            "chainNftId": nft_id as u32,
            "categoryId":"",
        });

        let result = self
            .post::<Vec<AnalyzeSellHistoryResponse>>(
                "https://game.metalist.io/api/marketQuery/queryAnalyzeSellHistory",
                payload,
            )
            .await?;

        Ok(result)
    }

    /// Queries the floor price trend analysis for a specific NFT over a given time range.
    ///
    /// This function sends a POST request to fetch the floor price trend analysis of a specified NFT. It constructs a JSON payload including the NFT's ID, an empty category ID, and a time range (set to "7d" for 7 days in this example), then sends this payload to a predefined URL. The function returns an `AnalyzeFloorPriceTrendResponse`, which contains the analysis of the floor price trend for the specified NFT.
    ///
    /// # Arguments
    ///
    /// * `nft_id` - The ID of the NFT for which the floor price trend analysis is being queried.
    ///
    /// # Returns
    ///
    /// A `Result` type containing `AnalyzeFloorPriceTrendResponse` on success, or an error if the request fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use analyze::AnalyzeFloorPriceTrendResponse;
    /// use serde_json::json;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = CardsAhoyApi::new();
    ///     let nft_id = nft::NftId::Cards;
    ///     let result = client.query_analyze_floor_price_trend(nft_id).await;
    ///     let floor_price_trend = result.unwrap();
    ///     // Process floor price trend...
    /// }
    ///
    /// ```
    pub async fn query_analyze_floor_price_trend(
        &self,
        nft_id: nft::NftId,
    ) -> Result<AnalyzeFloorPriceTrendResponse> {
        let payload = json!({
            "chainNftId": nft_id as u32,
            "categoryId":"",
            "timeRange": "7d",
        });

        let result = self
            .post::<AnalyzeFloorPriceTrendResponse>(
                "https://game.metalist.io/api/marketQuery/queryAnalyzeFloorPriceTrend",
                payload,
            )
            .await?;

        Ok(result)
    }

    /// Queries the deal trend analysis for a specific NFT over a given time range.
    ///
    /// This function sends a POST request to retrieve the deal trend analysis of a specified NFT. It constructs a JSON payload including the NFT's ID, an empty category ID, and a specified time range ("7d" for 7 days), then sends this payload to a predefined URL. The function returns an `AnalyzeDealTrendResponse`, which contains the analysis of the deal trend for the specified NFT.
    ///
    /// # Arguments
    ///
    /// * `nft_id` - The ID of the NFT for which the deal trend analysis is being queried.
    ///
    /// # Returns
    ///
    /// A `Result` type containing `AnalyzeDealTrendResponse` on success, or an error if the request fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use analyze::AnalyzeDealTrendResponse;
    /// use serde_json::json;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = CardsAhoyApi::new();
    ///     let nft_id = nft::NftId::Cards; // Assume NftId::new() returns a valid NftId
    ///     let result = client.query_analyze_deal_trend(nft_id).await;
    ///     let deal_trend = result.unwrap();
    ///     // Process deal trend...
    /// }
    ///
    /// ```
    pub async fn query_analyze_deal_trend(
        &self,
        nft_id: nft::NftId,
    ) -> Result<AnalyzeDealTrendResponse> {
        let payload = json!({
            "chainNftId": nft_id as u32,
            "categoryId":"",
            "timeRange": "7d",
        });
        let result = self
            .post::<AnalyzeDealTrendResponse>(
                "https://game.metalist.io/api/marketQuery/queryAnalyzeDealTrend",
                payload,
            )
            .await?;

        Ok(result)
    }
}
