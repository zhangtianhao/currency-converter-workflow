use std::{env, path::PathBuf};
use currency_converter::api::fetch_rates;
use currency_converter::formatter::{
    convert_currency, show_all_currencies, show_error, show_instructions, show_source_currencies
};
use currency_converter::parser::parse_input;

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