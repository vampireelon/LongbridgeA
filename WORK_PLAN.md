# LongbridgeA 工作计划

> **目标**：A 股选股与分析工具（Rust + gpui-kit + 腾讯行情）。无交易功能。
>
> **当前阶段**：Phase 1 数据层基本完成（含缓存与轮询订阅）。

---

## Phase 0：基础设施 ✅

## Phase 1：数据层

- [x] 实时快照 `qt.gtimg.cn`（批量 + GBK + 五档）
- [x] 分时 `minute/query`
- [x] K 线 `fqkline/get`（日/周/月 + 前/后/不复权）
- [x] 分钟 K `mkline`（m1/m5/m15/m30/m60）
- [x] 股票搜索 smartbox
- [x] 模型：`Symbol` / `Quote` / `Bar` / `MinuteSeries` / `SearchHit` / `MinuteInterval`
- [x] 内存缓存：`QuoteCache`（TTL）+ `KlineCache`
- [x] 轮询订阅：`QuoteHub`（watch 集合 + `poll_once` / 后台 `start` + `broadcast` `QuoteTick`）

## Phase 2+：UI（gpui-kit）等

见 `docs/DESIGN.md`。

---

## 本地命令

```bash
cargo test -p longbridgea-core -p tencent-quote -p longbridgea-chart
cargo run -p longbridgea-app
LONGBRIDGEA_LIVE=1 cargo run -p longbridgea-app
```

数据 API：

- `TencentQuoteClient::fetch_quotes` / `fetch_minute` / `fetch_kline` / `fetch_mkline` / `fetch_search`
- `QuoteCache` / `KlineCache`
- `QuoteHub::{watch, unwatch, poll_once, start, stop, subscribe_ticks, get_quote}`

---

**更新**：2026-11-15  
**仓库**：https://github.com/vampireelon/LongbridgeA
