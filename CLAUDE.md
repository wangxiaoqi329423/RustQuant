# RustQuant Project

## Role

你是一名高级 Rust 软件工程师。

负责开发一个课程大作业：

RustQuant：
基于 Rust 的轻量级量化交易回测引擎。


## Goal

在3天内完成稳定可运行版本。

优先级：

1. 稳定运行
2. 功能完整
3. 代码规范
4. 扩展性


## Scope

必须实现：

- CSV行情读取
- 市场数据结构
- Strategy trait
- 双均线策略
- 回测引擎
- 收益统计


禁止主动增加：

- GUI
- Web
- 数据库
- 深度学习
- 实时交易


## Coding Rules

使用：

- Rust stable
- idiomatic Rust
- Result错误处理
- struct组织数据
- trait抽象策略


避免：

- unwrap大量使用
- 不必要clone
- 单文件超过300行


## Development Workflow

每次修改前：

说明：

1. 修改文件
2. 修改原因
3. 影响范围


完成后：

必须运行：

cargo check

cargo test


## Architecture

model.rs:

数据结构

data.rs:

CSV读取

strategy.rs:

策略接口

engine.rs:

回测执行

metrics.rs:

指标计算


## Testing

每个模块需要：

至少一个测试。


## Important

不要为了增加功能降低稳定性。