# LongbridgeA 工作计划

> **目标**：A 股选股与分析工具（Rust + gpui-kit + 腾讯行情）。无交易功能。
>
> **当前阶段**：自选表 ↔ QuoteHub 无窗口数据流已跑通。

---

## Phase 0–1：基础设施 + 数据层 ✅

## Phase 2：UI 壳层

- [x] 设计 tokens / `AppShell` / 页面 ViewModel / `PriceChangeView`
- [x] **无窗口数据流**：`WatchlistSession`（`WatchlistPage` ↔ `QuoteHub`）
- [ ] gpui-kit 窗口：`Render` 壳 + Sidebar / TabBar
- [ ] 自选表 GUI 绑定（复用 Session）
- [ ] 个股 QuoteHeader + 图表占位

## Phase 3+：选股器执行、图表指标、WASM 等

见 `docs/DESIGN.md`。

---

## 本地命令

```bash
cargo test -p longbridgea-core -p tencent-quote -p longbridgea-ui
cargo run -p longbridgea-app

# 无窗口自选轮询（默认 10s，茅台/平安/上证）
LONGBRIDGEA_LIVE=1 cargo run -p longbridgea-app

# 自定义时长与标的
LONGBRIDGEA_LIVE=1 LONGBRIDGEA_WATCH_SECS=12 \
  LONGBRIDGEA_SYMBOLS=sh600519,sz000001,sh000001 \
  cargo run -p longbridgea-app
```

数据流：

```text
WatchlistPage.symbols → QuoteHub.watch
QuoteHub.poll / broadcast QuoteTick
    → WatchlistPage.apply_quotes (保序)
    → render_table (CLI)
```

---

**更新**：2026-11-15  
**仓库**：https://github.com/vampireelon/LongbridgeA
