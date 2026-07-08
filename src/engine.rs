//! 回测引擎模块

use crate::model::MarketData;
use crate::strategy::{Signal, Strategy};

/// 单笔交易记录
#[derive(Debug, Clone)]
pub struct Trade {
    pub date: String,
    pub signal: Signal,
    pub price: f64,
    pub shares: u64,
}

/// 回测结果
#[derive(Debug, Clone)]
pub struct BacktestResult {
    pub initial_capital: f64,
    pub final_capital: f64,
    pub total_return: f64,
    pub trade_count: usize,
    pub trades: Vec<Trade>,
}

/// 回测引擎（无状态，仅保存初始资金）
pub struct BacktestEngine {
    initial_capital: f64,
}

impl BacktestEngine {
    pub fn new(initial_capital: f64) -> Self {
        Self { initial_capital }
    }

    /// 执行回测
    ///
    /// 逐日向策略推送历史窗口（`data[0..=i]`），根据信号执行全仓买卖。
    /// 成交价统一使用当日收盘价。
    pub fn run(&self, data: &[MarketData], strategy: &dyn Strategy) -> BacktestResult {
        let mut portfolio = Portfolio::new(self.initial_capital);
        let mut trades = Vec::new();

        for i in 0..data.len() {
            let window = &data[0..=i];
            let signal = strategy.generate_signal(window);
            let price = data[i].close;

            match signal {
                Signal::Buy => {
                    if let Some(trade) = portfolio.buy(&data[i].date, price) {
                        trades.push(trade);
                    }
                }
                Signal::Sell => {
                    if let Some(trade) = portfolio.sell(&data[i].date, price) {
                        trades.push(trade);
                    }
                }
                Signal::Hold => { /* 无操作 */ }
            }
        }

        let final_price = data.last().map(|d| d.close).unwrap_or(0.0);
        let final_capital = portfolio.value(final_price);

        BacktestResult {
            initial_capital: self.initial_capital,
            final_capital,
            total_return: (final_capital - self.initial_capital) / self.initial_capital,
            trade_count: trades.len(),
            trades,
        }
    }
}

/// 投资组合（现金 + 持仓）
struct Portfolio {
    cash: f64,
    shares: u64,
}

impl Portfolio {
    fn new(cash: f64) -> Self {
        Self { cash, shares: 0 }
    }

    /// 总资产 = 现金 + 持仓市值
    fn value(&self, price: f64) -> f64 {
        self.cash + self.shares as f64 * price
    }

    /// 全仓买入，返回交易记录；资金不足时返回 None
    fn buy(&mut self, date: &str, price: f64) -> Option<Trade> {
        let max_shares = (self.cash / price) as u64;
        if max_shares == 0 {
            return None;
        }
        self.cash -= max_shares as f64 * price;
        self.shares += max_shares;
        Some(Trade {
            date: date.to_string(),
            signal: Signal::Buy,
            price,
            shares: max_shares,
        })
    }

    /// 全仓卖出，返回交易记录；无持仓时返回 None
    fn sell(&mut self, date: &str, price: f64) -> Option<Trade> {
        if self.shares == 0 {
            return None;
        }
        self.cash += self.shares as f64 * price;
        let sold = self.shares;
        self.shares = 0;
        Some(Trade {
            date: date.to_string(),
            signal: Signal::Sell,
            price,
            shares: sold,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::MarketData;

    fn make_bars(closes: &[f64]) -> Vec<MarketData> {
        closes
            .iter()
            .enumerate()
            .map(|(i, &close)| {
                MarketData::new(
                    format!("2024-01-{:02}", i + 2),
                    close - 0.5,
                    close + 0.5,
                    close - 1.0,
                    close,
                    1_000_000.0,
                )
            })
            .collect()
    }

    // ── 测试用策略 ──

    struct BuyOnThirdDay;
    impl Strategy for BuyOnThirdDay {
        fn generate_signal(&self, data: &[MarketData]) -> Signal {
            if data.len() == 3 {
                Signal::Buy
            } else {
                Signal::Hold
            }
        }
    }

    struct AlwaysSell;
    impl Strategy for AlwaysSell {
        fn generate_signal(&self, _data: &[MarketData]) -> Signal {
            Signal::Sell
        }
    }

    struct AlwaysBuy;
    impl Strategy for AlwaysBuy {
        fn generate_signal(&self, _data: &[MarketData]) -> Signal {
            Signal::Buy
        }
    }

    struct AlwaysHold;
    impl Strategy for AlwaysHold {
        fn generate_signal(&self, _data: &[MarketData]) -> Signal {
            Signal::Hold
        }
    }

    // ── 测试 ──

    #[test]
    fn test_buy_and_profit_in_uptrend() {
        // 连续上涨：第3天买入，最终持仓盈利
        let data = make_bars(&[100.0, 101.0, 102.0, 103.0, 104.0]);
        let engine = BacktestEngine::new(100_000.0);
        let result = engine.run(&data, &BuyOnThirdDay);

        assert_eq!(result.trade_count, 1);
        assert!(result.final_capital > result.initial_capital);
        assert!(result.total_return > 0.0);
    }

    #[test]
    fn test_sell_with_no_shares_ignored() {
        // 空仓时 Sell 信号应被忽略
        let data = make_bars(&[100.0, 101.0, 102.0]);
        let engine = BacktestEngine::new(100_000.0);
        let result = engine.run(&data, &AlwaysSell);

        assert_eq!(result.trade_count, 0);
        assert_eq!(result.final_capital, result.initial_capital);
    }

    #[test]
    fn test_buy_with_no_cash_ignored() {
        // 资金为0时 Buy 信号应被忽略
        let data = make_bars(&[100.0]);
        let engine = BacktestEngine::new(50.0); // 不够买1股
        let result = engine.run(&data, &AlwaysBuy);

        assert_eq!(result.trade_count, 0);
    }

    #[test]
    fn test_hold_produces_no_trades() {
        let data = make_bars(&[100.0, 101.0, 102.0, 103.0]);
        let engine = BacktestEngine::new(100_000.0);
        let result = engine.run(&data, &AlwaysHold);

        assert_eq!(result.trade_count, 0);
        assert_eq!(result.final_capital, result.initial_capital);
    }

    #[test]
    fn test_buy_then_sell_roundtrip() {
        // 第3天Buy，第4天Sell，验证完整交易链路
        let data = make_bars(&[100.0, 101.0, 102.0, 103.0]);

        // 先用 BuyOnThirdDay 买入
        let engine = BacktestEngine::new(100_000.0);

        // 手动模拟：第3天买入
        struct BuyThenSell;
        impl Strategy for BuyThenSell {
            fn generate_signal(&self, data: &[MarketData]) -> Signal {
                match data.len() {
                    3 => Signal::Buy,
                    4 => Signal::Sell,
                    _ => Signal::Hold,
                }
            }
        }

        let result = engine.run(&data, &BuyThenSell);

        assert_eq!(result.trade_count, 2);
        assert_eq!(result.trades[0].signal, Signal::Buy);
        assert_eq!(result.trades[1].signal, Signal::Sell);
        // 卖出后持仓应为0，且因为是平盘（买入价=卖出价=102），资金不变
        // 按收盘价买入/卖出 same price，基本持平
    }

    #[test]
    fn test_empty_data() {
        let data: Vec<MarketData> = vec![];
        let engine = BacktestEngine::new(100_000.0);
        let result = engine.run(&data, &AlwaysBuy);

        assert_eq!(result.trade_count, 0);
        assert_eq!(result.final_capital, result.initial_capital);
        assert_eq!(result.total_return, 0.0);
    }
}
