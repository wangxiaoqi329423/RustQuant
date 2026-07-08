mod data;
mod engine;
mod metrics;
mod model;
mod strategy;

use data::load_market_data;
use engine::BacktestEngine;
use metrics::calculate_metrics;
use strategy::MovingAverageStrategy;

fn main() {
    println!("======================================");
    println!("RustQuant Backtest Engine");
    println!("======================================");
    println!();

    let records = match load_market_data("data/sample.csv") {
        Ok(r) => {
            println!("Loaded Records: {}", r.len());
            r
        }
        Err(e) => {
            eprintln!("Error: Failed to load data/sample.csv");
            eprintln!("Details: {}", e);
            eprintln!();
            eprintln!("Please ensure:");
            eprintln!("  1. The file 'data/sample.csv' exists");
            eprintln!("  2. The CSV format is: date,open,high,low,close,volume");
            return;
        }
    };

    let strategy = MovingAverageStrategy::new(5, 20);
    let engine = BacktestEngine::new(100_000.0);

    let result = engine.run(&records, &strategy);
    let metrics = calculate_metrics(&result);

    println!();
    println!("{}", metrics);

    println!();
    println!("======================================");
    println!("Trade History");
    println!("======================================");

    if result.trades.is_empty() {
        println!();
        println!("No trades executed.");
    } else {
        let max_display = 5;
        let total = result.trades.len();

        for (i, trade) in result.trades.iter().enumerate() {
            if i >= max_display {
                break;
            }
            let signal_str = match trade.signal {
                strategy::Signal::Buy => "BUY",
                strategy::Signal::Sell => "SELL",
                strategy::Signal::Hold => "HOLD",
            };
            println!();
            println!("Trade #{}", i + 1);
            println!("Date   : {}", trade.date);
            println!("Signal : {}", signal_str);
            println!("Price  : {:.2}", trade.price);
            println!("Shares : {}", trade.shares);
            println!("--------------------------------------");
        }

        if total > max_display {
            println!("... and {} more trades", total - max_display);
        }
    }

    println!();
    println!("======================================");
}
