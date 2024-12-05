use aggregator_sj::PriceService;
use std::error::Error;
use std::io::{self, Write};
use tracing::{debug, info};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize tracing with debug level
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    info!("Starting Cryptocurrency Price Aggregator");
    
    // Create PriceService instance
    let price_service = PriceService::new();
    
    println!("=== Cryptocurrency Price Aggregator ===");
    println!("Commands:");
    println!("  <coin symbol> - Show prices for a coin");
    println!("  list         - Show supported coins");
    println!("  exit         - Exit the program");

    let mut input = String::new();
    loop {
        input.clear();
        print!("> ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut input)?;

        let command = input.trim();
        match command {
            "exit" => {
                debug!("Received exit command");
                break;
            }
            "list" => {
                debug!("Fetching supported coins list");
                let coins = price_service.fetch_supported_coins().await?;
                println!("\n=== Supported Coins ===");
                for (symbol, ids) in &coins {
                    println!("{:<6} -> {}", symbol, ids.join(", "));
                }
            }
            symbol => {
                debug!("Fetching prices for symbol: {}", symbol);
                price_service.fetch_and_display_prices(symbol).await?
            }
        }
    }

    info!("Shutting down");
    Ok(())
}