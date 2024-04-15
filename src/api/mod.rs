use std::time::Duration;

use crate::crypto;

use super::utils;
use anyhow::Result;
use reqwest::{header, StatusCode};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{json, Value};
pub mod analyze;
pub mod buy_nft_detail;
pub mod category_list;
pub mod game_config;
pub mod market_home;
pub mod market_secondary;
pub mod nft;
pub mod payment;
pub mod user_balance;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResponseWrapper<T> {
    pub msg: String,
    pub data: Option<T>,
    pub code: String,
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

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Discrete {
    pub filter_name: String,
    pub filter_value_list: Vec<DiscreteFilterValue>,
    pub value_id_list: Vec<String>,
    pub filter_id_list: Vec<String>,
}

impl Discrete {
    pub fn filter_type(value_id_list: Vec<String>, filter_id_list: Vec<String>) -> Discrete {
        Discrete {
            filter_name: "Type".into(),
            filter_value_list: vec![
                DiscreteFilterValue::new("Leaders", "Leaders"),
                DiscreteFilterValue::new("Members", "Members"),
            ],
            value_id_list,
            filter_id_list,
        }
    }
    pub fn faction(value_id_list: Vec<String>, filter_id_list: Vec<String>) -> Discrete {
        Discrete {
            filter_name: "Faction".into(),
            filter_value_list: vec![
                DiscreteFilterValue::new("Neutral", "Neutral"),
                DiscreteFilterValue::new("Animal", "Animal"),
                DiscreteFilterValue::new("Plant", "Plant"),
                DiscreteFilterValue::new("Zombie", "Zombie"),
                DiscreteFilterValue::new("Mech", "Mech"),
                DiscreteFilterValue::new("Dragon", "Dragon"),
            ],
            value_id_list,
            filter_id_list,
        }
    }

    pub fn rarity(value_id_list: Vec<String>, filter_id_list: Vec<String>) -> Discrete {
        Discrete {
            filter_name: "Rarity".into(),
            filter_value_list: vec![
                DiscreteFilterValue::new("Common", "Common"),
                DiscreteFilterValue::new("Rare", "Rare"),
                DiscreteFilterValue::new("Epic", "Epic"),
                DiscreteFilterValue::new("Legendary", "Legendary"),
            ],
            value_id_list,
            filter_id_list,
        }
    }
    pub fn foil(value_id_list: Vec<String>, filter_id_list: Vec<String>) -> Discrete {
        Discrete {
            filter_name: "Foil".into(),
            filter_value_list: vec![
                DiscreteFilterValue::new("Regular", "Regular"),
                DiscreteFilterValue::new("Gold", "Gold"),
            ],
            value_id_list,
            filter_id_list,
        }
    }

