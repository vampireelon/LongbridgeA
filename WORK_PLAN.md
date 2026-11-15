# LongbridgeA 工作计划

> **目标**：A 股选股与分析工具（Rust + gpui-kit + 腾讯行情）。无交易功能。
>
> **当前阶段**：Phase 2 壳层骨架（tokens / nav / pages）；gpui 窗口待接线。

---

## Phase 0：基础设施 ✅

## Phase 1：数据层 ✅

- [x] 快照 / 分时 / 日K / 分钟K / smartbox
- [x] `QuoteCache` + `QuoteHub` 轮询订阅

## Phase 2：UI 壳层

- [x] 设计 tokens（`theme`）与涨跌色方案
- [x] `AppShell`：侧栏路由、Tab、搜索状态
- [x] 页面 ViewModel：自选 / 个股 / 选股 / 行情 / 设置
- [x] `PriceChangeView` 等纯逻辑组件
- [x] 文档 `docs/UI_SKELETON.md`；feature `gpui` 预留
- [ ] gpui-kit 窗口：`Render` 壳 + Sidebar / TabBar
- [ ] 自选表绑定 `QuoteHub`
- [ ] 个股 QuoteHeader + 图表占位

## Phase 3+：选股器执行、图表指标、WASM 等

见 `docs/DESIGN.md`。

---

## 本地命令

```bash
cargo test -p longbridgea-core -p tencent-quote -p longbridgea-ui
cargo run -p longbridgea-app
LONGBRIDGEA_LIVE=1 cargo run -p longbridgea-app
# 可选（需系统依赖）:
# cargo check -p longbridgea-ui --features gpui
```

---

**更新**：2026-11-15  
**仓库**：https://github.com/vampireelon/LongbridgeA
