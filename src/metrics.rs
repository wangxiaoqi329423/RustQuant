//! 回测统计指标模块

use std::fmt;

use crate::engine::BacktestResult;

/// 回测统计指标
pub struct Metrics {
    pub initial_capital: f64,
    pub final_capital: f64,
    /// 收益率百分比（如 5.0 表示 5.00%）
    pub total_return_pct: f64,
    pub trade_count: usize,
}

/// 从回测结果计算统计指标
pub fn calculate_metrics(result: &BacktestResult) -> Metrics {
    Metrics {
        initial_capital: result.initial_capital,
        final_capital: result.final_capital,
        total_return_pct: result.total_return * 100.0,
        trade_count: result.trade_count,
    }
}

impl fmt::Display for Metrics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "=== Backtest Metrics ===\n\
             Initial Capital: {:>12.2}\n\
             Final Capital:   {:>12.2}\n\
             Total Return:    {:>11.2}%\n\
             Trade Count:     {:>12}",
            self.initial_capital, self.final_capital, self.total_return_pct, self.trade_count,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::{BacktestResult, Trade};
    use crate::strategy::Signal;

    #[test]
    fn test_metrics_profitable() {
        let result = BacktestResult {
            initial_capital: 100_000.0,
            final_capital: 105_200.0,
            total_return: 0.052,
            trade_count: 2,
            trades: vec![
                Trade {
                    date: "2024-01-02".into(),
                    signal: Signal::Buy,
                    price: 100.0,
                    shares: 1000,
                },
                Trade {
                    date: "2024-01-05".into(),
                    signal: Signal::Sell,
                    price: 105.2,
                    shares: 1000,
                },
            ],
        };

        let m = calculate_metrics(&result);

        assert_eq!(m.initial_capital, 100_000.0);
        assert_eq!(m.final_capital, 105_200.0);
        assert!((m.total_return_pct - 5.2).abs() < 1e-10);
        assert_eq!(m.trade_count, 2);
    }

    #[test]
    fn test_metrics_no_trades() {
        let result = BacktestResult {
            initial_capital: 100_000.0,
            final_capital: 100_000.0,
            total_return: 0.0,
            trade_count: 0,
            trades: vec![],
        };

        let m = calculate_metrics(&result);

        assert_eq!(m.total_return_pct, 0.0);
        assert_eq!(m.trade_count, 0);
        assert_eq!(m.initial_capital, m.final_capital);
    }

    #[test]
    fn test_display_formatting() {
        let result = BacktestResult {
            initial_capital: 100_000.0,
            final_capital: 105_200.0,
            total_return: 0.052,
            trade_count: 2,
            trades: vec![],
        };

        let m = calculate_metrics(&result);
        let output = format!("{}", m);

        assert!(output.contains("100000.00"));
        assert!(output.contains("105200.00"));
        assert!(output.contains("5.20%"));
        assert!(output.contains("2"));
    }
}
