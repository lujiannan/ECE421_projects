use clap::Parser; // command line parser
extern crate yahoo_finance_api as yahoo; // rename as yahoo
                                         // Import necessary modules for using yahoo
use std::time::{Duration, UNIX_EPOCH};
use tokio; // tokio async dependencies (make sure full features enabled in toml)

use plotly::layout::Layout;
use plotly::Candlestick;
use plotly::Plot;
use plotly::common::Title;
use yahoo_finance_api::Quote;


// use chrono::{prelude::*, Duration};
use chrono::{prelude::*};

/// Program to analyze a stock
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Stock ticker
    #[arg(short, long)]
    ticker: String,
}
fn timestamp_to_date(timestamp: u64) -> String {
    // Create a NaiveDateTime from the timestamp
    let naive = NaiveDateTime::from_timestamp(timestamp as i64, 0);

    // Create a normal DateTime from the NaiveDateTime
    let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);

    // Format the datetime as desired (e.g., "%Y-%m-%d %H:%M:%S")
    let formatted_date = datetime.format("%Y-%m-%d %H:%M:%S").to_string();

    formatted_date
}

fn quotes_to_candlestick_data(
    quotes: Vec<Quote>,
) -> (Vec<String>, Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) {
    // let dates: Vec<String> = quotes.iter().map(|quote| quote.timestamp.to_string()).collect();
    let dates: Vec<String> = quotes.iter().map(|quote| timestamp_to_date(quote.timestamp)).collect();
    let opens: Vec<f64> = quotes.iter().map(|quote| quote.open).collect();
    let highs: Vec<f64> = quotes.iter().map(|quote| quote.high).collect();
    let lows: Vec<f64> = quotes.iter().map(|quote| quote.low).collect();
    let closes: Vec<f64> = quotes.iter().map(|quote| quote.close).collect();

    (dates, opens, highs, lows, closes)
}

fn simple_candlestick_chart(quotes: Vec<Quote>, ticker: &String) {
    let (dates, opens, highs, lows, closes) = quotes_to_candlestick_data(quotes);

    let trace1 = Candlestick::new(dates, opens, highs, lows, closes)
        .name("Candlestick");

    let mut plot = Plot::new();
    plot.add_trace(Box::new(trace1));

    let layout = Layout::new().title(Title::new(&format!("Candlestick Chart of stock: {}", ticker)));
    plot.set_layout(layout);

    plot.show();
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
    // match provider.get_quote_range(ticker.as_str(), "1d", "1mo").await {
    //     Ok(response) => {
    //         match response.quotes() {
    //             Ok(quotes) => println!("Quotes for the last month: {:?}", quotes),
    //             Err(_) => println!("Failed to get quotes for ticker: {}", ticker),
    //         }
    //     },
    //     Err(_) => println!("Invalid ticker: {}", ticker),
    // }

    match provider.get_quote_range(ticker.as_str(), "1d", "6mo").await {
        Ok(response) => {
            match response.quotes() {
                // Ok(quotes) => println!("Quotes for the last month: {:?}", quotes),
                Ok(quotes) => {
                    // Call the modified candlestick chart function with the obtained quotes
                    simple_candlestick_chart(quotes, &args.ticker);
                }
                Err(_) => println!("Failed to get quotes for ticker: {}", ticker),
            }
        }
        Err(_) => println!("Invalid ticker: {}", ticker),
    }

    // plot the data
}
