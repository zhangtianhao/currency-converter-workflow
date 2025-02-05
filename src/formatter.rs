use std::collections::HashMap;
use serde::Serialize;
use crate::model::CurrencyInfo;
use crate::PRIORITY;
use serde_json::Value;
use crate::matcher::match_currencies;

const ICON_PATH: &str = "images/flags";
#[derive(Serialize)]
struct AlfredOutput {
    items: Vec<AlfredItem>,
}

// 为AlfredOutput添加序列化方法
impl AlfredOutput {
    fn to_json(self) -> String {
        serde_json::to_string(&self).unwrap_or_else(|_| "{\"items\":[]}".to_string())
    }
}

#[derive(Serialize)]
struct AlfredItem {
    title: String,
    subtitle: String,
    arg: Option<String>,
    autocomplete: Option<String>,
    icon: Icon,
    valid: bool,
}

#[derive(Serialize)]
struct Icon {
    path: String,
}

pub fn show_error(message: &str) -> String {
    let output = AlfredOutput {
        items: vec![AlfredItem {
            title: message.to_string(),
            subtitle: "".to_string(),
            arg: None,
            autocomplete: None,
            icon: Icon {
                path: "".to_string(),
            },
            valid: false,
        }],
    };
    serde_json::to_string(&output).unwrap()
}

pub fn show_instructions() -> String {
    let output = AlfredOutput {
        items: vec![AlfredItem {
            title: "输入金额和货币".to_string(),
            subtitle: "示例：100 CNY".to_string(),
            arg: None,
            autocomplete: None,
            icon: Icon {
                path: "".to_string(),
            },
            valid: false,
        }],
    };
    serde_json::to_string(&output).unwrap()
}


pub fn show_all_currencies(amount: f64, currencies: &HashMap<String, CurrencyInfo>) -> String {
    let mut items = Vec::new();
    // 先添加优先货币
    for code in &PRIORITY {
        if let Some(info) = currencies.get(*code) {
            items.push(create_currency_item(
                amount,
                code,
                info,
                true, // 需要自动补全
            ));
        }
    }

    // 添加其他货币（排除已添加的优先货币）
    for (code, info) in currencies {
        if !PRIORITY.contains(&code.as_str()) {
            items.push(create_currency_item(
                amount,
                code,
                info,
                true,
            ));
        }
    }

    AlfredOutput { items }.to_json()
}

// 辅助函数：创建货币展示项
fn create_currency_item(
    amount: f64,
    code: &str,
    info: &CurrencyInfo,
    with_autocomplete: bool,
) -> AlfredItem {
    AlfredItem {
        title: format!("{} {}", amount, code),
        subtitle: format!("{} {}", info.country, info.coin),
        arg: None,
        autocomplete: if with_autocomplete {
            Some(format!("{} {} to ", amount, code))
        } else {
            None
        },
        icon: Icon {
            path: format!("{}/{}.png", ICON_PATH, code),
        },
        valid: false,
    }
}


pub fn show_source_currencies(
    amount: f64,
    src: &str,
    currencies: &HashMap<String, CurrencyInfo>,
) -> String {
    let matches = match_currencies(src, currencies);

    if matches.is_empty() {
        return show_error("未找到匹配的货币");
    }

    if matches.len() > 1 {
        // 已经通过match_currencies排序过优先级
        let items = matches
            .into_iter()
            .map(|(code, info)| create_currency_item(amount, code, info, true))
            .collect();
        return AlfredOutput { items }.to_json();
    }

    // 单个匹配时显示目标货币选择（带优先级）
    let (src_code, src_info) = matches[0];
    let mut items = Vec::new();

    // 先添加优先货币
    for code in &PRIORITY {
        if let Some(info) = currencies.get(*code) {
            if *code != src_code {
                items.push(create_conversion_item(amount, src_code, src_info, code, info));
            }
        }
    }

    // 添加其他货币
    for (code, info) in currencies {
        if !PRIORITY.contains(&code.as_str()) && code != src_code {
            items.push(create_conversion_item(amount, src_code, src_info, code, info));
        }
    }

    AlfredOutput { items }.to_json()
}

pub fn convert_currency(
    amount: f64,
    src: &str,
    dst: &str,
    currencies: &HashMap<String, CurrencyInfo>,
) -> String {
    let src_matches = match_currencies(src, currencies);
    let dst_matches = match_currencies(dst, currencies);

    if src_matches.is_empty() || dst_matches.is_empty() {
        return show_error("无效的货币代码");
    }

    let (src_code, src_info) = &src_matches[0];
    let items: Vec<_> = dst_matches
        .into_iter()
        .filter(|(code, _)| *code != *src_code)
        .map(|(dst_code, dst_info)| {
            let converted = (amount * dst_info.rate / src_info.rate * 100.0).round() / 100.0;
            AlfredItem {
                title: format!("{} {}", converted, dst_code),
                subtitle: format!(
                    "{} {} → {} {}",
                    src_info.country, src_info.coin, dst_info.country, dst_info.coin
                ),
                arg: Some(converted.to_string()),
                autocomplete: Some(format!("{} {} {}", amount, src_code, dst_code)),
                icon: Icon {
                    path: format!("{}/{}.png", ICON_PATH, dst_code),
                },
                valid: true,
            }
        })
        .collect();

    if items.is_empty() {
        show_error("不能转换相同货币")
    } else {
        AlfredOutput { items }.to_json()
    }
}


fn create_conversion_item(
    amount: f64,
    src_code: &str,
    src_info: &CurrencyInfo,
    dst_code: &str,
    dst_info: &CurrencyInfo,
) -> AlfredItem {
    let converted = (amount * dst_info.rate / src_info.rate * 100.0).round() / 100.0;
    AlfredItem {
        title: format!("{} {}", converted, dst_code),
        subtitle: format!(
            "{} {} → {} {}",
            src_info.country, src_info.coin, dst_info.country, dst_info.coin
        ),
        arg: Some(converted.to_string()),
        autocomplete: Some(format!("{} {} {}", amount, src_code, dst_code)),
        icon: Icon {
            path: format!("{}/{}.png", ICON_PATH, dst_code),
        },
        valid: true,
    }
}
