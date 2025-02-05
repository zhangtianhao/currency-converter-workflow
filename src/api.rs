use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use chrono::{DateTime, Utc};
use reqwest::blocking::get;
use serde_json::Value;
use crate::CURRENCY_NAMES_CN;
use crate::model::CurrencyInfo;

const API_URL: &str = "https://open.er-api.com/v6/latest/USD";
const CACHE_HOURS: i64 = 12;

// 获取汇率数据（带缓存）
pub fn fetch_rates(cache_path: &PathBuf) -> Result<HashMap<String, CurrencyInfo>, String> {
    if let Ok(metadata) = fs::metadata(cache_path) {
        if let Ok(modified) = metadata.modified() {
            let cache_time: DateTime<Utc> = modified.into();
            if Utc::now().signed_duration_since(cache_time).num_hours() < CACHE_HOURS {
                let data = fs::read_to_string(cache_path).map_err(|e| e.to_string())?;
                return serde_json::from_str(&data).map_err(|e| e.to_string());
            }
        }
    }

    let response = get(API_URL)
        .map_err(|e| e.to_string())?
        .text()
        .map_err(|e| e.to_string())?;

    let data: Value = serde_json::from_str(&response).map_err(|e| e.to_string())?;
    let rates = data["rates"].as_object().ok_or("无效API响应")?;

    let mut currencies = HashMap::new();
    for (code, rate) in rates {
        if let Some(rate) = rate.as_f64() {
            if let Some(&(country, coin)) = CURRENCY_NAMES_CN
                .iter()
                .find(|&&(c, _)| c == code.as_str())
                .map(|(_, names)| names) {
                currencies.insert(
                    code.clone(), CurrencyInfo::new(rate, country.to_string(), coin.to_string()),
                );
            }
        }
    }

    let json = serde_json::to_string(&currencies).map_err(|e| e.to_string())?;
    fs::write(cache_path, json).map_err(|e| e.to_string())?;

    Ok(currencies)
}