//! 市场数据结构
//!
//! 定义量化回测引擎使用的核心数据类型。

/// 单根 K 线行情数据
///
/// 对应 CSV 中一行 OHLCV 记录，通过 `serde::Deserialize` 直接从 CSV 反序列化。
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct MarketData {
    /// 交易日期 (YYYY-MM-DD)
    pub date: String,
    /// 开盘价
    pub open: f64,
    /// 最高价
    pub high: f64,
    /// 最低价
    pub low: f64,
    /// 收盘价
    pub close: f64,
    /// 成交量
    pub volume: f64,
}

impl MarketData {
    /// 创建新的行情数据记录
    pub fn new(
        date: impl Into<String>,
        open: f64,
        high: f64,
        low: f64,
        close: f64,
        volume: f64,
    ) -> Self {
        Self {
            date: date.into(),
            open,
            high,
            low,
            close,
            volume,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_market_data_creation() {
        let bar = MarketData::new("2024-01-02", 100.0, 105.0, 98.0, 102.5, 1000000.0);

        assert_eq!(bar.date, "2024-01-02");
        assert_eq!(bar.open, 100.0);
        assert_eq!(bar.high, 105.0);
        assert_eq!(bar.low, 98.0);
        assert_eq!(bar.close, 102.5);
        assert_eq!(bar.volume, 1000000.0);
    }

    #[test]
    fn test_market_data_clone() {
        let bar = MarketData::new("2024-01-03", 50.0, 55.0, 49.0, 53.0, 500000.0);
        let cloned = bar.clone();
        assert_eq!(bar.date, cloned.date);
        assert_eq!(bar.close, cloned.close);
    }
}
