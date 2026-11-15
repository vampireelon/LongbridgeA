# LongbridgeA 工作计划

> **目标**：A 股选股与分析工具（Rust + gpui-kit + 腾讯行情）。无交易功能。
>
> **当前阶段**：自选 + 个股详情无窗口数据流已跑通。

---

## Phase 0–1：基础设施 + 数据层 ✅

## Phase 2：UI 壳层

- [x] 设计 tokens / `AppShell` / 页面 ViewModel / `PriceChangeView`
- [x] 无窗口：`WatchlistSession`（自选 ↔ QuoteHub）
- [x] 无窗口：`StockDetailSession`（报价轮询 + 分时/日K/分钟K）
- [ ] gpui-kit 窗口：`Render` 壳 + Sidebar / TabBar
- [ ] 自选表 / 个股 GUI 绑定（复用 Session）

## Phase 3+：选股器执行、图表指标、WASM 等

见 `docs/DESIGN.md`。

---

## 本地命令

```bash
cargo test -p longbridgea-core -p tencent-quote -p longbridgea-ui
cargo run -p longbridgea-app

# 自选
LONGBRIDGEA_LIVE=1 cargo run -p longbridgea-app
LONGBRIDGEA_LIVE=1 LONGBRIDGEA_MODE=watchlist \
  LONGBRIDGEA_SYMBOLS=sh600519,sz000001 cargo run -p longbridgea-app

# 个股详情（默认分时）
LONGBRIDGEA_LIVE=1 LONGBRIDGEA_MODE=stock \
  LONGBRIDGEA_SYMBOL=sh600519 LONGBRIDGEA_PERIOD=minute \
  cargo run -p longbridgea-app

# 日K / 5分K
LONGBRIDGEA_LIVE=1 LONGBRIDGEA_MODE=stock LONGBRIDGEA_PERIOD=day \
  LONGBRIDGEA_SYMBOL=sh600519 cargo run -p longbridgea-app
LONGBRIDGEA_LIVE=1 LONGBRIDGEA_MODE=stock LONGBRIDGEA_PERIOD=m5 \
  LONGBRIDGEA_SYMBOL=sz000001 cargo run -p longbridgea-app
```

个股数据流：

```text
open(symbol, period)
  → hub.watch([symbol])
load_all
  → refresh_quote (hub.get_quote / cache)
  → refresh_chart
        minute  → fetch_minute
        day/week/month → fetch_kline (qfq)
        m5/… → fetch_mkline
run_headless
  → QuoteTick → apply_quote
  → 可选定期 refresh_chart
  → render_summary
```

---

**更新**：2026-11-15  
**仓库**：https://github.com/vampireelon/LongbridgeA
