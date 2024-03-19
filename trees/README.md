# Our crate Set of Binary Trees (Red-Black tree and AVL tree)
- ECE421 Group Project 2, due Mar 20, 2024  
- Version: 1.0.0?
- Authors: Prabh Kooner, Brandon Hoynick, Jiannan Lu  

## Red-Black Tree - Crate Usage Setup (*fill in)
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

## Red-Black Tree - Crate Usage Examples (*fill in)
After running the program via executable args:
1. Open the generated `plot_<stock_ticker>.html` file (in the same directory).
2. Navigate browser chart; clicking on the top-right options (e.g. zoom) allows you to select spots on the chart to enhance (e.g. zoom: allows you to select a smaller range of dates to zoom into); Selecting the legends will show/hide the 'daily closing price' and 'volatile day' traces.
3. The terminal will also display the dates of the highest and lowest closing prices for the last 6-months.

After running the program without executable args:
1. The program will prompt you to enter a stock ticker. e.g.: `aapl` or `GOOGL`
- If there is a valid stock ticker, it will check for data and generate a plot via html (or error if not valid);
2. Open the generated `plot_<stock_ticker>.html` file (in the same directory).
3. Navigate browser chart; clicking on the top-right options (e.g. zoom) allows you to select spots on the chart to enhance (e.g. zoom: allows you to select a smaller range of dates to zoom into); Selecting the legends will show/hide the 'daily closing price' and 'volatile day' traces.
4. The terminal will also display the dates of the highest and lowest closing prices for the last 6-months.

## AVL Tree - Crate Usage Setup (*fill in)
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

## AVL Tree - Crate Usage Examples (*fill in)
After running the program via executable args:
1. Open the generated `plot_<stock_ticker>.html` file (in the same directory).
2. Navigate browser chart; clicking on the top-right options (e.g. zoom) allows you to select spots on the chart to enhance (e.g. zoom: allows you to select a smaller range of dates to zoom into); Selecting the legends will show/hide the 'daily closing price' and 'volatile day' traces.
3. The terminal will also display the dates of the highest and lowest closing prices for the last 6-months.

After running the program without executable args:
1. The program will prompt you to enter a stock ticker. e.g.: `aapl` or `GOOGL`
- If there is a valid stock ticker, it will check for data and generate a plot via html (or error if not valid);
2. Open the generated `plot_<stock_ticker>.html` file (in the same directory).
3. Navigate browser chart; clicking on the top-right options (e.g. zoom) allows you to select spots on the chart to enhance (e.g. zoom: allows you to select a smaller range of dates to zoom into); Selecting the legends will show/hide the 'daily closing price' and 'volatile day' traces.
4. The terminal will also display the dates of the highest and lowest closing prices for the last 6-months.