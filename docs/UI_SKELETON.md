# UI 骨架与组件绑定

> 更新：2026-11-15

## 分层

| 层 | Crate | 职责 |
|----|-------|------|
| Session | `app` | `WatchlistSession` / `StockDetailSession` 拉数写 page |
| Page model | `ui::pages` | 状态与业务字段 |
| Component | `ui::components` | `bind(page) → View`，可 `render_text` |
| AppState | `app::state` | shell + sessions + `bind_*` 聚合 |
| gpui view | 待做 | 把 View 画成 Element |

`ui` **不**依赖 `tencent-quote`。

## 组件

| Component | 数据源 |
|-----------|--------|
| `WatchlistTableView` | `WatchlistPage` |
| `QuoteHeaderView` | `StockDetailPage.quote` |
| `DepthBookView` | 五档 |
| `ChartPanelView` | 分时 / K 线序列 |

## AppState API

```rust
app.watchlist.set_symbols(&syms).await?;
app.watchlist.refresh_once().await;
let table = app.bind_watchlist_table().await; // WatchlistTableView
print!("{}", table.render_text());

app.focus_stock(sym, ChartPeriod::Minute).await;
app.stock.load_all().await?;
print!("{}", app.render_stock_bound().await);
```

同一 `Arc<RwLock<Page>>` 供后续 gpui 订阅。
