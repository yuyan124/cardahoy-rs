use anyhow::Result;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PaymentTypeDetail {
    #[serde(rename = "type")]
    pub type_: u32,
    pub name: String,
    pub market_url: Option<String>,
}

#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PaymentInfo {
    pub unit: String,
}

impl super::CardsAhoyApi {
    pub async fn query_payment_type_detail(&self) -> Result<Vec<PaymentTypeDetail>> {
        let payload = json!({
            "coinId": 1
        });

        let result = self
            .post::<Vec<PaymentTypeDetail>>(
                "https://game.metalist.io/api/marketQuery/queryPaymentTypeDetail",
                payload,
            )
            .await?;

        Ok(result)
    }
}
