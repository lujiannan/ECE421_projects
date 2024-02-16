# Our Stock Querying Program
ECE421 Group Project 1, due Feb 16, 2024  
Program Name: Our Stock Querying Program  
Version: 1.0.0 
Author: Prabh Kooner, Brandon Hoynick, Jiannan Lu  

## Crates Used
- `chrono`: This crate is used for handling date and time data (like converting unix timestamps).  https://crates.io/crates/chrono
- `clap`: This crate is used for command line argument parsing (like `--help`). https://crates.io/crates/clap
- `plotly`: This crate is used for creating interactive charts to visualize the stock data. https://crates.io/crates/plotly
- `tokio`: This crate is used for asynchronous programming (for async-await parsing Yahoo's stock database). https://crates.io/crates/tokio
- `yahoo_finance_api`: This crate is used to fetch stock data from Yahoo Finance. https://crates.io/crates/yahoo_finance_api

## Financial Analysis Algorithms
As per the project requirements:
- We used the clap library's parser to read the stock ticker from the command line and provide "--help"
- We use a stock finder API (Yahoo) (along with tokio's async-await caller) to check for a valid given stock ticker data for last 6 months, which returns to us as a Quote variable.
- We pull that quote variable's data into vectors to be used as points in the interactive graph; the mapping, collect, and other iterate functions allow us to keep a container variable as an array of data that can be modified (like converting unixstamp to DateTime string) all within a single line non-looped function call.
- We use a simple for-loop that iterates over all vector data to find volatile days (using the `(highs[i] - lows[i]) / closes[i] > 0.02` calculate to signifying volatile days).
- We calculate the highest and lowest closing prices of the last 6 months using a 'fold' min max iterator (and print those dates and prices to the console).
- Finally, we use a simplified plotly library to populate the closing price and volatile day traces to a browser based plot.
- Additionally, our programs prompts and calls are setup to prevent crashes (i.e. they include error checkers).

## Charting Setup
We used the `plotly` crate to create a chart of both line and candlestick traces for the stock data. The line chart shows the daily closing prices, while the candlestick represents volatile days. The x-axis represents the date and the y-axis represents the stock price (in $USD). We enabled autoscaling for the y-axis to ensure that all data points are visible in the chart. The program generates an interactive "plot.html" file. The interactivity of the chart allows the user to show/hide traces, and zoom into date ranges.

## Project Setup
To build the program:

1. Extract any saved release (v1.0.0) zip/folder to desired location, or, 
1. Clone the repository: `git clone https://github.com/kooner27/421_projects.git` 
2. Navigate within the project directory to the src parent folder: `cd 421_projects/stock_monitor/`
3. Build the project: `cargo build --release`
4. (Optional) To directly build AND run the program, run the command: `cargo run --release -- --ticker <stock_ticker>` or `cargo run --release`
5. (Optional) To directly build AND run the help menu: `cargo run --release -- --help`

To run the compiled binary build:
1. Navigate to the executable directory: `cd target/release/`
2. Run the executable: `stock_monitor --ticker <stock_ticker>` or `stock_monitor`
3. (Optional) Open the help menu: `stock_monitor --help`

## Usage Instructions
After running the program:
1. Open the generated `plot_<stock_ticker>.html` file (in the same directory).
2. Navigate browser chart; clicking on the top-right options (e.g. zoom) allows you to select spots on the chart to enhance (e.g. zoom: allows you to select a smaller range of dates to zoom into); Selecting the legends will show/hide the 'daily closing price' and 'volatile day' traces.
3. The terminal will also display the dates of the highest and lowest closing prices for the last 6-months.

## References
- plotly examples - https://github.com/igiagkiozis/plotly/blob/master/examples/financial_charts/src/main.rs ; https://docs.rs/plotly/latest/plotly/plot/struct.Plot.html ; https://blog.logrocket.com/plotting-rust-projects-plotly/
- chrono timestamp conversion example - https://www.epochconvert.com/programming/rust
- match,OK,Err example -  https://doc.rust-lang.org/rust-by-example/error/result/result_map.html
- unwrap, panic exmaple - https://www.programiz.com/rust/unwrap-and-expect 
- Yahoo stock data examples - https://docs.rs/yahoo_finance_api/latest/yahoo_finance_api/#get-the-history-of-quotes-for-time-range ; https://crates.io/crates/yahoo_finance_api/0.3.1
