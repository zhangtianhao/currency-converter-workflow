use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CurrencyInfo {
    pub rate: f64,
    pub country: String,
    pub coin: String,
}

impl CurrencyInfo {
    pub fn new(rate: f64, country: String, coin: String) -> Self {
        Self {
            rate,
            country,
            coin,
        }
    }
}