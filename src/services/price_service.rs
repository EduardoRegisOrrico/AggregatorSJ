use tracing::{info, error};
use crate::api::coingecko::CoingeckoApi;
use crate::utils::display::DisplayFormatter;
use std::error::Error;
use std::collections::HashMap;
use crate::utils::display::CoinSummary;

pub struct PriceService {
    api: CoingeckoApi,
    display: DisplayFormatter,
}

impl PriceService {
    pub fn new() -> Self {
        Self {
            api: CoingeckoApi::new(),
            display: DisplayFormatter::new(),
        }
    }

    pub async fn fetch_and_display_prices(&self, symbol: &str) -> Result<(), Box<dyn Error>> {
        info!("Fetching prices for symbol: {}", symbol);
        let coins = self.api.fetch_supported_coins().await?;
        let coin_ids = coins.get(&symbol.to_uppercase())
            .ok_or("Unsupported coin symbol")?;
        
        let filtered_coin_ids: Vec<String> = coin_ids.iter()
            .filter(|id| {
                !id.contains("wrapped") && 
                !id.contains("bridged") && 
                !id.contains("starkgate") &&
                !id.contains("osmosis") &&
                !id.contains("alleth") &&
                !id.contains("infinite-garden") &&
                !id.contains("wormhole")

            })
            .take(5)
            .cloned()
            .collect();
    
        let mut coin_details = Vec::<CoinSummary>::new();
    
        // Single pass: Get market data and tickers in one call
        for coin_id in &filtered_coin_ids {
            let details_url = format!(
                "https://api.coingecko.com/api/v3/coins/{}?localization=false&tickers=true&market_data=true&community_data=false&developer_data=false&sparkline=false",
                coin_id
            );
            
            match self.api.get_with_cache(&details_url).await {
                Ok(json) => {
                    if let (
                        Some(market_data),
                        Some(tickers)
                    ) = (
                        json.get("market_data"),
                        json.get("tickers").and_then(|t| t.as_array())
                    ) {
                        if let (
                            Some(price),
                            Some(market_cap),
                            Some(volume_24h),
                            Some(price_change_24h)
                        ) = (
                            market_data.get("current_price").and_then(|p| p.get("usd")).and_then(|p| p.as_f64()),
                            market_data.get("market_cap").and_then(|m| m.get("usd")).and_then(|m| m.as_f64()),
                            market_data.get("total_volume").and_then(|v| v.get("usd")).and_then(|v| v.as_f64()),
                            market_data.get("price_change_percentage_24h").and_then(|c| c.as_f64())
                        ) {
                            // Sort tickers by price in ascending order
                            let mut sorted_tickers: Vec<_> = tickers.iter().collect();
                            sorted_tickers.sort_by(|a, b| {
                                let price_a = a.get("converted_last").and_then(|p| p.get("usd")).and_then(|p| p.as_f64()).unwrap_or(f64::MAX);
                                let price_b = b.get("converted_last").and_then(|p| p.get("usd")).and_then(|p| p.as_f64()).unwrap_or(f64::MAX);
                                price_a.partial_cmp(&price_b).unwrap_or(std::cmp::Ordering::Equal)
                            });

                            // Get the cheapest exchange
                            let cheapest_exchange = sorted_tickers.first().and_then(|t| {
                                t.get("market").and_then(|m| m.get("name")).and_then(|n| n.as_str())
                            }).unwrap_or("Unknown");

                            // Create coin summary
                            let summary = CoinSummary {
                                symbol: symbol.to_uppercase(),
                                id: coin_id.clone(),
                                exchange: cheapest_exchange.to_string(),
                                price,
                                market_cap,
                                volume_24h,
                                price_change_24h,
                            };

                            // Display coin summary
                            println!("{}", self.display.format_coin_summary(&summary));

                            // Display exchange listings
                            println!("\nTop Exchange Prices:");
                            let headers = &["Exchange", "Price (USD)", "24h Volume"];
                            let mut rows = Vec::new();

                            for ticker in sorted_tickers.iter().take(5) {
                                if let (
                                    Some(market),
                                    Some(price),
                                    Some(volume)
                                ) = (
                                    ticker.get("market").and_then(|m| m.get("name")).and_then(|n| n.as_str()),
                                    ticker.get("converted_last").and_then(|p| p.get("usd")).and_then(|p| p.as_f64()),
                                    ticker.get("converted_volume").and_then(|v| v.get("usd")).and_then(|v| v.as_f64()),
                                ) {
                                    rows.push(vec![
                                        market.to_string(),
                                        self.display.format_currency(price),
                                        self.display.format_currency(volume),
                                    ]);
                                }
                            }
                            println!("{}", self.display.format_price_table(headers, &rows));
                        }
                    }
                }
                Err(e) => error!("Failed to fetch market data for {}: {}", coin_id, e)
            }
        }
        
        Ok(())
    }

    pub async fn fetch_supported_coins(&self) -> Result<HashMap<String, Vec<String>>, Box<dyn Error>> {
        self.api.fetch_supported_coins().await
    }
}