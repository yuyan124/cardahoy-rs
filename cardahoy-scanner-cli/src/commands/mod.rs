use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    ScanMarket {
        #[arg(long)]
        card: Option<String>,
    },
    Analyze,
    AnalyzeRealtime,
}

#[derive(Args, Debug)]
pub struct ScanMarket {
    #[clap(subcommand)]
    pub command: Option<ScanMarketCommands>,
}

#[derive(Subcommand, Debug)]
pub enum ScanMarketCommands {
    Card {
        #[arg(long)]
        all: bool,
    },
}
