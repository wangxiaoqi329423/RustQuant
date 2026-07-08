//! CSV 行情数据读取模块

use crate::model::MarketData;
use std::error::Error;
use std::fs::File;
use std::path::Path;

/// 从 CSV 文件加载行情数据
///
/// CSV 文件需包含列头：`date,open,high,low,close,volume`，
/// 与 `MarketData` 字段名一一对应，通过 serde 自动反序列化。
pub fn load_market_data(path: &str) -> Result<Vec<MarketData>, Box<dyn Error>> {
    let file = File::open(Path::new(path))?;
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);

    let mut records = Vec::new();
    for result in reader.deserialize() {
        let record: MarketData = result?;
        records.push(record);
    }

    Ok(records)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    const SAMPLE_CSV: &str = "\
date,open,high,low,close,volume
2024-01-02,100.0,105.0,98.0,102.5,1000000.0
2024-01-03,102.5,107.0,101.0,106.0,1200000.0
2024-01-04,106.0,108.0,103.5,104.0,950000.0
";

    #[test]
    fn test_load_market_data_count() {
        let path = write_temp_csv(SAMPLE_CSV);
        let result = load_market_data(&path);
        assert!(result.is_ok(), "Failed to load CSV: {:?}", result.err());

        let records = result.unwrap();
        assert_eq!(
            records.len(),
            3,
            "Expected 3 records, got {}",
            records.len()
        );
    }

    #[test]
    fn test_load_market_data_close_prices() {
        let path = write_temp_csv(SAMPLE_CSV);
        let records = load_market_data(&path).expect("Failed to load CSV");

        assert!((records[0].close - 102.5).abs() < 1e-10);
        assert!((records[1].close - 106.0).abs() < 1e-10);
        assert!((records[2].close - 104.0).abs() < 1e-10);
    }

    #[test]
    fn test_load_market_data_file_not_found() {
        let result = load_market_data("nonexistent_file.csv");
        assert!(result.is_err());
    }

    /// 将 CSV 内容写入临时文件，返回文件路径
    fn write_temp_csv(content: &str) -> String {
        let dir = std::env::temp_dir();
        let path = dir.join("rustquant_test_data.csv");
        let mut file = File::create(&path).expect("Failed to create temp file");
        file.write_all(content.as_bytes())
            .expect("Failed to write temp file");
        path.to_str().unwrap().to_string()
    }
}
