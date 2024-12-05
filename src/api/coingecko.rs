use crate::models::cache::{CacheEntry, CACHE};
use reqwest::Client;
use std::error::Error;
use std::time::Duration;
use tokio::time::sleep;
use std::collections::HashMap;
use std::time::SystemTime;
use tracing::debug;

pub struct CoingeckoApi {
    client: Client,
}

impl CoingeckoApi {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn get_with_cache(&self, url: &str) -> Result<serde_json::Value, Box<dyn Error>> {
        const CACHE_DURATION: u64 = 300; // 5 minutes for regular data
        const COINS_LIST_CACHE_DURATION: u64 = 3600; // 1 hour for coins list
        
        // Use longer cache for coins list
        let cache_duration = if url.contains("/coins/list") {
            COINS_LIST_CACHE_DURATION
        } else {
            CACHE_DURATION
        };

        // Check cache first
        let mut cache = CACHE.lock().unwrap();
        if let Some(entry) = cache.get(url) {
            if let Ok(elapsed) = entry.timestamp.elapsed() {
                if elapsed.as_secs() < cache_duration {
                    debug!("Cache hit for {}", url);
                    return Ok(entry.data.clone());
                }
            }
        }

        // If not in cache or expired, fetch new data
        debug!("Cache miss for {}", url);
        let response = self.client.get(url).send().await?;
        
        if response.status() == 429 {
            debug!("Rate limit hit, checking cache");
            if let Some(entry) = cache.get(url) {
                return Ok(entry.data.clone());
            }
            return Err("Rate limit reached".into());
        }

        let json: serde_json::Value = response.json().await?;
        
        cache.insert(url.to_string(), CacheEntry {
            data: json.clone(),
            timestamp: SystemTime::now(),
        });

        Ok(json)
    }

    pub async fn fetch_supported_coins(&self) -> Result<HashMap<String, Vec<String>>, Box<dyn Error>> {
        let url = "https://api.coingecko.com/api/v3/coins/list";
        
        debug!("Sending request to {}", url);
        let json = self.get_with_cache(url).await?;  // Use our cache method
        
        let mut coin_map: HashMap<String, Vec<String>> = HashMap::new();
        
        // Parse as array of objects
        let coins = json.as_array()
            .ok_or("Expected array of coins")?;
        
        debug!("Parsed {} coins from response", coins.len());

        for coin in coins {
            if let (Some(symbol), Some(id)) = (
                coin.get("symbol").and_then(|s| s.as_str()),
                coin.get("id").and_then(|i| i.as_str()),
            ) {
                coin_map.entry(symbol.to_uppercase())
                    .or_insert_with(Vec::new)
                    .push(id.to_string());
            }
        }
        debug!("Created map with {} entries", coin_map.len());

        Ok(coin_map)
    }
}
