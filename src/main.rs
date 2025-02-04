use std::{collections::HashMap, env, fs, path::PathBuf};
use chrono::{DateTime, Utc};
use reqwest::blocking::get;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use regex::Regex;

const CACHE_HOURS: i64 = 12;
const API_URL: &str = "https://open.er-api.com/v6/latest/USD";
const ICON_PATH: &str = "images/flags";

const PRIORITY: [&str; 7] = ["CNY", "USD", "BHD", "EUR", "HKD", "GBP", "JPY"]; // 优先货币列表

#[derive(Serialize, Deserialize, Debug, Clone)]
struct CurrencyInfo {
    rate: f64,
    country: String,
    coin: String,
}

static CURRENCY_NAMES_CN: &[(&str, (&str, &str))] = &[
    ("AED", ("阿联酋", "迪拉姆")),
    ("AFN", ("阿富汗", "阿富汗尼")),
    ("ALL", ("阿尔巴尼亚", "列克")),
    ("AMD", ("亚美尼亚", "德拉姆")),
    ("ANG", ("荷兰属安地列斯", "盾")),
    ("AOA", ("安哥拉", "宽扎")),
    ("ARS", ("阿根廷", "比索")),
    ("AUD", ("澳大利亚", "澳元")),
    ("AWG", ("阿鲁巴", "弗罗林")),
    ("AZN", ("阿塞拜疆", "马纳特")),
    ("BAM", ("波斯尼亚和黑塞哥维那", "可兑换马克")),
    ("BBD", ("巴巴多斯", "元")),
    ("BDT", ("孟加拉", "塔卡")),
    ("BGN", ("保加利亚", "列弗")),
    ("BHD", ("巴林", "第纳尔")),
    ("BIF", ("布隆迪", "法郎")),
    ("BMD", ("百慕大", "元")),
    ("BND", ("文莱", "元")),
    ("BOB", ("玻利维亚", "玻利维亚诺")),
    ("BRL", ("巴西", "雷亚尔")),
    ("BSD", ("巴哈马", "元")),
    ("BTN", ("不丹", "努尔特鲁姆")),
    ("BWP", ("博茨瓦纳", "普拉")),
    ("BYN", ("白俄罗斯", "卢布")),
    ("BZD", ("伯利兹", "元")),
    ("CAD", ("加拿大", "加元")),
    ("CDF", ("刚果民主共和国", "法郎")),
    ("CHF", ("瑞士", "法郎")),
    ("CLP", ("智利", "比索")),
    ("CNY", ("中国", "人民币")),
    ("COP", ("哥伦比亚", "比索")),
    ("CRC", ("哥斯达黎加", "科朗")),
    ("CUP", ("古巴", "比索")),
    ("CVE", ("佛得角", "埃斯库多")),
    ("CZK", ("捷克", "克朗")),
    ("DJF", ("吉布提", "法郎")),
    ("DKK", ("丹麦", "克朗")),
    ("DOP", ("多米尼加", "比索")),
    ("DZD", ("阿尔及利亚", "第纳尔")),
    ("EGP", ("埃及", "镑")),
    ("ERN", ("厄立特里亚", "纳克法")),
    ("ETB", ("埃塞俄比亚", "比尔")),
    ("EUR", ("欧盟", "欧元")),
    ("FJD", ("斐济", "元")),
    ("FKP", ("福克兰群岛", "镑")),
    ("FOK", ("法罗群岛", "克朗")),
    ("GBP", ("英国", "英镑")),
    ("GEL", ("格鲁吉亚", "拉里")),
    ("GGP", ("根西岛", "镑")),
    ("GHS", ("加纳", "塞地")),
    ("GIP", ("直布罗陀", "镑")),
    ("GMD", ("冈比亚", "达拉西")),
    ("GNF", ("几内亚", "法郎")),
    ("GTQ", ("危地马拉", "格查尔")),
    ("GYD", ("圭亚那", "元")),
    ("HKD", ("香港", "港元")),
    ("HNL", ("洪都拉斯", "伦皮拉")),
    ("HRK", ("克罗地亚", "库纳")),
    ("HTG", ("海地", "古德")),
    ("HUF", ("匈牙利", "福林")),
    ("IDR", ("印度尼西亚", "卢比")),
    ("ILS", ("以色列", "新谢克尔")),
    ("IMP", ("马恩岛", "镑")),
    ("INR", ("印度", "卢比")),
    ("IQD", ("伊拉克", "第纳尔")),
    ("IRR", ("伊朗", "里亚尔")),
    ("ISK", ("冰岛", "克朗")),
    ("JEP", ("泽西岛", "镑")),
    ("JMD", ("牙买加", "元")),
    ("JOD", ("约旦", "第纳尔")),
    ("JPY", ("日本", "日元")),
    ("KES", ("肯尼亚", "先令")),
    ("KGS", ("吉尔吉斯斯坦", "索姆")),
    ("KHR", ("柬埔寨", "瑞尔")),
    ("KID", ("基里巴斯", "元")),
    ("KMF", ("科摩罗", "法郎")),
    ("KRW", ("韩国", "韩元")),
    ("KWD", ("科威特", "第纳尔")),
    ("KYD", ("开曼群岛", "元")),
    ("KZT", ("哈萨克斯坦", "坚戈")),
    ("LAK", ("老挝", "基普")),
    ("LBP", ("黎巴嫩", "镑")),
    ("LKR", ("斯里兰卡", "卢比")),
    ("LRD", ("利比里亚", "元")),
    ("LSL", ("莱索托", "洛蒂")),
    ("LYD", ("利比亚", "第纳尔")),
    ("MAD", ("摩洛哥", "迪拉姆")),
    ("MDL", ("摩尔多瓦", "列伊")),
    ("MGA", ("马达加斯加", "阿里亚里")),
    ("MKD", ("北马其顿", "代纳尔")),
    ("MMK", ("缅甸", "缅元")),
    ("MNT", ("蒙古", "图格里克")),
    ("MOP", ("澳门", "澳门元")),
    ("MRU", ("毛里塔尼亚", "乌吉亚")),
    ("MUR", ("毛里求斯", "卢比")),
    ("MVR", ("马尔代夫", "拉菲亚")),
    ("MWK", ("马拉维", "克瓦查")),
    ("MXN", ("墨西哥", "比索")),
    ("MYR", ("马来西亚", "令吉")),
    ("MZN", ("莫桑比克", "梅蒂卡尔")),
    ("NAD", ("纳米比亚", "元")),
    ("NGN", ("尼日利亚", "奈拉")),
    ("NIO", ("尼加拉瓜", "科多巴")),
    ("NOK", ("挪威", "克朗")),
    ("NPR", ("尼泊尔", "卢比")),
    ("NZD", ("新西兰", "元")),
    ("OMR", ("阿曼", "里亚尔")),
    ("PAB", ("巴拿马", "巴波亚")),
    ("PEN", ("秘鲁", "索尔")),
    ("PGK", ("巴布亚新几内亚", "基那")),
    ("PHP", ("菲律宾", "比索")),
    ("PKR", ("巴基斯坦", "卢比")),
    ("PLN", ("波兰", "兹罗提")),
    ("PYG", ("巴拉圭", "瓜拉尼")),
    ("QAR", ("卡塔尔", "里亚尔")),
    ("RON", ("罗马尼亚", "列伊")),
    ("RSD", ("塞尔维亚", "第纳尔")),
    ("RUB", ("俄罗斯", "卢布")),
    ("RWF", ("卢旺达", "法郎")),
    ("SAR", ("沙特阿拉伯", "里亚尔")),
    ("SBD", ("所罗门群岛", "元")),
    ("SCR", ("塞舌尔", "卢比")),
    ("SDG", ("苏丹", "镑")),
    ("SEK", ("瑞典", "克朗")),
    ("SGD", ("新加坡", "元")),
    ("SHP", ("圣赫勒拿", "镑")),
    ("SLE", ("塞拉利昂", "新利昂")),
    ("SLL", ("塞拉利昂", "利昂")),
    ("SOS", ("索马里", "先令")),
    ("SRD", ("苏里南", "元")),
    ("SSP", ("南苏丹", "镑")),
    ("STN", ("圣多美和普林西比", "多布拉")),
    ("SYP", ("叙利亚", "镑")),
    ("SZL", ("斯威士兰", "里兰吉尼")),
    ("THB", ("泰国", "泰铢")),
    ("TJS", ("塔吉克斯坦", "索莫尼")),
    ("TMT", ("土库曼斯坦", "马纳特")),
    ("TND", ("突尼斯", "第纳尔")),
    ("TOP", ("汤加", "潘加")),
    ("TRY", ("土耳其", "里拉")),
    ("TTD", ("特立尼达和多巴哥", "元")),
    ("TVD", ("图瓦卢", "元")),
    ("TWD", ("台湾地区", "新台币")),
    ("TZS", ("坦桑尼亚", "先令")),
    ("UAH", ("乌克兰", "格里夫纳")),
    ("UGX", ("乌干达", "先令")),
    ("USD", ("美国", "美元")),
    ("UYU", ("乌拉圭", "比索")),
    ("UZS", ("乌兹别克斯坦", "苏姆")),
    ("VES", ("委内瑞拉", "玻利瓦尔")),
    ("VND", ("越南", "盾")),
    ("VUV", ("瓦努阿图", "瓦图")),
    ("WST", ("萨摩亚", "塔拉")),
    ("XAF", ("中非金融共同体", "法郎")),
    ("XCD", ("东加勒比", "元")),
    ("XDR", ("国际货币基金组织", "特别提款权")),
    ("XOF", ("西非金融共同体", "法郎")),
    ("XPF", ("太平洋法郎", "法郎")),
    ("YER", ("也门", "里亚尔")),
    ("ZAR", ("南非", "兰特")),
    ("ZMW", ("赞比亚", "克瓦查")),
    ("ZWL", ("津巴布韦", "元"))
];


