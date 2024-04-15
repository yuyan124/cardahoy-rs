use anyhow::Result;
use serde::Deserialize;
use serde_json::json;
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserBalance {
    pub balance: f64,
    pub str_balance: String,
    pub chain_name: String,
    pub price_unity: String,
}

impl super::CardsAhoyApi {
    /// Queries the balance of the user for a specific coin and payment type.
    ///
    /// This function sends a POST request to fetch the user's balance information based on the specified coin ID and payment type. It constructs a JSON payload containing the coin ID and the payment type (in this case, "Wallet"), then sends this payload to a predefined URL. The function returns a vector of `UserBalance`, which contains the balance details for the specified parameters.
    ///
    /// # Returns
    ///
    /// A `Result` type containing a vector of `UserBalance` on success, or an error if the request fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use your_crate::UserBalance;
    /// use serde_json::json;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = CardsAhoyApi::new();
    ///     let result = client.query_user_balance().await;
    ///     let user_balances = result.unwrap();
    ///     // Process user balances...
    /// }
    /// ```
    pub async fn query_user_balance(&self) -> Result<Vec<UserBalance>> {
        let payload = json!({
            "coinId": 1,
            "paymentType": "Wallet"
        });

        let result = self
            .post::<Vec<UserBalance>>(
                "https://game.metalist.io/api/marketQuery/queryUserBalance",
                payload,
            )
            .await?;

        Ok(result)
    }
}
