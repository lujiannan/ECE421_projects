/*
ECE421 Group Project 1, due Feb 15, 2024

Program Name: Our Stock Querying Program
Version: 0.1
Author: Prabh Kooner, Brandon Hoynick, Jiannan Lu
*/
const ABOUT_TEXT: &str = r#"This program queries stock data from Yahoo Finance and displays it in an interactive chart.
When started, it will prompt the user to enter a stock ticker, and then, if valid,
display the ticker's last 6-month's daily closing values in an interactive browser-based chart;
The chart also highlights volatile days (where the difference between the high
and low prices is greater than 2%), using a candlestick chart overlay option.
The user will then be prompted to continue checking stocks or exit the program."#;




use chrono::prelude::{DateTime, NaiveDateTime, Utc}; // for date and time unix conversion
use clap::Parser; // for command line argument parser
use plotly::common::{Marker, Mode, Title}; // for charting abilities
use plotly::layout::{Axis, Layout};
use plotly::{Candlestick, Plot, Scatter};
use tokio; // for async await abilities (make sure full features enabled in toml)
extern crate yahoo_finance_api as yahoo; // for grabbing stock data from Yahoo
use yahoo_finance_api::Quote;

// Function takes a u64 Unix timestamp and returns a formatted date String
fn convert_timestamp_to_date(timestamp: u64) -> String {
    // Create a NaiveDateTime from the timestamp
    let naive = NaiveDateTime::from_timestamp(timestamp as i64, 0); //* this is deprecated, but still works; and screws up the next DateTime if updated

    // Create a normal DateTime from the NaiveDateTime
    let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);

    // Format the datetime as desired (e.g., "%Y-%m-%d %H:%M:%S")
    // let formatted_date = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
    let formatted_date = datetime.format("%Y-%m-%d").to_string();

    return formatted_date;
}

// Function to convert Yahoo Quote quotes to separated Vector data (this currently seems easier to work with in plotly, but might be unnecessary)
fn convert_quotes_to_candlestick_data(
    quotes: Vec<Quote>,
) -> (Vec<String>, Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) {
    // pull out the dates, opens, highs, lows, and closes from the quotes and put them into separate vectors
    // need to convert the unix timestamp to a date string
    // 'iter' sets up iteration over the quotes, 'map' applies a possible internal function conversion to each quote,
    // and 'collect' collects the results into a vector
    let dates: Vec<String> = quotes
        .iter()
        .map(|quote| convert_timestamp_to_date(quote.timestamp))
        .collect();
    let opens: Vec<f64> = quotes.iter().map(|quote| quote.open).collect();
    let highs: Vec<f64> = quotes.iter().map(|quote| quote.high).collect();
    let lows: Vec<f64> = quotes.iter().map(|quote| quote.low).collect();
    let closes: Vec<f64> = quotes.iter().map(|quote| quote.close).collect();

    return (dates, opens, highs, lows, closes); //return tuple of vectors ready to be used in a stock chart
}

