# LongbridgeA 工作计划

> **目标**：A 股选股与分析工具（Rust + gpui-kit + 腾讯行情）。无交易功能。
>
> **当前阶段**：Session 已绑定到页面组件（`AppState` + `components::*`）。

---

## Phase 0–1：数据层 ✅

## Phase 2：UI 壳层

- [x] tokens / `AppShell` / 页面 ViewModel
- [x] `WatchlistSession` / `StockDetailSession`
- [x] **组件绑定**：`WatchlistTableView` / `QuoteHeaderView` / `DepthBookView` / `ChartPanelView`
- [x] **`AppState`**：shell + sessions + `bind_*` / `render_*_bound`
- [ ] gpui-kit 窗口：`Render` 映射上述 View 结构体

## 绑定关系

```text
Tencent client / QuoteHub
        │
   Session (app)
        │ writes
        ▼
  Page model (ui::pages)
        │ bind()
        ▼
  Component view (ui::components)
        │ render_text()  ── headless
        │ gpui Element   ── 待做
```

---

## 本地命令

```bash
cargo test -p longbridgea-ui
LONGBRIDGEA_LIVE=1 LONGBRIDGEA_MODE=watchlist cargo run -p longbridgea-app
LONGBRIDGEA_LIVE=1 LONGBRIDGEA_MODE=stock LONGBRIDGEA_PERIOD=m5 \
  LONGBRIDGEA_SYMBOL=sh600519 cargo run -p longbridgea-app
```

---

**更新**：2026-11-15  
**仓库**：https://github.com/vampireelon/LongbridgeA
