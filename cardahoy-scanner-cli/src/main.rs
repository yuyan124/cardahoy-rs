use anyhow::Result;
#[warn(dead_code)]
use cardahoy_api as api;
use cardahoy_scanner as scanner;
use clap::Parser;
use std::env;
use tracing_appender::rolling;
use tracing_subscriber::{fmt, layer::SubscriberExt, EnvFilter};
pub mod commands;
use crate::commands::{Cli, Commands};

fn setup_logging() {
    let file_appender = rolling::daily("./logs", "scanner");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    let filter = EnvFilter::new("info");

    // 设置订阅者，用于日志输出到控制台
    let stdout_layer = fmt::layer().with_writer(std::io::stdout);
    // 设置订阅者，用于日志记录到文件
    let file_layer = fmt::layer().with_writer(non_blocking).with_ansi(false);

    // Configure subscriber with stdout logging and file logging
    let subscriber = tracing_subscriber::registry()
        .with(filter) // 设置过滤器
        .with(stdout_layer) // 输出到控制台
        .with(file_layer); // 记录到文件
                           // .init()

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}

#[tokio::main]
async fn main() -> Result<()> {
    setup_logging();
    tracing::info!("日志系统已初始化，开始记录日志");
    api::nft::init_nft_card_map();

    let raw_args = env::args_os().collect::<Vec<_>>();
    let cli = Cli::parse_from(raw_args);
    match &cli.command {
        Some(Commands::Analyze) => {
            scanner::script::get_all_card_deal_trend().await?;
        }
        Some(Commands::ScanMarket { card }) => {
            let s = scanner::store::Store::new().expect("Store init error");
            loop {
                // or use filter_scan
                let result = s.custom_scan_full().await;

                if let Err(e) = result {
                    println!("{:#?}", e);
                }
                // tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            }
        }
        Some(Commands::AnalyzeRealtime) => {
            tracing::info!("开始获取实时数据");
            scanner::script::get_all_card_realtime().await?;
        }
        None => {}
    }

    Ok(())
}
