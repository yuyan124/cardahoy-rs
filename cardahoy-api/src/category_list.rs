use anyhow::Result;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CategoryListResponse {
    pub category_list: Vec<Category>,
    pub coin_type_list: Vec<CoinType>,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    pub nft_type: u32,
    pub if_support_batch_buy: bool,
    pub category_id: u32,
    pub analyzed: bool,
    pub sort_type_list: Vec<CategorySortType>,
    pub category_type: u32,
    pub category_name: String,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CategorySortType {
    pub sort_type_id: u32,
    pub order_type: u32,
    pub sort_name: String,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CoinType {
    pub coin_id: u32,
    pub coin_type_name: String,
}

impl super::CardsAhoyApi {
    /// Makes a query to retrieve the category list.
    ///
    /// # Examples
    ///
    /// ```
    /// use category_list::CategoryListResponse;
    /// use serde_json::json;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = CardsAhoyApi::new();
    ///     let result = api.query_category_list().await;
    ///     let category_list = result.unwrap();
    ///     // Process category list...
    /// }
    /// ```
    pub async fn query_category_list(&self) -> Result<CategoryListResponse> {
        let result = self
            .post::<CategoryListResponse>(
                "https://game.metalist.io/api/marketQuery/queryCategoryList",
                json!({}),
            )
            .await?;

        Ok(result)
    }
}
