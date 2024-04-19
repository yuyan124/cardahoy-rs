use crate::nft::NftId;

use super::nft;
use anyhow::Result;
use cardahoy_crypto as crypto;
use cardahoy_utils as utils;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SaleNftPriceRange {
    pub lowest_price: String,
    pub last_sale_price: Option<String>,
    pub exp_attribute: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SellNftParams {
    pub nonce: String,
    pub amount: u32,
    pub password: String,
    pub price: String,
    pub req_timestamp: i64,
    pub token_id: String,
    pub coin_id: u32,
    pub chain_nft_id: u32,
}

impl super::CardsAhoyApi {
    pub async fn query_sell_nft_detail(
        &self,
        nft_id: nft::NftId,
        token_ids: Vec<String>,
    ) -> Result<SaleNftPriceRange> {
        let payload = json!({
            "chainNftId": nft_id as u32,
            "coinId": 1,
            "tokenIds": token_ids
        });

        let result = self
            .post::<SaleNftPriceRange>(
                "https://game.metalist.io/api/marketQuery/querySaleNftPriceRange",
                payload,
            )
            .await?;
        Ok(result)
    }

    pub async fn ground_nft_asset(&self, price: f64, token_id: String) -> Result<String> {
        let params = SellNftParams {
            nonce: uuid::Uuid::new_v4().to_string(),
            amount: 1,
            password: String::from(""),
            price: format!("{}", price),
            req_timestamp: utils::timestamp(),
            token_id,
            coin_id: 1,
            chain_nft_id: NftId::Cards as u32,
        };

        let (enc_key, enc_content) = self.encrypt(params)?;
        let payload = json!({
            "encContent": enc_content,
            "encKey": enc_key
        });
        tracing::debug!("[API] payload: {:?} ", payload);

        let result = self
            .post::<String>(
                "https://game.metalist.io/api/marketOperate/groundNFTAsset",
                payload,
            )
            .await?;

        Ok(result)
    }
}
