use anyhow::Result;
use api::{analyze, market_home::MarketHomeResponse, nft::NftCardId};

use cardahoy_api as api;
use chrono::DateTime;
#[cfg(not(feature = "xlsxwriter"))]
use csv::Writer;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
};

#[cfg(feature = "xlsxwriter")]
use xlsxwriter::*;

#[cfg(feature = "xlsxwriter")]
use calamine::{Reader, Xlsx};

#[allow(dead_code)]
pub async fn get_all_card() -> Result<()> {
    // sort output.txt
    let capi = api::CardsAhoyApi::new()?;

    let discrete_list = vec![
        api::filter::Discrete::filter_type(vec![]),
        api::filter::Discrete::faction(vec![]),
        api::filter::Discrete::rarity(vec![]),
        api::filter::Discrete::foil(vec!["Regular".into()]),
        api::filter::Discrete::source(vec![]),
    ];

    let resp = capi
        .query_market_secondary(
            api::nft::NftId::Cards,
            1,
            20,
            api::nft::NftSortType::PriceAscending,
            &discrete_list,
        )
        .await?;
    let total = resp.total;
    let pages = total / 20 + 1;

    let mut cards: HashMap<String, u32> = HashMap::new();

    println!("Total: {}, pages: {}", total, pages);
    for page in 1..=pages {
        let resp = capi
            .query_market_secondary(
                api::nft::NftId::Cards,
                page,
                20,
                api::nft::NftSortType::PriceAscending,
                &discrete_list,
            )
            .await?;
        for card in resp.list {
            cards.insert(card.secondary_name, card.secondary_id);
        }
    }

    let discrete_list = vec![
        api::filter::Discrete::filter_type(vec![]),
        api::filter::Discrete::faction(vec![]),
        api::filter::Discrete::rarity(vec![]),
        api::filter::Discrete::foil(vec!["Gold".into()]),
        api::filter::Discrete::source(vec![]),
    ];

    for page in 1..=pages {
        let resp = capi
            .query_market_secondary(
                api::nft::NftId::Cards,
                page,
                20,
                api::nft::NftSortType::PriceAscending,
                &discrete_list,
            )
            .await?;
        for card in resp.list {
            cards.insert(format!("{}Gold", card.secondary_name), card.secondary_id);
        }
    }

    let path = "output.txt";
    let mut file = File::create(path)?;

    let result: Vec<String> = cards
        .iter()
        .map(|(key, value)| format!("{} = {},", key, value))
        .collect();
    let combined = result.join("\n");

    file.write_all(combined.as_bytes())?;

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CardDealTrend {
    pub name: u32,
    pub max_value: String,
    pub total_value: String,
    pub avg_value: String,
    pub count: u32,
    pub min_value: String,
}

#[cfg(feature = "xlsxwriter")]
fn write_excel_from_other_excel(workbook: &Workbook, from: &str) -> Result<()> {
    if let Ok(_) = fs::metadata(from) {
        let mut old_workbook: Xlsx<_> = calamine::open_workbook("价格表.xlsx")?;

        for sheet_name in old_workbook.sheet_names().to_owned() {
            let range = old_workbook.worksheet_range(&sheet_name)?;

            let sheet_data = range
                .rows()
                .map(|r| r.iter().collect::<Vec<_>>())
                .collect::<Vec<_>>();

            let mut worksheet = workbook.add_worksheet(Some(&sheet_name))?;
            // 遍历现有工作表中的数据，并写入新的工作表
            for (row_index, row) in sheet_data.iter().enumerate() {
                for (col_index, cell) in row.iter().enumerate() {
                    let value = format!("{}", cell);
                    worksheet.write_string(row_index as u32, col_index as u16, &value, None)?;
                }
            }
        }
    }
    Ok(())
}

#[cfg(feature = "xlsxwriter")]
pub async fn get_all_card_deal_trend() -> Result<()> {
    let cards = NftCardId::to_vec_u32();

    let file_path = "history_price.xlsx";
    let workbook = Workbook::new("output.xlsx")?;
    write_excel_from_other_excel(&workbook, file_path)?;

    let capi = api::CardsAhoyApi::new()?;
    let mut results: HashMap<u32, analyze::AnalyzeDealTrendResponse> = HashMap::new();

    for card in cards {
        let resp = capi
            .query_analyze_deal_trend(api::nft::NftId::Cards, card)
            .await?;

        results.insert(card, resp);
    }

    let mut classified_results: HashMap<String, Vec<CardDealTrend>> = HashMap::new();

    for (name, response) in results {
        for node in response.nodes {
            let timestamp = DateTime::from_timestamp_millis(node.timestamp)
                .unwrap()
                .format("%Y-%m-%d")
                .to_string();
            let card_deal_trend = CardDealTrend {
                name,
                max_value: node.max_value,
                total_value: node.total_value,
                avg_value: node.avg_value,
                count: node.count,
                min_value: node.min_value,
            };
            classified_results
                .entry(timestamp)
                .or_insert_with(Vec::new)
                .push(card_deal_trend);
        }
    }
    let _ = classified_results
        .iter()
        .map(|(timestamp, card_deal_trend)| {
            match workbook.get_worksheet(&timestamp) {
                Ok(Some(worksheet)) => {}
                Ok(None) => {
                    let mut worksheet = workbook.add_worksheet(Some(&timestamp))?;

                    let header = vec![
                        "名字",
                        "成交数量",
                        "平均成交额",
                        "最低成交额",
                        "最高成交额",
                        "总成交额",
                    ];
                    for (col, title) in header.iter().enumerate() {
                        worksheet.write_string(0, col as u16, title, None)?;
                    }
                    let mut row_index = 1;
                    let _ = card_deal_trend
                        .iter()
                        .map(|node| {
                            let row = vec![
                                NftCardId::get_name_by_value(node.name, "cn").unwrap(),
                                node.count.to_string(),
                                node.avg_value.clone(),
                                node.min_value.clone(),
                                node.max_value.clone(),
                                node.total_value.clone(),
                            ];
                            for (col, value) in row.iter().enumerate() {
                                let value = format!("{}", value);
                                worksheet.write_string(row_index, col as u16, &value, None)?;
                            }
                            row_index += 1;
                            Ok::<_, anyhow::Error>(())
                        })
                        .collect::<Vec<_>>();
                }
                Err(err) => {
                    // 处理错误
                    println!("Error: {}", err);
                }
            }

            Ok::<_, anyhow::Error>(())
        })
        .collect::<Vec<_>>();

    workbook.close()?;

    fs::remove_file(file_path)?;
    fs::rename("output.xlsx", "history_price.xlsx")?;

    Ok(())
}

#[cfg(feature = "xlsxwriter")]
pub async fn get_all_card_realtime() -> Result<()> {
    let cards = NftCardId::to_vec_u32();
    let ca_api = api::CardsAhoyApi::new()?;
    let mut results: HashMap<String, MarketHomeResponse> = HashMap::new();

    for card in cards {
        let name = NftCardId::get_name_by_value(card, "cn").unwrap();
        tracing::info!("[扫描]: {}", name);
        let resp = ca_api
            .query_market_home(
                api::nft::NftId::Cards,
                card,
                1,
                api::market_home::MarketHomeSortType::PriceExpAscending,
            )
            .await?;
        results.insert(name, resp);
    }

    let workbook = Workbook::new("实时价格.xlsx")?;
    let mut worksheet = workbook.add_worksheet(None)?;
    let mut row_index = 0;
    let header = vec!["名字", "均价"];
    for (col, title) in header.iter().enumerate() {
        worksheet.write_string(0, col as u16, title, None)?;
    }

    let _ = results
        .iter()
        .map(|(name, resp)| {
            let sum: f64 = resp
                .list
                .iter()
                .take(5)
                .filter_map(|ci| {
                    let price = ci
                        .metadata_list
                        .iter()
                        .find(|meta| meta.name == "Price/EXP")
                        .map(|meta| meta.value.parse::<f64>().unwrap())
                        .unwrap_or_else(|| {
                            ci.sale_price.parse::<f64>().unwrap() / ci.accumulate_trait.value as f64
                        });
                    Some(price)
                })
                .sum();
            let avg = sum / 5 as f64;
            let row = vec![name.clone(), format!("{:.3}", avg)];
            for (col, value) in row.iter().enumerate() {
                let value = format!("{}", value);
                worksheet.write_string(row_index, col as u16, &value, None)?;
            }
            row_index += 1;

            Ok::<_, anyhow::Error>(())
        })
        .collect::<Vec<_>>();

    tracing::info!("文件已输出到 -  实时价格.xlsx, 共计{}行。", row_index);
    workbook.close()?;
    Ok(())
}

#[cfg(not(feature = "xlsxwriter"))]
pub async fn get_all_card_realtime() -> Result<()> {
    let cards = NftCardId::to_vec_u32();
    let ca_api = api::CardsAhoyApi::new()?;
    let mut results: HashMap<u32, MarketHomeResponse> = HashMap::new();

    for card in cards {
        let name = NftCardId::get_name_by_value(card.clone(), "cn").unwrap();
        tracing::info!("[扫描]: {}", name);
        let resp = ca_api
            .query_market_home(
                api::nft::NftId::Cards,
                card,
                1,
                api::market_home::MarketHomeSortType::PriceExpAscending,
            )
            .await?;
        results.insert(card, resp);
    }

    let csv_file = File::create("实时价格.csv")?;
    let mut writer = Writer::from_writer(csv_file);

    let headers = ["名字", "英文", "均价"];
    writer
        .write_record(&headers)
        .expect("Unable to write header record");

    let _ = results
        .iter()
        .map(|(card, resp)| {
            let sum: f64 = resp
                .list
                .iter()
                .take(5)
                .filter_map(|ci| {
                    let price = ci
                        .metadata_list
                        .iter()
                        .find(|meta| meta.name == "Price/EXP")
                        .map(|meta| meta.value.parse::<f64>().unwrap())
                        .unwrap_or_else(|| {
                            ci.sale_price.parse::<f64>().unwrap() / ci.accumulate_trait.value as f64
                        });
                    Some(price)
                })
                .sum();
            let avg = sum / 5 as f64;
            let cn_name = NftCardId::get_name_by_value(card.clone(), "cn").unwrap();
            let en_name = NftCardId::get_name_by_value(card.clone(), "en").unwrap();
            let row = [cn_name, en_name, format!("{:.3}", avg)];
            writer.write_record(&row).expect("unable to write record.");

            Ok::<_, anyhow::Error>(())
        })
        .collect::<Vec<_>>();

    tracing::info!("文件已输出到 -  实时价格.csv。");
    writer.flush().expect("unable to flush.");
    Ok(())
}
