# 项目结构说明

> LongbridgeA Cargo Workspace 与目录约定  
> 最后更新：2026-11-15

---

## 1. 目标

- 多 crate 清晰拆分：数据、领域模型、UI、图表、应用入口分离
- 便于先实现 `tencent-quote` 与 `core`，再挂 gpui-kit 应用
- 预留 Desktop 与 WASM 双目标，不在初期强行统一所有依赖 feature

---

## 2. 目录树（当前骨架）

```
LongbridgeA/
├── Cargo.toml                 # Workspace root
├── rust-toolchain.toml        # 可选，锁定稳定工具链
├── README.md
├── WORK_PLAN.md
├── LICENSE
├── .gitignore
├── docs/
│   ├── tencent-quote-api.md
│   ├── DESIGN.md
│   └── PROJECT_STRUCTURE.md   # 本文件
├── crates/
│   ├── app/                   # 可执行入口（Desktop；WASM 后续）
│   ├── tencent-quote/         # 腾讯行情 HTTP 客户端
│   ├── core/                  # 共享模型、选股规则、仓储接口
│   ├── ui/                    # 页面与业务组件（依赖 gpui-kit）
│   └── chart/                 # 分时 / K 线绘制与指标
└── assets/                    # 图标、主题等静态资源（占位）
```

---

## 3. Crate 职责

| Crate | 类型 | 职责 | 允许依赖 |
|-------|------|------|----------|
| `longbridgea-core` | lib | `Quote`/`Bar`/`Symbol` 等模型；选股条件 AST；错误类型；与 UI 无关的业务规则 | serde、thiserror、chrono 等，**不**依赖 gpui |
| `tencent-quote` | lib | 腾讯接口请求、GBK 解码、字段解析、实现 `core` 的行情 trait | reqwest、tokio、encoding_rs、core |
| `longbridgea-chart` | lib | K 线/分时几何与指标计算；尽量与具体 GUI 解耦，由 ui/app 喂数据绘制 | core；可选 plotters 等 |
| `longbridgea-ui` | lib | 自选、个股、选股器、市场页；主题与布局壳 | gpui-kit、core、chart |
| `longbridgea-app` | bin | `main`、窗口启动、依赖注入、配置加载 | ui、tencent-quote、core、chart |

依赖方向（单向）：

```
app → ui → chart → core
 app → tencent-quote → core
 ui ↛ tencent-quote（通过 core trait / app 注入）
```

避免 `ui` 直接依赖具体行情实现，方便以后换数据源。

---

## 4. Workspace `Cargo.toml` 约定

- `resolver = "2"`
- members：`crates/*`
- 公共依赖在 `[workspace.dependencies]` 统一版本
- 各 crate 用 `dependency = { workspace = true }` 引用

初期锁定思路（具体版本以创建时 crates.io 为准）：

- `tokio`、`reqwest`、`serde`、`serde_json`、`thiserror`、`anyhow`
- `gpui-kit` 在进入 Phase 2 再引入，避免骨架阶段强依赖 GUI 编译

---

## 5. 命名

- 包名（`package.name`）：`longbridgea-core`、`tencent-quote`、`longbridgea-chart`、`longbridgea-ui`、`longbridgea-app`
- 目录名：`core`、`tencent-quote`、`chart`、`ui`、`app`
- 库对外 `lib.name` 与包名一致或使用下划线形式（Rust 惯例）

---

## 6. 骨架阶段故意不做的事

- 不实现真实 HTTP 与解析逻辑（仅 mod / todo 占位）
- 不引入 gpui-kit 依赖（等 Phase 2）
- 不配置 WASM target 与 trunk
- 不添加 CI（可后续 GitHub Actions）

---

## 7. 本地验证

```bash
git clone https://github.com/vampireelon/LongbridgeA.git
cd LongbridgeA
cargo check
```

应能通过 workspace 内各 lib 的空/占位编译。

---

## 8. 后续演进

1. Phase 1：充实 `core` 模型 + `tencent-quote` 客户端与测试
2. Phase 2：`ui` / `app` 引入 gpui-kit，搭壳层
3. Phase 3+：`chart` 接入绘制；选股逻辑落在 `core`
4. 需要时增加 `crates/macros` 或 `crates/config`

详见 [WORK_PLAN.md](../WORK_PLAN.md)。
