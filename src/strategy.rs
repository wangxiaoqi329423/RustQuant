//! 交易策略抽象接口

use crate::model::MarketData;

/// 交易信号
#[derive(Debug, Clone, PartialEq)]
pub enum Signal {
    /// 买入
    Buy,
    /// 卖出
    Sell,
    /// 持有 / 无操作
    Hold,
}

/// 策略 trait
///
/// 所有具体策略需实现此接口。
/// `generate_signal` 接收历史行情切片（按时间升序），返回当前应执行的交易信号。
pub trait Strategy {
    fn generate_signal(&self, data: &[MarketData]) -> Signal;
}

/// 双均线策略
///
/// 比较短期与长期移动平均线：
/// - 短期 > 长期 → Buy（上涨趋势，做多）
/// - 短期 < 长期 → Sell（下跌趋势，做空/离场）
/// - 相等 → Hold
///
/// 数据不足 `long_window` 时返回 `Hold`，不会 panic。
pub struct MovingAverageStrategy {
    short_window: usize,
    long_window: usize,
}

impl MovingAverageStrategy {
    pub fn new(short_window: usize, long_window: usize) -> Self {
        Self {
            short_window,
            long_window,
        }
    }
}

impl Strategy for MovingAverageStrategy {
    fn generate_signal(&self, data: &[MarketData]) -> Signal {
        if data.len() < self.long_window {
            return Signal::Hold;
        }

        let closes: Vec<f64> = data.iter().map(|d| d.close).collect();

        let short_ma = simple_moving_average(&closes[closes.len() - self.short_window..]);
        let long_ma = simple_moving_average(&closes[closes.len() - self.long_window..]);

        if short_ma > long_ma {
            Signal::Buy
        } else if short_ma < long_ma {
            Signal::Sell
        } else {
            Signal::Hold
        }
    }
}

/// 计算简单移动平均
fn simple_moving_average(values: &[f64]) -> f64 {
    values.iter().sum::<f64>() / values.len() as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── 用于测试的简单策略 ──

    /// 永远返回 Buy 的策略（仅用于 trait 实现测试）
    struct AlwaysBuy;

    impl Strategy for AlwaysBuy {
        fn generate_signal(&self, _data: &[MarketData]) -> Signal {
            Signal::Buy
        }
    }

    /// 永远返回 Hold 的策略
    struct AlwaysHold;

    impl Strategy for AlwaysHold {
        fn generate_signal(&self, _data: &[MarketData]) -> Signal {
            Signal::Hold
        }
    }

    // ── 测试 ──

    #[test]
    fn test_signal_variants() {
        let buy = Signal::Buy;
        let sell = Signal::Sell;
        let hold = Signal::Hold;

        assert_ne!(buy, sell);
        assert_ne!(buy, hold);
        assert_ne!(sell, hold);

        let clone = buy.clone();
        assert_eq!(buy, clone);
    }

    #[test]
    fn test_always_buy_strategy() {
        let strategy = AlwaysBuy;
        let empty_data: Vec<MarketData> = vec![];

        let signal = strategy.generate_signal(&empty_data);
        assert_eq!(signal, Signal::Buy);
    }

    #[test]
    fn test_always_hold_strategy() {
        let strategy = AlwaysHold;
        let empty_data: Vec<MarketData> = vec![];

        let signal = strategy.generate_signal(&empty_data);
        assert_eq!(signal, Signal::Hold);
    }

    #[test]
    fn test_strategy_with_real_data() {
        let data = vec![
            MarketData::new("2024-01-02", 100.0, 105.0, 98.0, 102.5, 1_000_000.0),
            MarketData::new("2024-01-03", 102.5, 107.0, 101.0, 106.0, 1_200_000.0),
        ];

        // AlwaysBuy 应该忽略数据，始终返回 Buy
        let signal = AlwaysBuy.generate_signal(&data);
        assert_eq!(signal, Signal::Buy);
    }

    // ── MovingAverageStrategy 测试 ──

    fn make_bars(closes: &[f64]) -> Vec<MarketData> {
        closes
            .iter()
            .enumerate()
            .map(|(i, &close)| {
                MarketData::new(
                    format!("2024-01-{:02}", i + 2),
                    close - 1.0,
                    close + 1.0,
                    close - 2.0,
                    close,
                    1_000_000.0,
                )
            })
            .collect()
    }

    #[test]
    fn test_ma_strategy_uptrend_buy() {
        // close: 10, 12, 14, 16 — 持续上涨
        let data = make_bars(&[10.0, 12.0, 14.0, 16.0]);
        let strategy = MovingAverageStrategy::new(2, 3);
        // short_ma = (14+16)/2 = 15, long_ma = (12+14+16)/3 = 14 → Buy
        assert_eq!(strategy.generate_signal(&data), Signal::Buy);
    }

    #[test]
    fn test_ma_strategy_downtrend_sell() {
        // close: 16, 14, 12, 10 — 持续下跌
        let data = make_bars(&[16.0, 14.0, 12.0, 10.0]);
        let strategy = MovingAverageStrategy::new(2, 3);
        // short_ma = (12+10)/2 = 11, long_ma = (14+12+10)/3 = 12 → Sell
        assert_eq!(strategy.generate_signal(&data), Signal::Sell);
    }

    #[test]
    fn test_ma_strategy_insufficient_data_hold() {
        // 只有 2 条数据，不足 long_window=3
        let data = make_bars(&[10.0, 12.0]);
        let strategy = MovingAverageStrategy::new(2, 3);
        assert_eq!(strategy.generate_signal(&data), Signal::Hold);
    }

    #[test]
    fn test_ma_strategy_equal_averages_hold() {
        // 所有 close 相同 → MA 相等 → Hold
        let data = make_bars(&[10.0, 10.0, 10.0, 10.0]);
        let strategy = MovingAverageStrategy::new(2, 3);
        assert_eq!(strategy.generate_signal(&data), Signal::Hold);
    }
}
