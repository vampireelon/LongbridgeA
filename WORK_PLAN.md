# LongbridgeA 工作计划

> **目标**：使用 Rust + gpui-kit + WASM/Desktop 技术栈，复刻面向 A 股市场的 **选股与分析工具**（对标 Longbridge Pro 的行情/分析部分），并集成腾讯行情接口作为核心数据源。
>
> **明确范围调整**：搁置所有交易 / 下单 / 模拟盘功能，仅聚焦选股、行情展示、技术分析、基本面辅助与智能筛选。
>
> **当前阶段**：Phase 1 进行中 — 实时快照解析与 HTTP 客户端已落地。

---

## 1. 项目愿景与范围

### 1.1 核心目标
- 打造高性能、跨平台（macOS / Windows / Linux）的 A 股专业 **选股与分析** 桌面端。
- 视觉与交互对标 Longbridge Pro 的行情与分析体验。
- 数据层优先使用腾讯公开行情接口，后续可扩展其他源。
- 支持 WASM 与原生 Desktop。

### 1.2 非目标（明确排除）
- **不实现任何交易、下单、模拟盘、持仓、委托、成交功能**。
- 不覆盖港股、美股完整功能（可预留扩展接口）。
- 不实现完整量化回测引擎。
- 不接入任何券商交易通道。

---

## 2. 技术栈确认

见 README / PROJECT_STRUCTURE。参考文档：
- [docs/tencent-quote-api.md](./docs/tencent-quote-api.md)
- [docs/DESIGN.md](./docs/DESIGN.md)
- [docs/PROJECT_STRUCTURE.md](./docs/PROJECT_STRUCTURE.md)

---

## 3. 功能模块拆分（按优先级）

### Phase 0：基础设施 ✅

### Phase 1：数据层（核心）
- [x] 腾讯行情客户端 — **实时快照**（`fetch_quotes` + GBK + `~` 字段解析）
- [ ] 分时数据
- [ ] K线（日/周/月/分钟）
- [ ] 股票搜索
- [ ] 指数与板块（可复用快照批量）
- [x] 数据模型初版（`Symbol` / `Quote` / `Bar` / 五档）
- [ ] 缓存与订阅机制
- [x] 基础错误类型（`QuoteError`）

### Phase 2–6
见历史计划；UI 仍待 gpui-kit。

---

## 4. 本地命令

```bash
cargo test -p longbridgea-core -p tencent-quote -p longbridgea-chart
cargo run -p longbridgea-app
LONGBRIDGEA_LIVE=1 cargo run -p longbridgea-app   # 真实请求茅台快照
```

工具链：`rust-toolchain.toml` → 1.85.0

---

## 5. 风险

- 腾讯接口非官方；字段变更需改 `parse.rs`
- 限流与 GBK 编码
- 严格排除交易功能

---

**最后更新**：2026-11-15（Phase 1：快照客户端）  
**仓库**：https://github.com/vampireelon/LongbridgeA
