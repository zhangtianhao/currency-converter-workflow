use std::collections::HashMap;
use crate::model::CurrencyInfo;
use crate::PRIORITY;

pub fn match_currencies<'a>(
    search: &str,
    currencies: &'a HashMap<String, CurrencyInfo>,
) -> Vec<(&'a String, &'a CurrencyInfo)> {
    let search = search.to_lowercase();
    let mut matched = Vec::new();

    for (code, info) in currencies {
        if code.to_lowercase().starts_with(&search) {
            matched.push((code, info));
            continue;
        }

        let coin_name = format!("{}s", info.coin).to_lowercase();
        if coin_name.starts_with(&search) {
            matched.push((code, info));
        }
    }

    // 优先排序常用货币
    matched.sort_by(|a, b| {
        let a_prio = PRIORITY.iter().position(|&c| c == a.0).unwrap_or(100);
        let b_prio = PRIORITY.iter().position(|&c| c == b.0).unwrap_or(100);
        a_prio.cmp(&b_prio)
    });

    matched
}