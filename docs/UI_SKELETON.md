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

gpui-kit 视图绑定通过 feature `gpui` 预留，默认关闭，保证 CI / `cargo test` 轻量。

---

## 模块

| 模块 | 职责 |
|------|------|
| `theme` | `ColorTheme` / `ThemeMode` / `ChangeColorScheme` / spacing |
| `nav` | `Route` / `NavItem` |
| `shell` | `AppShell` / `Tab` / `ShellLayout` |
| `pages::*` | 各页状态与列定义 |
| `widgets::price_change` | 涨跌展示文案与颜色 |

依赖方向仍为：`ui → core`（+ chart），**不**直接依赖 `tencent-quote`。行情由 `app` 注入 `QuoteHub` 后写入 page model。

---

## 启用 gpui-kit（可选）

```toml
# crates/ui/Cargo.toml 已声明
gpui = ["dep:gpui-kit"]
```

```bash
cargo check -p longbridgea-ui --features gpui
```

需要本机安装 gpui-kit 文档中的系统依赖；Rust 工具链建议跟随 gpui-kit 要求（可能高于 workspace 的 1.85）。窗口 `Render` 实现放在后续提交。

---

## 与数据层衔接

```text
QuoteHub (poll) → WatchlistPage.apply_quotes
fetch_search    → AppShell.search_* / 打开 StockDetail Tab
fetch_minute/kline/mkline → StockDetailPage
```

---

## 验证

```bash
cargo test -p longbridgea-ui
cargo run -p longbridgea-app
```