    pub fn source(value_id_list: Vec<String>, filter_id_list: Vec<String>) -> Discrete {
        Discrete {
            filter_name: "Source".into(),
            filter_value_list: vec![
                DiscreteFilterValue::new("All", "All"),
                DiscreteFilterValue::new("Ahoy Box", "Ahoy Box"),
                DiscreteFilterValue::new("Ladder Chest", "Ladder Chest"),
                DiscreteFilterValue::new("Alchemy", "Alchemy"),
                DiscreteFilterValue::new("Reward", "Reward"),
                DiscreteFilterValue::new("Season Box", "Season Box"),
            ],
            value_id_list,
            filter_id_list,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BuyNftParams {
    pub nonce: String,
    pub amount: u32,
    pub password: String,
    pub payment_type: String,
    pub req_timestamp: i64,
    pub sale_aggregator_number: String,
}

const POST_MAX_RETRY: u32 = 4;
pub struct CardsAhoyApi {
    client: reqwest::Client,
}

impl CardsAhoyApi {
    pub fn new() -> Result<CardsAhoyApi> {
        let config = utils::Config::new();
        let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36 Edg/123.0.0.0")
        .default_headers({
            let mut headers = header::HeaderMap::new();
            headers.insert("Authorization-Token", header::HeaderValue::from_str(&config.authorization)?);
            headers.insert("Client-App-Id", header::HeaderValue::from_str(&config.client_app_id)?);
            headers.insert("Cookie", header::HeaderValue::from_str(&config.cookies)?);
            headers.insert("Content-Type", header::HeaderValue::from_static("application/json"));
            headers
        })
        .build()?;

        Ok(CardsAhoyApi { client })
    }

    /// Sends a POST request with the given payload to the specified URL and
    /// handles retries in case of failures or unexpected responses.
    ///
    /// This function is generic over `T`, which must implement `DeserializeOwned` and `Debug`.
    /// It sends a POST request to the provided URL with the given payload,
    /// and attempts to retry the request up to a maximum of POST_MAX_RETRY times in case of
    /// specific failures or unexpected responses. After each failed attempt,
    /// the function waits for 1 second before retrying.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL to which the POST request will be sent.
    /// * `payload` - The payload to be included in the POST request.
    ///
    /// # Returns
    ///
    /// A `Result` type containing `T` on success, or an error if the request fails or reaches the maximum number of retries.
    ///
    /// # Examples
    ///
    /// ```
    /// use serde::{Deserialize, Serialize};
    /// use serde_json::json;
    ///
    /// #[derive(Debug, Deserialize)]
    /// struct MyResponse {
    ///     // Response struct fields...
    /// }
    ///
    ///
    /// # impl CardsAhoyApi {
    /// #   pub async fn SomeFunction(&self) -> Result<MyResponse> {
    /// #       let url = "https://example.com/api";
    /// #       let payload = json!({});
    /// #       let result = self.post::<MyResponse>(url, payload).await?;
    /// #       Ok(result)       
    /// #   }        
    /// # }
    /// ```
    async fn post<T: DeserializeOwned + std::fmt::Debug>(
        &self,
        url: &str,
        payload: Value,
    ) -> Result<T> {
        for retry_count in 0..=POST_MAX_RETRY {
            let request_builder = self.client.post(url);

            let response = match &payload {
                Value::Object(map) if map.is_empty() => request_builder.send().await?,
                _ => request_builder.json(&payload).send().await?,
            };

            let status = response.status();

            let response_body = response.text().await?;
            if status != StatusCode::OK && retry_count <= POST_MAX_RETRY {
                // 状态码不为200，且重试次数不足，需要重试
                tracing::debug!("Retrying after 1 second...");
                tokio::time::sleep(Duration::from_secs(1)).await;
                continue;
            } else if status != StatusCode::OK && retry_count > POST_MAX_RETRY {
                // 超过最大重试次数，直接返回错误
                break;
            } else if status == StatusCode::OK
                && retry_count <= POST_MAX_RETRY
                && response_body.starts_with("<html>")
            {
                // 状态码为200，返回的503页面，且重试次数不足，需要重试。
                tracing::debug!("Retrying after 1 second...");
                tokio::time::sleep(Duration::from_secs(1)).await;
                continue;
            }

            let result: ResponseWrapper<T> = serde_json::from_str(&response_body)?;

            // 000000为成功， 不成功，返回错误信息。这里一般为 Not Logged，需要重新登录。
            if result.code != "000000" {
                println!("{:#?}", &result);
                return Err(anyhow::anyhow!(result.msg));
            }

            match result.data {
                Some(data) => return Ok(data),
                None => return Err(anyhow::anyhow!("No data in response")),
            }
        }
        Err(anyhow::anyhow!("Max retries attempts reached"))
    }

    pub async fn query_secondary_filter_list(&self) -> Result<()> {
        unimplemented!("query_secondary_filter_list")
    }

    /// Initiates the purchase of an NFT asset using a sale aggregator number.
    ///
    /// This function constructs a payload to securely initiate the purchase of an NFT asset.
    /// It generates a unique nonce using UUID, sets the purchase amount,
    /// and specifies the payment type.
    /// The payload is then encrypted using AES ECB mode with a randomly generated key,
    /// which itself is encrypted using RSA PKCS#1.
    /// Both the encrypted data and encrypted key are encoded in base64 and
    /// sent as a JSON payload in a POST request to the specified endpoint.
    ///
    /// # Arguments
    ///
    /// * `sale_aggregator_number` - A unique identifier for the sale aggregator, used to specify the NFT asset to purchase.
    ///
    /// # Returns
    ///
    /// A `Result` type containing a `String` on success, which typically represents confirmation of the purchase, or an error if the purchase process fails.
    ///
    /// # Examples
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// #     let api = CardsAhoyApi::new();
    /// #     let sale_aggregator_number = "some_unique_sale_aggregator_number";
    /// #     let result = client.buy_ntf_asset(sale_aggregator_number).await;
    /// #     match result {
    /// #         Ok(confirmation) => println!("Purchase confirmation: {}", confirmation),
    /// #         Err(error) => eprintln!("Purchase failed: {:?}", error),
    /// #     }
    /// # }
    /// ```
    pub async fn buy_ntf_asset(&self, sale_aggregator_number: &str) -> Result<String> {
        let params = BuyNftParams {
            nonce: uuid::Uuid::new_v4().to_string(),
            amount: 1,
            password: "".into(),
            payment_type: "Wallet".into(),
            req_timestamp: utils::timestamp(),
            sale_aggregator_number: sale_aggregator_number.into(),
        };

        let data = serde_json::to_string(&params)?;
        let key = crypto::random_key(16);
        let pem = std::fs::read_to_string("key.pem")?;

        let encrypted = crypto::rsa_pkcs1_encrypt(&key, &pem)?;
        // let encrypted = crypto::rsa_encrypt(&key, pem)?;
        let enc_key = crypto::base64_encode(&encrypted);
        tracing::debug!("[RSA] enc_key: {:?} ", enc_key);

        let encrypted_data = crypto::aes_ecb_encrypt(&data, &key)?;
        let enc_content = crypto::base64_encode(&encrypted_data);
        tracing::debug!("[AES] enc_content: {:?} ", enc_content);

        let payload = json!({
            "encContent": enc_content,
            "encKey": enc_key
        });
        tracing::debug!("[API] payload: {:?} ", payload);

        let result = self
            .post::<String>(
                "https://game.metalist.io/api/marketOperate/buyNFTAsset",
                payload,
            )
            .await?;

        Ok(result)
    }

    pub fn pwd_login(&self) -> Result<String> {
        unimplemented!("pwd_login")
    }
}
