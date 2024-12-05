use colored::Colorize;
use prettytable::{Table, Row, Cell, format};
use std::fmt::Display;

pub struct DisplayFormatter;

impl DisplayFormatter {
    pub fn new() -> Self {
        Self
    }

    pub fn format_header(&self, text: &str) -> String {
        format!("\n=== {} ===", text.bright_white().bold())
    }

    pub fn format_price_table(&self, headers: &[&str], rows: &[Vec<String>]) -> String {
        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
        
        // Add headers
        table.add_row(Row::new(
            headers.iter().map(|h| Cell::new(h).style_spec("b")).collect()
        ));

        // Add data rows
        for row in rows {
            table.add_row(Row::new(
                row.iter().map(|cell| Cell::new(cell)).collect()
            ));
        }

        table.to_string()
    }

    pub fn format_colored_change(&self, change: f64) -> String {
        if change >= 0.0 {
            format!("+{:.2}%", change).green().to_string()
        } else {
            format!("{:.2}%", change).red().to_string()
        }
    }

    pub fn format_currency(&self, amount: f64) -> String {
        if amount >= 1.0 {
            format!("${:.2}", amount)
        } else {
            format!("${:.6}", amount)
        }
    }

    pub fn format_coin_summary(&self, coin_data: &CoinSummary) -> String {
        let mut output = Vec::new();
        output.push(self.format_header(&format!("{} ({})", coin_data.symbol, coin_data.id)));
        output.push(format!("Exchange: {}", coin_data.exchange));
        output.push(format!("Price: {}", self.format_currency(coin_data.price)));
        output.push(format!("Market Cap: {}", self.format_currency(coin_data.market_cap)));
        output.push(format!("24h Volume: {}", self.format_currency(coin_data.volume_24h)));
        output.push(format!("24h Change: {}", self.format_colored_change(coin_data.price_change_24h)));
        
        output.join("\n")
    }
}

#[derive(Debug)]
pub struct CoinSummary {
    pub symbol: String,
    pub id: String,
    pub exchange: String,
    pub price: f64,
    pub market_cap: f64,
    pub volume_24h: f64,
    pub price_change_24h: f64,
}