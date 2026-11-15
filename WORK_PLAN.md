# LongbridgeA 工作计划

> **目标**：A 股选股与分析工具（Rust + gpui-kit + 腾讯行情）。无交易功能。

---

## Phase 0–1：数据层 ✅

## Phase 2：UI

- [x] tokens / AppShell / 页面 ViewModel / 组件 bind
- [x] WatchlistSession / StockDetailSession / AppState
- [x] **gpui 窗口壳（feature `gpui`）**：`RootShell` + 后台 Session 写 `ShellSnapshot`
- [ ] 侧栏点击切换 / 真正色板与表格组件（非纯 Label 文本）
- [ ] 窗口定时 `cx.notify` 刷新（当前依赖重新 render；可再加 timer）

---

## 运行

```bash
# 无窗口
LONGBRIDGEA_LIVE=1 LONGBRIDGEA_MODE=watchlist cargo run -p longbridgea-app
LONGBRIDGEA_LIVE=1 LONGBRIDGEA_MODE=stock LONGBRIDGEA_SYMBOL=sh600519 cargo run -p longbridgea-app

# 桌面窗口（需较新 Rust，见 gpui-kit 文档；本仓库沙箱 1.75 无法编译验证）
LONGBRIDGEA_GUI=1 cargo run -p longbridgea-app --features gpui
LONGBRIDGEA_GUI=1 LONGBRIDGEA_MODE=stock LONGBRIDGEA_PERIOD=m5 \
  LONGBRIDGEA_SYMBOL=sh600519 cargo run -p longbridgea-app --features gpui
```

数据 → UI：

```text
Session → Page → Component::bind → render_text
                         ↓
              ShellSnapshot (Mutex)
                         ↓
              RootShell (gpui Render)
```

---

**更新**：2026-11-15  
**仓库**：https://github.com/vampireelon/LongbridgeA