#[derive(Serialize)]
struct AlfredOutput {
    items: Vec<AlfredItem>,
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

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = args.get(1).map(|s| s.as_str()).unwrap_or_default();

    // 获取缓存路径
    // ~/Library/Caches/com.runningwithcrayons.Alfred/Workflow Data/com.alfredapp.currency-converter
    // let cache_dir = env::var("alfred_workflow_cache").unwrap();
    let cache_dir = String::from("./cache");
    let cache_path = PathBuf::from(cache_dir).join("ratesUSD.json");

    // 获取汇率数据
    let currencies = match fetch_rates(&cache_path) {
        Ok(c) => c,
        Err(e) => {
            show_error(&format!("获取汇率失败: {}", e));
            return;
        }
    };

    // 解析输入
    let (raw_num, parts) = parse_input(input);
    let number = match raw_num.parse::<f64>() {
        Ok(n) if n > 0.0 => n,
        _ => {
            show_instructions();
            return;
        }
    };

    // 处理不同阶段
    let output = match parts.as_slice() {
        [] => show_all_currencies(number, &currencies),
        [src] => show_source_currencies(number, src, &currencies),
        [src, dst] => convert_currency(number, src, dst, &currencies),
        _ => {
            show_error("无效输入格式");
            return;
        }
    };

