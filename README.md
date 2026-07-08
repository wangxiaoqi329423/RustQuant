# RustQuant

> A lightweight quantitative trading backtesting engine built with Rust.

## 项目简介

RustQuant 是一个基于 Rust 开发的轻量级量化交易回测系统，用于模拟历史行情下交易策略的执行过程，并统计收益情况。

本项目为 Rust 课程大作业，采用模块化设计，实现了历史数据读取、交易策略、回测引擎、指标统计等完整流程。

---

## 功能特性

- ✅ CSV 历史行情读取
- ✅ MarketData 数据模型
- ✅ Strategy Trait 策略抽象
- ✅ 双均线（Moving Average）策略
- ✅ 回测引擎
- ✅ 收益统计
- ✅ 交易历史输出
- ✅ 22 个单元测试

---

## 项目架构

```
CSV Data
    │
    ▼
Data Loader
    │
    ▼
MarketData
    │
    ▼
Strategy
    │
    ▼
Backtest Engine
    │
    ▼
Metrics
    │
    ▼
Console Output
```

---

## 项目目录

```text
RustQuant
│
├── Cargo.toml
├── README.md
├── data
│   └── sample.csv
├── src
│   ├── main.rs
│   ├── model.rs
│   ├── data.rs
│   ├── strategy.rs
│   ├── engine.rs
│   └── metrics.rs
└── tests
```

---

## 快速开始

### 克隆项目

```bash
git clone https://github.com/你的用户名/RustQuant.git
```

### 进入项目

```bash
cd RustQuant
```

### 编译

```bash
cargo build
```

### 运行

```bash
cargo run
```

### 测试

```bash
cargo test
```

---

## 示例输出

```
======================================
RustQuant Backtest Engine
======================================

Loaded Records: 30

=== Backtest Metrics ===

Initial Capital:    100000.00
Final Capital:      101356.00
Total Return:           1.36%
Trade Count:                1
```

---

## 技术特点

- Rust 模块化架构
- Trait 策略抽象
- Result 错误处理
- CSV 数据解析
- 单元测试覆盖
- 无未来函数设计

---

## 后续计划

未来计划支持：

- 多资产回测
- 手续费模型
- 滑点模拟
- 最大回撤
- Sharpe Ratio
- 图形化界面（GUI）

---

## License

This project is for educational purposes.