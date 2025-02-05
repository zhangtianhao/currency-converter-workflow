use regex::Regex;

// 解析输入
pub fn parse_input(input: &str) -> (String, Vec<String>) {
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


#[cfg(test)]
mod tests {
    use crate::parser::parse_input;

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