// Function to setup the plotly chart
fn setup_plotly_chart(quotes: Vec<Quote>, stock_ticker: &String) {
    // pull out the dates, opens, highs, lows, and closes from the quotes and put them into separate vectors
    let (dates, opens, highs, lows, closes) = convert_quotes_to_candlestick_data(quotes.clone());

    // find the highest and lowest closing prices and their respective dates, and print them to the console
    let highest_closing = closes.iter().fold(f64::MIN, |a, &b| a.max(b)); // find the highest closing price
    let date_of_highest_closing =
        &dates[closes.iter().position(|&x| x == highest_closing).unwrap()]; // find the date of the highest closing price
    let lowest_closing = closes.iter().fold(f64::MAX, |a, &b| a.min(b)); // find the lowest closing price
    let date_of_lowest_closing = &dates[closes.iter().position(|&x| x == lowest_closing).unwrap()]; // find the date of the lowest closing price
    println!(
        "The highest closing price of {} was ${:.2} on {}",
        stock_ticker, highest_closing, date_of_highest_closing
    );
    println!(
        "The lowest closing price of {} was ${:.2} on {}",
        stock_ticker, lowest_closing, date_of_lowest_closing
    );

    // Create a Scatter trace
    let trace_1_line_closing_days = Scatter::new(dates.clone(), closes.clone())
        .mode(Mode::Lines) // Set the marker mode
        .marker(Marker::new().size(4)) // Set the marker size
        .name("Daily Closing Prices"); // Set the trace name

    // Of the given days, let's find the volatile dates //* should put this in a separate function
    let mut vdates = vec![];
    let mut vopens = vec![];
    let mut vhighs = vec![];
    let mut vlows = vec![];
    let mut vcloses = vec![];

    for (i, date) in dates.iter().enumerate() {
        if (highs[i] - lows[i]) / closes[i] > 0.02 {
            vdates.push(date.clone());
            vopens.push(opens[i]);
            vhighs.push(highs[i]);
            vlows.push(lows[i]);
            vcloses.push(closes[i]);
        }
    }

    // Create a second trace (in Candlestick form) to represent volatile dates
    let trace_2_error_volatile_days =
        Candlestick::new(vdates, vopens, vhighs, vlows, vcloses).name("Volatile Days");

    // create plot and add traces
    let mut plot = Plot::new();
    plot.add_trace(Box::new(trace_2_error_volatile_days));
    plot.add_trace(trace_1_line_closing_days);

    // Create the plot layout
    let layout = Layout::new()
        .title(Title::new(&format!(
            "Stock Price Chart of Stock Ticker: {}",
            stock_ticker
        )))
        .x_axis(Axis::new().title(Title::new("Date")))
        .y_axis(Axis::new().title(Title::new("Price ($USD)")))
        .height(900); // Set the height to 800 pixels (default is 450 pixels, which is too short)
    plot.set_layout(layout);

    // plot.show(); // open the plot in a browser window
    plot.write_html(&format!("plot_{}.html", stock_ticker));

}

// DONT REMOVE this is needed to clap, cmd line parsing, about, and help
/// Program to analyze a stock
#[derive(Parser, Debug)]
#[command(version = "1.0", about = ABOUT_TEXT , long_about = None)]
struct Args {
    /// Stock ticker
    #[arg(short, long)]
    ticker: String,
}



// main function to run the program
#[tokio::main] // allows async await function in main() (when using Yahoo to check for stock data)
async fn main() {
    // parse args
    let args = Args::parse();
    let stock_ticker = args.ticker.to_uppercase();
    println!("Your stock ticker: {}", stock_ticker);

    let provider = yahoo::YahooConnector::new(); // setup stock data provider (Yahoo) ref: https://docs.rs/yahoo_finance_api/latest/yahoo_finance_api/#get-the-history-of-quotes-for-time-range , https://crates.io/crates/yahoo_finance_api/0.3.1

    match provider // match,OK,Err Ref: https://doc.rust-lang.org/rust-by-example/error/result/result_map.html
        .get_quote_range(stock_ticker.as_str(), "1d", "6mo")
        .await
    {
        // check Yahoo for this ticker, for daily values, for last 6-months
        Ok(response) => {
            // if we get a good ticker...
            match response.quotes() {
                // ...then get the quotes...
                Ok(quotes) => {
                    // println!("We have received a valid stock ticker response from: {}", quotes.longname); //* print the longname of the stock (cant seem find it in the quotes struct)
                    setup_plotly_chart(quotes, &stock_ticker); // ...and use the quotes in a candlestick chart. Ref:
                }
                Err(e) => println!(
                    "Error: {}, Failed to get quotes for ticker: {}",
                    e, stock_ticker
                ),
            }
        }
        Err(e) => println!("Error: {}, Invalid ticker: {}", e, stock_ticker),
    }
        
}
