use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A multi-currency balance.
/// Maps a currency code (e.g., "USD", "SAR") to a balance amount.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CurrencyBalance(pub HashMap<String, f64>);

impl CurrencyBalance {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn get(&self, currency: &str) -> f64 {
        self.0.get(currency).copied().unwrap_or(0.0)
    }

    pub fn add(&mut self, currency: &str, amount: f64) {
        let current = self.get(currency);
        self.0.insert(currency.to_string(), current + amount);
    }

    pub fn subtract(&mut self, currency: &str, amount: f64) {
        let current = self.get(currency);
        self.0.insert(currency.to_string(), current - amount);
    }
}

impl From<HashMap<String, f64>> for CurrencyBalance {
    fn from(map: HashMap<String, f64>) -> Self {
        Self(map)
    }
}
