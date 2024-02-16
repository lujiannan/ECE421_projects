/*
ECE421 Group Project 1, due Feb 16, 2024

Program Name: Our Stock Querying Program
Version: 1.0.0
Author: Prabh Kooner, Brandon Hoynick, Jiannan Lu
*/

use chrono::prelude::{DateTime, NaiveDateTime, Utc}; // for date and time unix conversion
use clap::App; // for command line argument parser

// Program Description:
const ABOUT_TEXT: &str = r#"
This program queries stock tickers from Yahoo Finance 
and displays it's historical data in an interactive chart.


There are two ways to use this program:

1. Start the executable with a stock ticker as a --ticker argument.
If that ticker is valid, 
it will then query Yahoo Finance for the ticker's last 6-month's daily closing values,
and output the values in an interactive chart (using a browser-based html file).
The program will then exit.

2. Start the executable without any arguments.
The program will prompt you to enter a stock ticker.
If that ticker is valid, 
it will then query Yahoo Finance for the ticker's last 6-month's daily closing values,
and output the values in an interactive chart (using a browser-based html file).
The program will then ask if you would like to continue checking stocks.
If you choose to continue, the program will prompt you to enter another stock ticker,
otherwise you can exit the program.

The chart also highlights volatile days 
(where the difference between the high and low prices is greater than 2% of the closing price), 
using a candlestick chart overlay option.
"#;

// Importing the necessary libraries
use chrono::prelude::DateTime; // for date and time unix conversion
use clap::Parser; // for command line argument parser
use plotly::common::{Marker, Mode, Title}; // for charting abilities
use plotly::layout::{Axis, Layout};
use plotly::{Candlestick, Plot, Scatter};
use tokio; // for async await abilities (make sure full features enabled in toml)
extern crate yahoo_finance_api as yahoo; // for grabbing stock data from Yahoo
use yahoo_finance_api::Quote;

// Function takes a u64 Unix timestamp and returns a formatted date String
fn convert_timestamp_to_date(timestamp: u64) -> String {
    // Create a DateTime from the 'Unix seconds' timestamp
    let datetime = DateTime::from_timestamp(timestamp as i64, 0).unwrap();

    // Format the datetime as desired (e.g., "%Y-%m-%d %H:%M:%S")
    let formatted_date = datetime.format("%Y-%m-%d").to_string();

    return formatted_date;
}

// Function to convert Yahoo Quote quotes to separated Vector data (this currently seems easier to work with in plotly more than the struct itself)
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

    // returns tuple of vectors ready to be used in a stock chart
    return (dates, opens, highs, lows, closes);
}


// Function to find which days of the previous vectors are volatile, and return them in a new set of vectors
fn grab_volatile_days(
    dates: Vec<String>,
    opens: Vec<f64>,
    highs: Vec<f64>,
    lows: Vec<f64>,
    closes: Vec<f64>,
) -> (Vec<String>, Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) {
    // Let's setup some new vectors to hold the volatile days
    let mut vdates = vec![];
    let mut vopens = vec![];
    let mut vhighs = vec![];
    let mut vlows = vec![];
    let mut vcloses = vec![];

    // iterate through the dates, and if the difference between the high and low is greater than 2% of the closing price, then add it to the new vectors
    for (i, date) in dates.iter().enumerate() {
        if (highs[i] - lows[i]) / closes[i] > 0.02 {
            vdates.push(date.clone());
            vopens.push(opens[i]);
            vhighs.push(highs[i]);
            vlows.push(lows[i]);
            vcloses.push(closes[i]);
        }
    }

    // returns tuple of vectors ready to be used in a stock chart
    return (vdates, vopens, vhighs, vlows, vcloses);
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


    // Of the given days, let's find the volatile dates
    let (vdates, vopens, vhighs, vlows, vcloses) =
        grab_volatile_days(dates, opens, highs, lows, closes);

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
        .height(900); // Set the height to 900 pixels (default is 450 pixels, which is too short)
    plot.set_layout(layout);

    // saves the plot to an html file
    plot.write_html(&format!("plot_{}.html", stock_ticker));
}

#[tokio::main] // allows async await function (when using Yahoo to check for stock data)
async fn stock_data_checker(provider: &yahoo::YahooConnector, stock_ticker_arg: &String) {
    // Function to check for stock data using Yahoo
    match provider // look for quote, using 'match,OK,Err' style
        .get_quote_range(&stock_ticker_arg, "1d", "6mo")
        .await
    {
        // check Yahoo for this ticker, for daily values, for last 6-months
        Ok(response) => {
            // if we get a good ticker...
            match response.quotes() {
                // ...then get the quotes...
                Ok(quotes) => {
                    // println!("We have received a valid stock ticker response from: {}", quotes.longname); //* print the longname of the stock (cant seem find it in the quotes struct)
                    setup_plotly_chart(quotes, &stock_ticker_arg); // ...and use the quotes in a candlestick chart. Ref:
                }
                Err(e) => println!(
                    "Error: {}, Failed to get quotes for ticker: {}",
                    e, stock_ticker_arg
                ),
            }
        }
        Err(e) => println!("Error: {}, Invalid ticker: {}", e, stock_ticker_arg),
    }
}

// this is needed to use clap, cmd line parsing, about, and help
#[derive(Parser, Debug)]
#[command(version = "1.0.0", about = ABOUT_TEXT , long_about = None)]
struct Args {
    /// Stock ticker
    #[arg(short, long)]
    ticker: Option<String>,
}

// main function to run the program
fn main() {
    let arg_present; // used to check if a ticker was passed as an argument

    // parse args (if any)
    let args = Args::parse();
    let stock_ticker_arg = match args.ticker {
        Some(ticker) => {
            // if a ticker is present, use it
            let ticker = ticker.trim().to_uppercase();
            println!("Your stock ticker: {}", ticker);
            arg_present = true;
            ticker
        }
        None => {
            // if no ticker is present, keep the program running to ask the user for a ticker
            arg_present = false;
            String::new() // blank unused ticker, but satisfies the compiler
        }
    };

    let provider = yahoo::YahooConnector::new(); // setup stock data provider (Yahoo)

    // check if a ticker was passed as an argument
    if arg_present == true {
        stock_data_checker(&provider, &stock_ticker_arg); // check for the stock ticker data using the provider
    } else {
        // if no ticker was passed as an argument, keep the program running to ask the user for a ticker
        println!("Welcome to our stock querying program!");
        let mut while_holder = true; // keep the program running until the user decides to exit
        while while_holder {
            println!("Enter a valid stock ticker to get the last 6-month's daily values: ");
            let mut stock_ticker = String::new();
            let _ = std::io::stdin().read_line(&mut stock_ticker); // get keyboard input
            stock_ticker = stock_ticker.trim().to_uppercase(); // make it trimmed and uppercase
            println!("Your stock ticker: {}", stock_ticker);

            stock_data_checker(&provider, &stock_ticker); // check for the stock ticker data using the provider

            let mut while_holder_2 = true; // keep the program running until the user decides to exit
            while while_holder_2 {
                println!("Would you like to continue checking stocks? (y or n): ");
                let mut ask = String::new();
                let _ = std::io::stdin().read_line(&mut ask); // get keyboard input
                ask = ask.trim().to_uppercase(); // make it trimmed and uppercase
                if ask == "Y" || ask == "YES" {
                    while_holder = true;
                    while_holder_2 = false;
                } else if ask == "N" || ask == "NO" {
                    while_holder = false;
                    while_holder_2 = false;
                    println!("Thank you for using our stock querying program, goodbye!");
                } else {
                    println!("Invalid input, please try again.");
                }
            }
        }
    }
}