    println!("{}", output);
}

// 获取汇率数据（带缓存）
fn fetch_rates(cache_path: &PathBuf) -> Result<HashMap<String, CurrencyInfo>, String> {
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
                .map(|(_, names)| names)
            {
                currencies.insert(
                    code.clone(),
                    CurrencyInfo {
                        rate,
                        country: country.to_string(),
                        coin: coin.to_string(),
                    },
                );
            }
        }
    }

    let json = serde_json::to_string(&currencies).map_err(|e| e.to_string())?;
    fs::write(cache_path, json).map_err(|e| e.to_string())?;

    Ok(currencies)
}

// 解析输入
fn parse_input(input: &str) -> (String, Vec<String>) {
    // 步骤1：清理输入并提取数字部分
    let cleaned = input.replace(|c: char| c.is_ascii_punctuation() && c != '.' && c != '-', " ")
        .to_lowercase();

    // 步骤2：使用正则表达式精确匹配数字和货币代码的组合
    let re = Regex::new(r"(?x)
        ^
        (\d*\.?\d+)    # 匹配数字（含小数）
        ([a-z]{1,3})     # 匹配1到3字母货币代码
        (?:\s+([a-z]+))? # 可选的其他货币代码
        $
    ").unwrap();

    if let Some(caps) = re.captures(&cleaned) {
        // 处理类似 "14.1USD" 的情况
        let num_str = caps.get(1).unwrap().as_str();
        let first_code = caps.get(2).unwrap().as_str();
        let second_code = caps.get(3).map(|m| m.as_str()).unwrap_or_default();

        return (
            num_str.to_string(),
            vec![first_code.to_string(), second_code.to_string()]
                .into_iter()
                .filter(|s| !s.is_empty())
                .collect()
        );
    }

    // 步骤3：处理其他格式（带空格分隔）
    let mut parts: Vec<&str> = cleaned.split_whitespace().collect();

    // 处理类似 "100USD" 的情况
    if parts.len() == 1 {
        let re_compact = Regex::new(r"^(\d+\.?\d*)([a-z]{3})$").unwrap();
        if let Some(caps) = re_compact.captures(parts[0]) {
            return (
                caps.get(1).unwrap().as_str().to_string(),
                vec![caps.get(2).unwrap().as_str().to_string()]
            );
        }
    }

    // 步骤4：常规解析
    let (num_part, rest): (Vec<&str>, Vec<&str>) = parts
        .iter()
        .cloned()
        .partition(|s| s.parse::<f64>().is_ok());

    let num_str = num_part.first().cloned().unwrap_or_default();
    let rest = rest.into_iter()
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect();

    (num_str.to_string(), rest)
}

fn match_currencies<'a>(
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

fn show_error(message: &str) -> String {
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

fn show_instructions() -> String {
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

fn show_all_currencies(amount: f64, currencies: &HashMap<String, CurrencyInfo>) -> String {
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

fn show_source_currencies(
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

fn convert_currency(
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

// 为AlfredOutput添加序列化方法
impl AlfredOutput {
    fn to_json(self) -> String {
        serde_json::to_string(&self).unwrap_or_else(|_| "{\"items\":[]}".to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    #[test]
    fn test_parse_input() {
        // 带小数点的紧凑格式
        assert_eq!(
            parse_input("14.1USD"),
            ("14.1".into(), vec!["usd".into()])
        );

        // 带目标货币的紧凑格式
        assert_eq!(
            parse_input("100.5EUR CNY"),
            ("100.5".into(), vec!["eur".into(), "cny".into()])
        );

        // 常规带空格格式
        assert_eq!(
            parse_input("123.45 gbp to jpy"),
            ("123.45".into(), vec!["gbp".into(), "to".into(), "jpy".into()])
        );

        // 混合格式
        assert_eq!(
            parse_input("500usd,cny"),
            ("500".into(), vec!["usd".into(), "cny".into()])
        );
    }
}