# UI 骨架说明（Phase 2）

> 与 `docs/DESIGN.md` 对齐的可编译壳层  
> 更新：2026-11-15

---

## 目标

在不强制链接原生窗口库的前提下，先落地：

1. **设计 tokens**（深/浅色、红涨绿跌）
2. **Shell 状态机**（侧栏路由、Tab、搜索开关）
3. **页面 ViewModel**（自选 / 个股 / 选股 / 行情 / 设置）
4. **纯逻辑组件**（`PriceChangeView` 等）
5. **无窗口自选数据流**（`WatchlistSession`）

gpui-kit 视图绑定通过 feature `gpui` 预留，默认关闭。

---

## 模块

| 模块 | 职责 |
|------|------|
| `theme` | `ColorTheme` / `ThemeMode` / `ChangeColorScheme` |
| `nav` | `Route` / `NavItem` |
| `shell` | `AppShell` / `Tab` / `ShellLayout` |
| `pages::*` | 各页状态与列定义 |
| `widgets::price_change` | 涨跌展示文案与颜色 |

App 侧：

| 模块 | 职责 |
|------|------|
| `watchlist_session` | `WatchlistPage` ↔ `QuoteHub`：`set_symbols` / `refresh_once` / `run_headless` |

依赖方向：`ui → core`；`app → ui + tencent-quote`。行情不进入 `ui` crate。

---

## 无窗口自选数据流

```text
set_symbols([sh600519, …])
    → page.active_group.symbols
    → hub.watch
refresh_once / hub.start
    → QuoteTick
    → page.apply_quotes  （默认按自选顺序，不重排）
    → page.render_table
```

```bash
LONGBRIDGEA_LIVE=1 cargo run -p longbridgea-app
LONGBRIDGEA_LIVE=1 LONGBRIDGEA_WATCH_SECS=12 \
  LONGBRIDGEA_SYMBOLS=sh600519,sz000001 cargo run -p longbridgea-app
```

后续 gpui 自选表只订阅同一 `WatchlistSession`（`page: Arc<RwLock<…>>` + ticks）。

---

## 启用 gpui-kit（可选）

```bash
cargo check -p longbridgea-ui --features gpui
```

---

## 验证

```bash
cargo test -p longbridgea-ui
cargo run -p longbridgea-app
```
