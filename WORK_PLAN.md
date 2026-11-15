# LongbridgeA 工作计划

> **目标**：A 股选股与分析工具（Rust + gpui-kit + 腾讯行情）。无交易功能。
>
> **当前阶段**：Phase 1 — 快照 / 分时 / 日K / 分钟K / 搜索 已实现。

---

## Phase 0：基础设施 ✅

## Phase 1：数据层

- [x] 实时快照 `qt.gtimg.cn`（批量 + GBK + 五档）
- [x] 分时 `minute/query`
- [x] K 线 `fqkline/get`（日/周/月 + 前/后/不复权）
- [x] 分钟 K `mkline`（m1/m5/m15/m30/m60，host=`ifzq.gtimg.cn`）
- [x] 股票搜索 smartbox（`fetch_search` + `SearchHit`）
- [x] 模型：`Symbol` / `Quote` / `Bar` / `MinuteSeries` / `SearchHit` / `MinuteInterval`
- [ ] 缓存与订阅

## Phase 2+：UI（gpui-kit）等

见 `docs/DESIGN.md`。

---

## 本地命令

```bash
cargo test -p longbridgea-core -p tencent-quote -p longbridgea-chart
cargo run -p longbridgea-app
LONGBRIDGEA_LIVE=1 cargo run -p longbridgea-app
```

API：

- `TencentQuoteClient::fetch_quotes`
- `TencentQuoteClient::fetch_minute`
- `TencentQuoteClient::fetch_kline(symbol, period, adjust, count)`
- `TencentQuoteClient::fetch_mkline(symbol, interval, count)`
- `TencentQuoteClient::fetch_search(keyword) -> Vec<SearchHit>`

---

**更新**：2026-11-15  
**仓库**：https://github.com/vampireelon/LongbridgeA
