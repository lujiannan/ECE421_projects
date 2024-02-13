use clap::Parser; // command line parser
extern crate yahoo_finance_api as yahoo; // rename as yahoo
// Import necessary modules for using yahoo
use std::time::{Duration, UNIX_EPOCH};
use tokio; // tokio async dependencies (make sure full features enabled in toml)

/// Program to analyze a stock
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Stock ticker
    #[arg(short, long)]
    ticker: String,
}


#[tokio::main] // allow async main function for now
async fn main() {
    // parse arguments and ticker
    let args = Args::parse();
    let mut ticker = args.ticker.to_uppercase();
    println!("ticker: {}", ticker);


    let provider = yahoo::YahooConnector::new();
    // Use `await` instead of `tokio_test::block_on` for async calls in the main function.
    let response = provider.get_quote_range(ticker.as_str(), "1d", "6mo").await.unwrap(); // ticker, frequency, time range
    let quotes = response.quotes().unwrap();
    println!("Apple's quotes of the last month: {:?}", quotes);
}