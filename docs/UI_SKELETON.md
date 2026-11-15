# UI 骨架与 gpui 绑定

## 分层

| 层 | 说明 |
|----|------|
| Session | app 内拉数写 `pages::*` |
| Component | `bind` → `*View` / `render_text` |
| ShellSnapshot | `Arc<Mutex<_>>`，GUI 与后台共享 |
| RootShell | gpui `Render`，画侧栏 + 主区文本 |

## Feature

```toml
# app
gpui = ["longbridgea-ui/gpui", "dep:gpui-kit"]
```

```bash
LONGBRIDGEA_GUI=1 cargo run -p longbridgea-app --features gpui
```

要求：Rust 版本满足 gpui-kit（官方文档倾向较新稳定版，可能 ≥1.92），并安装对应平台原生依赖。

## 刷新说明

当前实现：后台更新 Snapshot；窗口在布局/系统触发时重绘可读到新文本。后续可在 `RootShell` 上挂 `cx.spawn` 定时 `cx.notify()` 强制刷新。
