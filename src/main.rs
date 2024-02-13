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

// cargo run -- --ticker aapl
#[tokio::main] // allow async main function for now
async fn main() {
    // parse arguments and ticker
    let args = Args::parse();
    let mut ticker = args.ticker.to_uppercase();
    println!("ticker: {}", ticker);


    // get stock quote data
    let provider = yahoo::YahooConnector::new();
    // Use `await` instead of `tokio_test::block_on` for async calls in the main function.
    match provider.get_quote_range(ticker.as_str(), "1d", "1mo").await {
        Ok(response) => {
            match response.quotes() {
                Ok(quotes) => println!("Quotes for the last month: {:?}", quotes),
                Err(_) => println!("Failed to get quotes for ticker: {}", ticker),
            }
        },
        Err(_) => println!("Invalid ticker: {}", ticker),
    }

    // plot the data
}